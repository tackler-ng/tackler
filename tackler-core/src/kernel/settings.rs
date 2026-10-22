/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::overlaps::{InputOverlap, OverlapConfig, StorageOverlap};
use crate::config::{
    AccountSelectors, Config, Export, ExportType, Kernel, PriceLookupType, Report, ReportType,
    StorageType,
};
use crate::kernel::hash::Hash;
use crate::kernel::price_lookup::PriceLookup;
use crate::kernel::state::State;
use crate::kernel::state::account_tree::{AccountTree, Commodities, Tags};
use crate::model::price_entry::PriceDb;
use crate::model::{Commodity, TxnAccount};
use crate::{config, parser, tackler};
use jiff::Zoned;
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
pub struct Price {
    // todo: fix visibility
    pub price_db: PriceDb,
    pub lookup_type: PriceLookupType,
}

#[derive(Debug, Default)]
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
    state: State,
}

impl Settings {
    #[inline]
    pub(crate) fn get_txn_account(
        &self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        self.state.get_txn_account(name, commodity)
    }
    #[inline]
    pub(crate) fn get_or_create_txn_account(
        &mut self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        self.state.get_or_create_txn_account(name, commodity)
    }
    #[inline]
    pub(crate) fn get_or_create_tag(&mut self, name: &str) -> Result<Arc<Tag>, tackler::Error> {
        self.state.get_or_create_tag(name)
    }
    #[inline]
    pub(crate) fn get_or_create_commodity(
        &mut self,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        self.state.get_or_create_commodity(name)
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
    #[must_use]
    pub fn default_extid() -> Self {
        let mut s = Settings::default();
        s.kernel.extid.unique = true;
        s
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

        let account_trees = AccountTree::from(&cfg.transaction.accounts.items, strict_mode)?;

        let commodities = Commodities::from(&cfg)?;

        let tags = Tags::from(&cfg)?;

        let mut state = State {
            strict_mode,
            time_defaults: cfg.kernel.timestamp.clone(),
            accounts: account_trees,
            commodities,
            tags,
        };
        let cfg_rpt_commodity = cfg
            .report
            .commodity
            .map(|c| state.get_or_create_commodity(Some(c.name.as_str())))
            .transpose()?;

        let report_commodity = match overlaps.report.commodity {
            Some(c) => Some(state.get_or_create_commodity(Some(c.as_str()))?),
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
            state,
        };
        tmp_settings.report.balance_group.group_by = group_by;

        if strict_mode
            && tmp_settings.export.targets.contains(&ExportType::Equity)
            && !tmp_settings
                .state
                .accounts
                .is_defined(tmp_settings.export.equity.equity_account.as_str())
        {
            let msg = "Unknown `equity.equity-account` and `strict` mode is on".to_string();
            return Err(msg.into());
        }

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
    pub(crate) fn is_strict(&self) -> bool {
        self.strict_mode
    }

    pub(crate) fn get_hash(&self) -> Option<Hash> {
        if self.audit_mode {
            Some(self.kernel.audit.hash.clone())
        } else {
            None
        }
    }

    pub(crate) fn is_extid_unique(&self) -> bool {
        self.kernel.extid.unique
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
