/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::raw_items::{
    AccountsPathRaw, AccountsRaw, AuditRaw, BalanceGroupRaw, BalanceRaw, CommoditiesPathRaw,
    CommoditiesRaw, ConfigRaw, EquityRaw, ExportRaw, FsRaw, GitRaw, InputRaw, KernelRaw, PriceRaw,
    RegisterRaw, ReportRaw, ScaleRaw, TagsPathRaw, TagsRaw, TimestampRaw, TimezoneRaw,
    TransactionRaw,
};
use crate::config::{to_export_targets, to_report_formats, to_report_targets};
use crate::kernel::hash::Hash;
use crate::model::Commodity;
use crate::tackler;
use jiff::fmt::strtime::BrokenDownTime;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{cmp, fs};
use tackler_api::txn_ts::{GroupBy, TimestampStyle};
use tackler_rs::get_abs_path;

/// UI/CFG key value for none
pub const NONE_VALUE: &str = "none";

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum StorageType {
    #[default]
    Fs,
    Git,
}
#[rustfmt::skip]
impl StorageType {
    pub const FS:   &'static str = "fs";
    pub const GIT:  &'static str = "git";
}

impl StorageType {
    /// Storage type from string
    ///
    /// # Errors
    /// Returns `Err` in case of invalid type
    pub fn try_from(storage: &str) -> Result<StorageType, tackler::Error> {
        match storage {
            StorageType::FS => Ok(StorageType::Fs),
            StorageType::GIT => Ok(StorageType::Git),
            _ => Err(format!(
                "Unknown storage type: '{storage}'. Valid options are: {}, {}",
                Self::FS,
                Self::GIT,
            )
            .into()),
        }
    }
}

impl Display for StorageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            StorageType::Fs => StorageType::FS,
            StorageType::Git => StorageType::GIT,
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use crate::config::StorageType;

    #[test]
    // test: 195971d7-f16f-4c1c-a761-6764b28fd4db
    fn test_invalid_storage_type() {
        assert!(StorageType::try_from("invalid").is_err());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PriceLookupType {
    #[default]
    None,
    LastPrice,
    TxnTime,
    GivenTime,
}

impl PriceLookupType {
    pub const NONE: &'static str = NONE_VALUE;
    pub const LAST_PRICE: &'static str = "last-price";
    pub const TXN_TIME: &'static str = "txn-time";
    pub const GIVEN_TIME: &'static str = "given-time";
}

impl Display for PriceLookupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str(PriceLookupType::NONE),
            Self::LastPrice => f.write_str(PriceLookupType::LAST_PRICE),
            Self::TxnTime => f.write_str(PriceLookupType::TXN_TIME),
            Self::GivenTime => f.write_str(PriceLookupType::GIVEN_TIME),
        }
    }
}

impl TryFrom<&str> for PriceLookupType {
    type Error = tackler::Error;

    fn try_from(lookup: &str) -> Result<PriceLookupType, tackler::Error> {
        match lookup {
            PriceLookupType::NONE => Ok(PriceLookupType::None),
            PriceLookupType::LAST_PRICE => Ok(PriceLookupType::LastPrice),
            PriceLookupType::TXN_TIME => Ok(PriceLookupType::TxnTime),
            PriceLookupType::GIVEN_TIME => Ok(PriceLookupType::GivenTime),
            _ => Err(format!(
                "Unknown price lookup type: '{lookup}'. Valid options are: {}, {}, {}, {}",
                Self::NONE,
                Self::LAST_PRICE,
                Self::TXN_TIME,
                Self::GIVEN_TIME,
            )
            .into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum ReportType {
    #[default]
    Balance,
    BalanceGroup,
    Register,
}
impl ReportType {
    const BALANCE: &'static str = "balance";
    const BALANCE_GROUP: &'static str = "balance-group";
    const REGISTER: &'static str = "register";
    /// Report type from string
    ///
    /// # Errors
    /// Returns `Err` in case of an invalid type
    pub fn try_from(r: &str) -> Result<Self, tackler::Error> {
        match r {
            Self::BALANCE => Ok(ReportType::Balance),
            Self::BALANCE_GROUP => Ok(ReportType::BalanceGroup),
            Self::REGISTER => Ok(ReportType::Register),
            _ => Err(format!(
                "Unknown report type: '{r}'. Valid options are: {}, {}, {}",
                Self::BALANCE,
                Self::BALANCE_GROUP,
                Self::REGISTER,
            )
            .into()),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ExportType {
    #[default]
    Equity,
    Identity,
}
impl ExportType {
    const EQUITY: &'static str = "equity";
    const IDENTITY: &'static str = "identity";

    /// Export type from string
    ///
    /// # Errors
    /// Returns `Err` in case of invalid type
    pub fn try_from(e: &str) -> Result<Self, tackler::Error> {
        match e {
            Self::EQUITY => Ok(ExportType::Equity),
            Self::IDENTITY => Ok(ExportType::Identity),
            _ => Err(format!(
                "Unknown export type: '{e}'. Valid options are: {}, {}",
                Self::EQUITY,
                Self::IDENTITY,
            )
            .into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum BalanceType {
    #[default]
    Tree,
    Flat,
}
impl BalanceType {
    const TREE: &'static str = "tree";
    const FLAT: &'static str = "flat";

    fn try_from(b: &str) -> Result<Self, tackler::Error> {
        match b {
            Self::TREE => Ok(BalanceType::Tree),
            Self::FLAT => Ok(BalanceType::Flat),
            _ => Err(format!(
                "Unknown balance type: '{b}'. Valid options are: {}, {}",
                Self::TREE,
                Self::FLAT,
            )
            .into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum FormatType {
    #[default]
    Txt,
    Json,
}
impl FormatType {
    pub const TXT: &'static str = "txt";
    pub const JSON: &'static str = "json";
}
impl TryFrom<&str> for FormatType {
    type Error = tackler::Error;

    fn try_from(t: &str) -> Result<Self, tackler::Error> {
        match t {
            Self::TXT => Ok(FormatType::Txt),
            Self::JSON => Ok(FormatType::Json),
            _ => Err(format!(
                "Unknown report output format type: '{t}'. Valid options are: {}, {}",
                Self::TXT,
                Self::JSON
            )
            .into()),
        }
    }
}

enum Timezone {}

pub(crate) type AccountSelectors = Vec<String>;

#[derive(Debug)]
pub struct Config {
    /// Path of config file for this configuration
    path: PathBuf,
    pub(crate) kernel: Kernel,
    pub(crate) price: Price,
    pub(crate) transaction: Transaction,
    pub(crate) report: Report,
    pub(crate) export: Export,
}

impl Config {
    /// Create configuration
    ///
    /// # Errors
    /// Returns `Err` in case there are syntactical or semantic errors with config
    pub fn try_from<P: AsRef<Path>>(cfg_path: P) -> Result<Config, tackler::Error> {
        let cfg_raw: ConfigRaw = toml::from_str(fs::read_to_string(&cfg_path)?.as_str())?;

        Ok(Config {
            path: cfg_path.as_ref().to_path_buf(),
            kernel: Kernel::try_from(&cfg_raw.kernel)?,
            price: cfg_raw.price.map_or(Ok(Price::default()), |raw_price| {
                Price::try_from(&cfg_path, &raw_price)
            })?,
            transaction: Transaction::from(&cfg_path, &cfg_raw.transaction)?,
            report: Report::from(&cfg_raw.report)?,
            export: { Export::from(&cfg_raw.export, &cfg_raw.report)? },
        })
    }
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Kernel {
    pub(crate) strict: bool,
    pub(crate) timestamp: Timestamp,
    pub(crate) audit: Audit,
    pub input: Input,
}
impl Kernel {
    fn try_from(k_raw: &KernelRaw) -> Result<Kernel, tackler::Error> {
        let k = Kernel {
            strict: k_raw.strict,
            timestamp: Timestamp::from(&k_raw.timestamp)?,
            audit: Audit::from(&k_raw.audit)?,
            input: Input::try_from(&k_raw.input)?,
        };
        Ok(k)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Timestamp {
    pub(crate) default_time: jiff::civil::Time,
    pub(crate) timezone: jiff::tz::TimeZone,
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp {
            default_time: jiff::civil::Time::midnight(),
            timezone: jiff::tz::Offset::UTC.to_time_zone(),
        }
    }
}

impl Timestamp {
    #[allow(clippy::cast_possible_wrap)]
    fn from(ts_raw: &TimestampRaw) -> Result<Timestamp, tackler::Error> {
        let ts = Timestamp {
            default_time: {
                let t = ts_raw.default_time;
                jiff::civil::Time::new(
                    t.hour as i8,
                    t.minute as i8,
                    t.second as i8,
                    t.nanosecond as i32,
                )?
            },
            timezone: { Timezone::from(&ts_raw.timezone)? },
        };
        Ok(ts)
    }
}

impl Timezone {
    fn from(tz_raw: &TimezoneRaw) -> Result<jiff::tz::TimeZone, tackler::Error> {
        let tz = match (&tz_raw.name, &tz_raw.offset) {
            (Some(_), Some(_)) => {
                let msg = "timezone: 'name' and 'offset' are both defined".to_string();
                return Err(msg.into());
            }
            (Some(tz_name), None) => jiff::tz::TimeZone::get(tz_name)?,
            (None, Some(offset)) => {
                if let Some(tm) = BrokenDownTime::parse("%:z", offset)?.offset() {
                    tm.to_time_zone()
                } else {
                    let msg = format!("can't parse offset '{offset}' as valid offset");
                    return Err(msg.into());
                }
            }
            (None, None) => jiff::tz::Offset::UTC.to_time_zone(),
        };
        Ok(tz)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Audit {
    pub(crate) hash: Hash,
    pub(crate) mode: bool,
}
impl Audit {
    fn from(a_raw: &AuditRaw) -> Result<Audit, tackler::Error> {
        let a = Audit {
            hash: Hash::from(&a_raw.hash)?,
            mode: a_raw.mode,
        };
        Ok(a)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub storage: StorageType,
    pub fs: Option<FS>,
    pub git: Option<Git>,
}
impl Input {
    fn try_from(input_raw: &InputRaw) -> Result<Input, tackler::Error> {
        // todo: checks
        let i = Input {
            storage: StorageType::try_from(input_raw.storage.as_str())?,
            fs: match &input_raw.fs {
                Some(fs) => Some(FS::try_from(fs)?),
                None => None,
            },
            git: match &input_raw.git {
                Some(git) => Some(Git::try_from(git)?),
                None => None,
            },
        };
        Ok(i)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FS {
    pub path: String,
    pub dir: String,
    pub ext: String,
}
impl FS {
    fn try_from(fs_raw: &FsRaw) -> Result<FS, tackler::Error> {
        let extension = match &fs_raw.ext {
            Some(ext) => {
                if fs_raw.suffix.is_some() {
                    let msg = "FS has both 'suffix' and 'ext' keys defined";
                    return Err(msg.into());
                }
                ext.clone()
            }
            None => {
                if let Some(ext) = &fs_raw.suffix {
                    ext.clone()
                } else {
                    let msg = "FS is missing 'ext' key";
                    return Err(msg.into());
                }
            }
        };

        let ext = extension
            .strip_prefix('.')
            .unwrap_or(extension.as_str())
            .to_string();
        Ok(FS {
            path: fs_raw.path.clone().unwrap_or_default(),
            dir: fs_raw.dir.clone(),
            ext,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Git {
    pub repo: String,
    pub reference: String,
    pub dir: String,
    pub ext: String,
}
impl Git {
    fn try_from(git_raw: &GitRaw) -> Result<Git, tackler::Error> {
        let repo = match &git_raw.repo {
            Some(repo) => {
                if git_raw.repository.is_some() {
                    let msg = "Git has both 'repo' and 'repository' keys defined";
                    return Err(msg.into());
                }
                repo.clone()
            }
            None => {
                if let Some(repo) = &git_raw.repository {
                    repo.clone()
                } else {
                    let msg = "Git is missing 'repo' key";
                    return Err(msg.into());
                }
            }
        };
        let extension = match &git_raw.ext {
            Some(ext) => {
                if git_raw.suffix.is_some() {
                    let msg = "Git has both 'suffix' and 'ext' keys defined";
                    return Err(msg.into());
                }
                ext.clone()
            }
            None => {
                if let Some(ext) = &git_raw.suffix {
                    ext.clone()
                } else {
                    let msg = "Git is missing 'ext' key";
                    return Err(msg.into());
                }
            }
        };
        let ext = extension
            .strip_prefix('.')
            .unwrap_or(extension.as_str())
            .to_string();
        Ok(Git {
            repo,
            reference: git_raw.git_ref.clone(),
            dir: git_raw.dir.clone(),
            ext,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Price {
    pub(crate) db_path: PathBuf,
    pub(crate) lookup_type: PriceLookupType,
}
impl Price {
    fn try_from<P: AsRef<Path>>(
        base_path: P,
        price_raw: &PriceRaw,
    ) -> Result<Price, tackler::Error> {
        let db_path_str = price_raw.db_path.as_str();
        let lookup_type = PriceLookupType::try_from(price_raw.lookup_type.as_str())?;

        match db_path_str {
            NONE_VALUE => {
                if lookup_type == PriceLookupType::None {
                    Ok(Price::default())
                } else {
                    let msg = "Price database path is 'none' but lookup type is not 'none'";
                    Err(msg.into())
                }
            }
            _ => Ok(Price {
                db_path: get_abs_path(base_path, db_path_str)?,
                lookup_type,
            }),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Transaction {
    pub(crate) accounts: Accounts,
    pub(crate) commodities: Commodities,
    pub(crate) tags: Tags,
}

impl Transaction {
    fn from<P: AsRef<Path>>(
        path: P,
        txn_raw: &TransactionRaw,
    ) -> Result<Transaction, tackler::Error> {
        Ok(Transaction {
            accounts: Accounts::from(&path, &txn_raw.accounts)?,
            commodities: Commodities::from(&path, &txn_raw.commodities)?,
            tags: Tags::from(&path, &txn_raw.tags)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Accounts {
    pub names: Vec<String>,
}
impl Accounts {
    fn from<P: AsRef<Path>>(
        path: P,
        accs_path_raw: &AccountsPathRaw,
    ) -> Result<Accounts, tackler::Error> {
        let accs_path_str = accs_path_raw.path.as_str();
        if accs_path_str == NONE_VALUE {
            Ok(Accounts::default())
        } else {
            let accs_path = get_abs_path(&path, accs_path_str)?;
            let acc_raw: AccountsRaw = match fs::read_to_string(&accs_path) {
                Ok(s) => toml::from_str(s.as_str())?,
                Err(err) => {
                    let msg = format!(
                        "Accounts configuration error while reading file '{accs_path_str}': {err}"
                    );
                    return Err(msg.into());
                }
            };
            Ok(Accounts {
                names: acc_raw.names,
            })
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Commodities {
    pub(crate) permit_empty_commodity: Option<bool>,

    pub(crate) names: Vec<String>,
}
impl Commodities {
    fn from<P: AsRef<Path>>(
        path: P,
        comm_path_raw: &CommoditiesPathRaw,
    ) -> Result<Commodities, tackler::Error> {
        let comm_path_str = comm_path_raw.path.as_str();
        if comm_path_str == NONE_VALUE {
            Ok(Commodities {
                permit_empty_commodity: Some(true),
                names: Vec::new(),
            })
        } else {
            let comm_path = get_abs_path(&path, comm_path_str)?;
            let comm_raw: CommoditiesRaw = match fs::read_to_string(&comm_path) {
                Ok(s) => toml::from_str(s.as_str())?,
                Err(err) => {
                    let msg = format!(
                        "Commodities configuration error while reading file '{comm_path_str}': {err}"
                    );
                    return Err(msg.into());
                }
            };
            Ok(Commodities {
                permit_empty_commodity: comm_raw.permit_empty_commodity,
                names: comm_raw.names,
            })
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Tags {
    pub(crate) names: Vec<String>,
}

impl Tags {
    fn from<P: AsRef<Path>>(path: P, tags_path_raw: &TagsPathRaw) -> Result<Tags, tackler::Error> {
        let tags_path_str = tags_path_raw.path.as_str();
        if tags_path_str == NONE_VALUE {
            Ok(Tags::default())
        } else {
            let tags_path = get_abs_path(&path, tags_path_str)?;
            let tags_raw: TagsRaw = match fs::read_to_string(&tags_path) {
                Ok(s) => toml::from_str(s.as_str())?,
                Err(err) => {
                    let msg = format!(
                        "Tags configuration error while reading file '{tags_path_str}': {err}"
                    );
                    return Err(msg.into());
                }
            };
            Ok(Tags {
                names: tags_raw.names,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Report {
    pub tz: TimeZone,
    pub targets: Vec<ReportType>,
    pub formats: Vec<FormatType>,
    pub scale: Scale,
    pub commodity: Option<Arc<Commodity>>,
    pub register: Register,
    pub balance_group: BalanceGroup,
    pub balance: Balance,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            tz: TimeZone::UTC,
            targets: Vec::new(),
            formats: Vec::new(),
            scale: Scale::default(),
            commodity: None,
            register: Register::default(),
            balance_group: BalanceGroup::default(),
            balance: Balance::default(),
        }
    }
}

impl Report {
    fn from(report_raw: &ReportRaw) -> Result<Report, tackler::Error> {
        let targets = to_report_targets(&report_raw.targets)?;
        let formats = to_report_formats(report_raw.formats.as_deref())?;

        Ok(Report {
            tz: TimeZone::get(report_raw.report_tz.as_str())?,
            targets,
            formats,
            scale: Scale::from(&report_raw.scale)?,
            commodity: match &report_raw.commodity {
                Some(c) => Some(Arc::new(Commodity::from(c.clone())?)),
                None => None,
            },
            register: Register::from(&report_raw.register, report_raw)?,
            balance_group: BalanceGroup::from(&report_raw.balance_group, report_raw)?,
            balance: Balance::from(&report_raw.balance, report_raw)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Scale {
    min: u32,
    max: u32,
}
impl Scale {
    pub fn try_from(min: u32, max: u32) -> Result<Scale, tackler::Error> {
        Self::check_range(min, max)?;
        Ok(Scale { min, max })
    }

    fn check_range(min: u32, max: u32) -> Result<(), tackler::Error> {
        let max_scale = 28;
        if min > max_scale || max > max_scale {
            let msg = format!(
                "scale error: too large value - maximum scale value for min or max is {max_scale}"
            );
            return Err(msg.into());
        }
        if max < min {
            let msg = "scale error: 'min' can't be greater than 'max'";
            return Err(msg.into());
        }
        Ok(())
    }
    fn from(scale_raw: &ScaleRaw) -> Result<Scale, tackler::Error> {
        Self::check_range(scale_raw.min, scale_raw.max)?;
        Ok(Scale {
            min: scale_raw.min,
            max: scale_raw.max,
        })
    }
    pub fn get_precision(&self, d: &Decimal) -> usize {
        cmp::max(cmp::min(d.scale(), self.max), self.min) as usize
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale { min: 2, max: 7 }
    }
}

fn get_account_selector(
    acc_sel: Option<&AccountSelectors>,
    report: &ReportRaw,
) -> AccountSelectors {
    match acc_sel {
        Some(av) => av.clone(),
        None => match &report.accounts {
            Some(av) => av.clone(),
            None => vec![],
        },
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Register {
    pub title: String,
    pub timestamp_style: TimestampStyle,
    pub acc_sel: AccountSelectors,
}

impl Register {
    fn from(reg_raw: &RegisterRaw, report: &ReportRaw) -> Result<Register, tackler::Error> {
        Ok(Register {
            title: reg_raw.title.clone(),
            timestamp_style: match &reg_raw.timestamp_style {
                Some(style) => TimestampStyle::from(style.as_str())?,
                None => TimestampStyle::Date,
            },
            acc_sel: get_account_selector(reg_raw.acc_sel.as_ref(), report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BalanceGroup {
    pub title: String,
    pub bal_type: BalanceType,
    pub group_by: GroupBy,
    pub acc_sel: AccountSelectors,
}

impl BalanceGroup {
    fn from(
        balgrp_raw: &BalanceGroupRaw,
        report: &ReportRaw,
    ) -> Result<BalanceGroup, tackler::Error> {
        Ok(BalanceGroup {
            title: balgrp_raw.title.clone(),
            bal_type: match &balgrp_raw.bal_type {
                Some(t) => BalanceType::try_from(t.as_str())?,
                None => BalanceType::default(),
            },
            group_by: GroupBy::from(balgrp_raw.group_by.as_str())?,
            acc_sel: get_account_selector(balgrp_raw.acc_sel.as_ref(), report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Balance {
    pub title: String,
    pub bal_type: BalanceType,
    pub acc_sel: AccountSelectors,
}

impl Balance {
    fn from(bal_raw: &BalanceRaw, report: &ReportRaw) -> Result<Balance, tackler::Error> {
        Ok(Balance {
            title: bal_raw.title.clone(),
            bal_type: match &bal_raw.bal_type {
                Some(t) => BalanceType::try_from(t.as_str())?,
                None => BalanceType::default(),
            },
            acc_sel: get_account_selector(bal_raw.acc_sel.as_ref(), report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Export {
    pub targets: Vec<ExportType>,
    pub equity: Equity,
}
impl Export {
    fn from(export_raw: &ExportRaw, report: &ReportRaw) -> Result<Export, tackler::Error> {
        let trgs = to_export_targets(&export_raw.targets)?;
        Ok(Export {
            targets: trgs,
            equity: Equity::from(&export_raw.equity, report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Equity {
    pub(crate) equity_account: String,
    pub(crate) acc_sel: AccountSelectors,
}

impl Equity {
    fn from(eq_raw: &EquityRaw, report: &ReportRaw) -> Equity {
        Equity {
            equity_account: eq_raw.equity_account.clone(),
            acc_sel: get_account_selector(eq_raw.acc_sel.as_ref(), report),
        }
    }
}
