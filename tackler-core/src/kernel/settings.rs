/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::overlaps::{InputOverlap, OverlapConfig, StorageOverlap};
use crate::config::{
    AccountSelectors, Config, Export, ExportType, Kernel, PriceLookupType, Report, ReportType,
    StorageType,
};
use crate::kernel::hash::Hash;
use crate::kernel::price_lookup::PriceLookup;
use crate::model::TxnAccount;
use crate::model::price_entry::PriceDb;
use crate::model::{AccountTreeNode, Commodity};
use crate::{config, parser, tackler};
use jiff::Zoned;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tackler_api::txn_header::Tag;
use tackler_api::txn_ts::GroupBy;
use tackler_rs::normalize_extension;

#[derive(Debug, Default, Clone)]
pub struct FileInput {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FsInput {
    pub path: PathBuf,
    pub dir: PathBuf,
    pub ext: String,
}

#[derive(Debug, Clone)]
pub enum GitInputSelector {
    CommitId(String),
    Reference(String),
}

#[derive(Debug, Clone)]
pub struct GitInput {
    pub repo: PathBuf,
    pub dir: String,
    pub git_ref: GitInputSelector,
    pub ext: String,
}

#[derive(Debug, Clone)]
pub enum InputSettings {
    File(FileInput),
    Fs(FsInput),
    Git(GitInput),
}
impl Default for InputSettings {
    fn default() -> Self {
        Self::File(FileInput::default())
    }
}

#[derive(Debug, Default)]
struct Commodities {
    names: HashMap<String, Arc<Commodity>>,
    permit_empty_commodity: bool,
}

impl Commodities {
    fn default_empty_ok() -> Self {
        Commodities {
            names: HashMap::new(),
            permit_empty_commodity: true,
        }
    }

    fn from(cfg: &Config) -> Result<Commodities, tackler::Error> {
        let cfg_comm = &cfg.transaction.commodities;
        let permit_empty_commodity = cfg_comm.permit_empty_commodity.unwrap_or(false);

        let comms =
            cfg_comm.names.iter().try_fold(
                HashMap::new(),
                |mut chm, comm| match Commodity::from(comm.clone()) {
                    Ok(c) => {
                        chm.insert(comm.into(), Arc::new(c));
                        Ok(chm)
                    }
                    Err(e) => {
                        let msg = format!("Invalid Chart of Commodities: {e}");
                        Err(msg)
                    }
                },
            )?;
        Ok(Commodities {
            names: comms,
            permit_empty_commodity,
        })
    }
}

#[derive(Debug, Default)]
struct AccountTrees {
    defined_accounts: HashMap<String, Arc<AccountTreeNode>>,
    synthetic_parents: HashMap<String, Arc<AccountTreeNode>>,
}

impl AccountTrees {
    fn build_account_tree(
        target_account_tree: &mut HashMap<String, Arc<AccountTreeNode>>,
        atn: &Arc<AccountTreeNode>,
        other_account_tree: Option<&HashMap<String, Arc<AccountTreeNode>>>,
    ) -> Result<(), tackler::Error> {
        let parent = atn.parent.as_str();
        let has_parent = other_account_tree.is_some_and(|a| a.contains_key(parent))
            || target_account_tree.contains_key(parent);

        if has_parent || atn.is_root() {
            // this breaks recursion
            Ok(())
        } else {
            let parent_atn =
                Arc::new(AccountTreeNode::from(parent).expect("IE: synthetic parent is invalid"));
            target_account_tree.insert(parent.to_string(), parent_atn.clone());

            Self::build_account_tree(target_account_tree, &parent_atn, other_account_tree)
        }
    }

    fn from(account_names: &[String], strict_mode: bool) -> Result<AccountTrees, tackler::Error> {
        let defined_accounts =
            account_names
                .iter()
                .try_fold(
                    HashMap::new(),
                    |mut accs, account| match AccountTreeNode::from(account) {
                        Ok(atn) => {
                            accs.insert(account.into(), Arc::new(atn));
                            Ok(accs)
                        }
                        Err(e) => {
                            let msg = format!("Invalid Chart of Accounts: {e}");
                            Err(msg)
                        }
                    },
                )?;

        let synthetic_parents = if strict_mode {
            // Synthetic Account Parents are only needed in strict mode
            let mut sap = HashMap::new();
            for atn_entry in &defined_accounts {
                if !&defined_accounts.contains_key(atn_entry.1.parent.as_str()) {
                    // Parent is missing -> Let's build synthetic tree
                    let (_, atn) = atn_entry;
                    Self::build_account_tree(&mut sap, atn, Some(&defined_accounts))?;
                }
            }
            sap
        } else {
            HashMap::new()
        };
        Ok(AccountTrees {
            defined_accounts,
            synthetic_parents,
        })
    }
}

#[derive(Debug, Default)]
pub struct Price {
    // todo: fix visibility
    pub price_db: PriceDb,
    pub lookup_type: PriceLookupType,
}

#[derive(Debug)]
pub struct Settings {
    pub(crate) audit_mode: bool,
    pub(crate) report: Report,
    pub(crate) export: Export,
    strict_mode: bool,
    pub(crate) inverted: bool,
    input_config: InputSettings,
    kernel: Kernel,
    pub price: Price,
    price_lookup: PriceLookup,
    global_acc_sel: Option<AccountSelectors>,
    accounts: AccountTrees,
    commodities: Commodities,
    tags: HashMap<String, Arc<Tag>>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            strict_mode: false,
            audit_mode: false,
            inverted: false,
            input_config: InputSettings::default(),
            report: Report::default(),
            export: Export::default(),
            kernel: Kernel::default(),
            price: Price::default(),
            price_lookup: PriceLookup::default(),
            global_acc_sel: None,
            accounts: AccountTrees::default(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    #[must_use]
    pub fn default_audit() -> Self {
        Settings {
            audit_mode: true,
            ..Self::default()
        }
    }
}

impl Settings {
    /// # Errors
    /// Return `Err` in case of semantically incorrect configuration
    #[allow(clippy::too_many_lines)]
    pub fn try_from(cfg: Config, overlaps: OverlapConfig) -> Result<Settings, tackler::Error> {
        fn check_given_time_usage(
            gt: Option<&String>,
            plt: PriceLookupType,
        ) -> Result<(), tackler::Error> {
            if gt.is_some() {
                let msg = format!(
                    "Price \"before timestamp\" is not allowed when price lookup type is \"{plt}\""
                );
                return Err(msg.into());
            }
            Ok(())
        }

        let strict_mode = overlaps.strict.mode.unwrap_or(cfg.kernel.strict);
        let audit_mode = overlaps.audit.mode.unwrap_or(cfg.kernel.audit.mode);

        let input_settings = Self::input_settings(&cfg, &overlaps.storage)?;

        let reports = match overlaps.target.reports {
            Some(reports) => config::to_report_targets(&reports)?,
            None => cfg.report.targets.clone(),
        };
        let exports = match overlaps.target.exports {
            Some(exports) => config::to_export_targets(&exports)?,
            None => cfg.export.targets.clone(),
        };

        let formats = match overlaps.target.formats {
            Some(formats) => config::to_report_formats(Some(&formats))?,
            None => cfg.report.formats.clone(),
        };

        let lookup_type = overlaps.price.lookup_type.unwrap_or(cfg.price.lookup_type);

        let db_path = overlaps.price.db_path.unwrap_or(cfg.price.db_path.clone());

        let account_trees = AccountTrees::from(&cfg.transaction.accounts.names, strict_mode)?;

        let mut commodities = Commodities::from(&cfg)?;

        let tags = cfg
            .transaction
            .tags
            .names
            .iter()
            .fold(HashMap::new(), |mut tags, tag| {
                let t = Tag::from(tag.clone());
                tags.insert(tag.into(), Arc::new(t));
                tags
            });

        if strict_mode
            && exports.contains(&ExportType::Equity)
            && !account_trees
                .defined_accounts
                .contains_key(cfg.export.equity.equity_account.as_str())
        {
            let msg = "Unknown `equity.equity-account` and `strict` mode is on".to_string();
            return Err(msg.into());
        }

        let cfg_rpt_commodity = cfg
            .report
            .commodity
            .map(|c| {
                Self::inner_get_or_create_commodity(
                    &mut commodities,
                    strict_mode,
                    Some(c.name.as_str()),
                )
            })
            .transpose()?;

        let report_commodity = match overlaps.report.commodity {
            Some(c) => Some(Self::inner_get_or_create_commodity(
                &mut commodities,
                strict_mode,
                Some(c.as_str()),
            )?),
            None => cfg_rpt_commodity,
        };

        if report_commodity.is_none() && lookup_type != PriceLookupType::None {
            let msg =
                "Price conversion is activated, but there is no `report.commodity`".to_string();
            return Err(msg.into());
        }

        let group_by = overlaps
            .report
            .group_by
            .map(|g| GroupBy::from(g.as_str()))
            .unwrap_or(Ok(cfg.report.balance_group.group_by))?;

        let mut tmp_settings = Settings {
            strict_mode,
            audit_mode,
            inverted: overlaps.report.inverted,
            kernel: cfg.kernel,
            input_config: input_settings,
            price: Price::default(), // this is not real, see next one
            price_lookup: PriceLookup::default(), // this is not real, see next one
            global_acc_sel: overlaps.report.account_overlap,
            report: Report {
                commodity: report_commodity,
                targets: reports,
                formats,
                ..cfg.report
            },
            export: Export {
                targets: exports,
                ..cfg.export
            },
            accounts: account_trees,
            commodities,
            tags,
        };
        tmp_settings.report.balance_group.group_by = group_by;

        let given_time = overlaps.price.before_time;

        let price_lookup = match lookup_type {
            plt @ PriceLookupType::LastPrice => {
                check_given_time_usage(given_time.as_ref(), plt)?;
                PriceLookup::LastPriceDbEntry
            }
            plt @ PriceLookupType::TxnTime => {
                check_given_time_usage(given_time.as_ref(), plt)?;
                PriceLookup::AtTheTimeOfTxn
            }
            plt @ PriceLookupType::GivenTime => {
                if let Some(ts) = given_time {
                    tmp_settings
                        .parse_timestamp(ts.as_str())
                        .map(PriceLookup::GivenTime)?
                } else {
                    let msg =
                        format!("Price lookup type is \"{plt}\" and there is no timestamp given");
                    return Err(msg.into());
                }
            }
            plt @ PriceLookupType::None => {
                check_given_time_usage(given_time.as_ref(), plt)?;
                PriceLookup::None
            }
        };

        let price = match &lookup_type {
            PriceLookupType::None => Price::default(),
            _ => Price {
                // we need half-baked settings here bc commodity and timestamp lookups
                price_db: parser::pricedb_from_file(&db_path, &mut tmp_settings)?,
                lookup_type,
            },
        };

        Ok(Settings {
            price,
            price_lookup,
            ..tmp_settings
        })
    }
}
impl Settings {
    pub(crate) fn get_hash(&self) -> Option<Hash> {
        if self.audit_mode {
            Some(self.kernel.audit.hash.clone())
        } else {
            None
        }
    }

    pub(crate) fn get_txn_account(
        &self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let comm = self.get_commodity(commodity.name.as_str())?;

        match self.accounts.defined_accounts.get(name) {
            Some(account_tree) => Ok(TxnAccount {
                atn: account_tree.clone(),
                comm,
            }),
            None => {
                if let Some(acc_parent) = self.accounts.synthetic_parents.get(name) {
                    Ok(TxnAccount {
                        atn: acc_parent.clone(),
                        comm,
                    })
                } else {
                    let msg = format!("Unknown account: '{name}'");
                    Err(msg.into())
                }
            }
        }
    }

    pub(crate) fn get_or_create_txn_account(
        &mut self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let comm = self.get_or_create_commodity(Some(commodity.name.as_str()))?;

        let strict_mode = self.strict_mode;
        let atn_opt = self.accounts.defined_accounts.get(name).cloned();

        let atn = if let Some(account_tree) = atn_opt {
            TxnAccount {
                atn: account_tree.clone(),
                comm,
            }
        } else {
            if self.strict_mode {
                let msg = format!("Unknown account: '{name}'");
                return Err(msg.into());
            }
            let atn = Arc::new(AccountTreeNode::from(name)?);
            self.accounts
                .defined_accounts
                .insert(name.into(), atn.clone());
            AccountTrees::build_account_tree(&mut self.accounts.defined_accounts, &atn, None)?;

            TxnAccount { atn, comm }
        };
        if !strict_mode {
            // Not strict mode, so we build the (missing) parents
            // directly into main Chart of Accounts
            AccountTrees::build_account_tree(&mut self.accounts.defined_accounts, &atn.atn, None)?;
        }

        Ok(atn)
    }

    /// # Errors
    /// Returns reference for commodity, error if it doesn't exist
    pub fn get_commodity(&self, name: &str) -> Result<Arc<Commodity>, tackler::Error> {
        if let Some(comm) = self.commodities.names.get(name) {
            Ok(comm.clone())
        } else {
            let msg = format!("Unknown commodity: '{name}'");
            Err(msg.into())
        }
    }
    pub(crate) fn get_or_create_commodity(
        &mut self,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        Self::inner_get_or_create_commodity(&mut self.commodities, self.strict_mode, name)
    }

    fn inner_get_or_create_commodity(
        commodities: &mut Commodities,
        strict_mode: bool,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        if let Some(n) = name {
            if n.is_empty() {
                let res = if commodities.permit_empty_commodity {
                    if let Some(c) = commodities.names.get(n) {
                        Ok(c.clone())
                    } else {
                        let comm = Arc::new(Commodity::default());
                        commodities.names.insert(n.into(), comm.clone());

                        Ok(comm)
                    }
                } else {
                    let msg = "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                    Err(msg.into())
                };
                return res;
            }
            match commodities.names.get(n) {
                Some(comm) => Ok(comm.clone()),
                None => {
                    if strict_mode {
                        let msg = format!("Unknown commodity: '{n}'");
                        Err(msg.into())
                    } else {
                        let comm = Arc::new(Commodity::from(n.into())?);
                        commodities.names.insert(n.into(), comm.clone());
                        Ok(comm)
                    }
                }
            }
        } else {
            let comm = Arc::new(Commodity::default());
            Ok(comm)
        }
    }

    pub(crate) fn get_or_create_tag(&mut self, name: &str) -> Result<Arc<Tag>, tackler::Error> {
        if name.is_empty() {
            let msg = "Tag name is empty string".to_string();
            return Err(msg.into());
        }
        match self.tags.get(name) {
            Some(tag) => Ok(tag.clone()),
            None => {
                if self.strict_mode {
                    let msg = format!("Unknown tag: '{name}'");
                    Err(msg.into())
                } else {
                    let tag = Arc::new(Tag::from(name));
                    self.tags.insert(name.into(), tag.clone());
                    Ok(tag)
                }
            }
        }
    }

    #[must_use]
    pub fn get_price_lookup(&self) -> PriceLookup {
        self.price_lookup.clone()
    }

    #[must_use]
    pub fn input(&self) -> InputSettings {
        self.input_config.clone()
    }

    #[allow(clippy::too_many_lines)]
    fn input_settings(
        cfg: &Config,
        storage_overlap: &StorageOverlap,
    ) -> Result<InputSettings, tackler::Error> {
        let cfg_input = &cfg.kernel.input;

        // if input_overlap.git.repo => storage git
        // if input_overlap.fs.path => storage fs

        let storage_target = match storage_overlap.storage_type {
            Some(storage) => storage,
            None => cfg_input.storage,
        };

        let storage_type = if let Some(io) = &storage_overlap.input {
            match io {
                InputOverlap::File(f) => {
                    // This is file based input => no need to configure storage
                    return Ok(InputSettings::File(FileInput {
                        path: f.path.clone(),
                    }));
                }
                InputOverlap::Fs(fs) => {
                    if fs.path.is_none() && storage_target != StorageType::Fs {
                        let msg = "Conflicting input and storage system arguments. Targeting 'fs', but it's not activated by configuration or by cli options";
                        return Err(msg.into());
                    }
                    StorageType::Fs
                }
                InputOverlap::Git(git) => {
                    if git.repo.is_none() && storage_target != StorageType::Git {
                        let msg = "Conflicting input and storage system arguments. Targeting 'git', but it's not activated by configuration or by cli options";
                        return Err(msg.into());
                    }
                    StorageType::Git
                }
            }
        } else {
            storage_target
        };

        match storage_type {
            StorageType::Fs => match &cfg_input.fs {
                Some(fs_cfg) => {
                    let i = if let Some(InputOverlap::Fs(fs_soi)) = &storage_overlap.input {
                        // FS: Overlap + Config
                        let path = fs_soi.path.as_ref().unwrap_or(&fs_cfg.path);
                        let dir = fs_soi.dir.as_ref().unwrap_or(&fs_cfg.dir);
                        let ext = fs_soi.ext.as_ref().unwrap_or(&fs_cfg.ext);
                        FsInput {
                            path: tackler_rs::get_abs_path(cfg.path(), path)?,
                            dir: PathBuf::from(dir),
                            ext: normalize_extension(ext).to_string(),
                        }
                    } else {
                        // FS: No overlap, all info must come from config
                        let ext = &fs_cfg.ext;
                        FsInput {
                            path: tackler_rs::get_abs_path(cfg.path(), fs_cfg.path.as_str())?,
                            dir: PathBuf::from(&fs_cfg.dir),
                            ext: normalize_extension(ext).to_string(),
                        }
                    };
                    Ok(InputSettings::Fs(i))
                }
                None => {
                    // FS: No config, all info must come from overlap
                    if let Some(InputOverlap::Fs(fs_soi)) = &storage_overlap.input {
                        if let (Some(path), Some(dir), Some(ext)) =
                            (&fs_soi.path, &fs_soi.dir, &fs_soi.ext)
                        {
                            Ok(InputSettings::Fs(FsInput {
                                path: tackler_rs::get_abs_path(cfg.path(), path)?,
                                dir: PathBuf::from(dir),
                                ext: normalize_extension(ext).to_string(),
                            }))
                        } else {
                            let msg = format!(
                                "Not enough information to configure 'fs' storage: path = '{:?}', dir = '{:?}', ext = '{:?}'",
                                fs_soi.path, fs_soi.dir, fs_soi.ext
                            );
                            Err(msg.into())
                        }
                    } else {
                        Err("Storage type 'fs' is not configured".into())
                    }
                }
            },
            StorageType::Git => match &cfg_input.git {
                Some(git_cfg) => {
                    let i = if let Some(InputOverlap::Git(git_soi)) = &storage_overlap.input {
                        // GIT: Overlap + Config
                        let repo = git_soi.repo.as_ref().unwrap_or(&git_cfg.repo);
                        let dir = git_soi.dir.as_ref().unwrap_or(&git_cfg.dir);
                        let ext = git_soi.ext.as_ref().unwrap_or(&git_cfg.ext);
                        // reference is only option via cfg
                        let cfg_ref = GitInputSelector::Reference(git_cfg.reference.clone());
                        let git_ref = git_soi.git_ref.as_ref().unwrap_or(&cfg_ref);

                        GitInput {
                            repo: tackler_rs::get_abs_path(cfg.path(), repo)?,
                            git_ref: git_ref.clone(),
                            dir: dir.clone(),
                            ext: normalize_extension(ext).to_string(),
                        }
                    } else {
                        // GIT: No overlap, all info must come from config
                        let repo = git_cfg.repo.as_str();
                        let ext = &git_cfg.ext;
                        GitInput {
                            repo: tackler_rs::get_abs_path(cfg.path(), repo)?,
                            git_ref: GitInputSelector::Reference(git_cfg.reference.clone()),
                            dir: git_cfg.dir.clone(),
                            ext: normalize_extension(ext).to_string(),
                        }
                    };
                    Ok(InputSettings::Git(i))
                }
                None => {
                    // GIT: No config, all info must come from overlap
                    if let Some(InputOverlap::Git(git_soi)) = &storage_overlap.input {
                        if let (Some(repo), Some(dir), Some(ext), Some(git_ref)) =
                            (&git_soi.repo, &git_soi.dir, &git_soi.ext, &git_soi.git_ref)
                        {
                            Ok(InputSettings::Git(GitInput {
                                repo: tackler_rs::get_abs_path(cfg.path(), repo)?,
                                git_ref: git_ref.clone(),
                                dir: dir.clone(),
                                ext: normalize_extension(ext).to_string(),
                            }))
                        } else {
                            let msg = format!(
                                "Not enough information to configure 'git' storage: repo = '{:?}', dir = '{:?}', ext = '{:?}', ref = '{:?}'",
                                git_soi.repo, git_soi.dir, git_soi.ext, git_soi.git_ref
                            );
                            Err(msg.into())
                        }
                    } else {
                        Err("Storage type 'git' is not configured".into())
                    }
                }
            },
        }
    }
}

impl Settings {
    /// Parse timestamp in tackler-accepted format:
    ///
    /// Date (YYYY-MM-DD)
    /// Date-Time (YYYY-MM-DDTHH:MM:SS[.SSS])
    /// Date-Time-Zulu (YYYY-MM-DDTHH:MM:SS[.SSS]Z)
    /// Date-Time-Offset (YYYY-MM-DDTHH:MM:SS[.SSS]+-HH:MM)
    ///
    /// Fractional seconds are supported up to nanosecond
    ///
    /// # Errors
    /// Return `Err` timestamp is invalid
    pub fn parse_timestamp(&mut self, ts: &str) -> Result<Zoned, tackler::Error> {
        Ok(winnow::Parser::parse(
            &mut crate::parser::parts::timestamp::parse_timestamp,
            winnow::Stateful {
                input: ts,
                state: self,
            },
        )
        .map_err(|e| e.to_string())?)
    }

    /// # Errors
    /// Return `Err` if conversion to zone is not possible
    pub fn get_offset_datetime(&self, dt: jiff::civil::DateTime) -> Result<Zoned, tackler::Error> {
        match dt.to_zoned(self.kernel.timestamp.timezone.clone()) {
            Ok(ts) => Ok(ts),
            Err(err) => {
                let msg = format!("time is invalid '{err:?}'");
                Err(msg.into())
            }
        }
    }
    /// # Errors
    /// Return `Err` if conversion to timestamp is not possible
    pub fn get_offset_date(&self, date: jiff::civil::Date) -> Result<Zoned, tackler::Error> {
        let ts = date.to_datetime(self.kernel.timestamp.default_time);
        match ts.to_zoned(self.kernel.timestamp.timezone.clone()) {
            Ok(ts) => Ok(ts),
            Err(err) => {
                let msg = format!("time is invalid '{err:?}'");
                Err(msg.into())
            }
        }
    }

    #[must_use]
    pub fn get_report_commodity(&self) -> Option<Arc<Commodity>> {
        self.report.commodity.clone()
    }

    #[must_use]
    pub fn get_report_targets(&self) -> Vec<ReportType> {
        self.report.targets.clone()
    }

    #[must_use]
    pub fn get_export_targets(&self) -> Vec<ExportType> {
        self.export.targets.clone()
    }

    #[must_use]
    fn get_account_selector(&self, acc_sel: &AccountSelectors) -> AccountSelectors {
        let v = match &self.global_acc_sel {
            Some(global_acc_sel) => global_acc_sel.clone(),
            None => acc_sel.clone(),
        };

        // Turn "" into an empty ("select all") account selector
        if v.len() == 1 && v[0].is_empty() {
            Vec::new()
        } else {
            v
        }
    }

    #[must_use]
    pub fn get_balance_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.balance.acc_sel)
    }

    #[must_use]
    pub fn get_balance_group_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.balance_group.acc_sel)
    }

    #[must_use]
    pub fn get_register_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.register.acc_sel)
    }

    #[must_use]
    pub fn get_equity_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.export.equity.acc_sel)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn accounts_strict_false() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account, "a:b:c");
        assert_eq!(txntn_1.atn.get_name(), "c");

        let txntn_2 = settings.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account, "a:b:c");
        assert_eq!(txntn_2.atn.get_name(), "c");

        let txntn_3 = settings.get_or_create_txn_account("a:b:b-leaf", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 4);

        assert_eq!(txntn_3.atn.depth, 3);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a:b");
        assert_eq!(txntn_3.atn.account, "a:b:b-leaf");
        assert_eq!(txntn_3.atn.get_name(), "b-leaf");
    }

    #[test]
    fn accounts_strict_true() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account, "a:b:c");
        assert_eq!(txntn_1.atn.get_name(), "c");

        let txntn_2 = settings.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account, "a:b:c");
        assert_eq!(txntn_2.atn.get_name(), "c");

        // Check that it won't create a synthetic account as real one
        assert!(settings.get_or_create_txn_account("a:b", &comm).is_err());
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        // Check synthetic account
        let txntn_3 = settings.get_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_3.atn.depth, 2);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a");
        assert_eq!(txntn_3.atn.account, "a:b");
        assert_eq!(txntn_3.atn.get_name(), "b");

        // Check synthetic account
        let txntn_4 = settings.get_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_4.atn.depth, 1);
        assert_eq!(txntn_4.atn.get_root(), "a");
        assert_eq!(txntn_4.atn.parent, "");
        assert_eq!(txntn_4.atn.account, "a");
        assert_eq!(txntn_4.atn.get_name(), "a");
    }

    #[test]
    fn accounts_strict_true_child_first() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c".to_string(), "a:b".to_string(), "a".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_1.atn.account, "a:b:c");

        let txntn_2 = settings.get_or_create_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account, "a:b");

        let txntn_2 = settings.get_or_create_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account, "a");
    }

    #[test]
    fn accounts_strict_true_gap() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c:d".to_string(), "a:b".to_string(), "a".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);

        // Check that it won't create a synthetic account as real one
        assert!(settings.get_or_create_txn_account("a:b:c", &comm).is_err());

        let txntn_synth = settings.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_synth.atn.account, "a:b:c");

        let txntn_2 = settings.get_or_create_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account, "a:b");

        let txntn_2 = settings.get_or_create_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account, "a");
    }
}
