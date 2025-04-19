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
use crate::config::{to_export_targets, to_report_targets};
use crate::kernel::hash::Hash;
use crate::model::Commodity;
use crate::tackler;
use jiff::fmt::strtime::BrokenDownTime;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{cmp, fs};
use tackler_api::txn_ts::{GroupBy, TimestampStyle};
use tackler_rs::get_abs_path;

/// UI/CFG key value for none
pub const NONE_VALUE: &str = "none";

#[derive(Debug, Copy, Clone, Default)]
pub enum StorageType {
    #[default]
    FS,
    Git,
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
            _ => Err(format!("Unknown price lookup type: {}", lookup).into()),
        }
    }
}

#[rustfmt::skip]
impl StorageType {
    pub const STORAGE_FS:   &'static str = "fs";
    pub const STORAGE_GIT:  &'static str = "git";

    pub(crate) fn from(storage: &str) -> Result<StorageType, tackler::Error> {
        match storage {
            StorageType::STORAGE_FS => Ok(StorageType::FS),
            StorageType::STORAGE_GIT => Ok(StorageType::Git),
            _ => Err(format!("Unknown storage type: {}", storage).into()),
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
    pub fn from(r: &str) -> Result<Self, tackler::Error> {
        match r {
            "balance" => Ok(ReportType::Balance),
            "balance-group" => Ok(ReportType::BalanceGroup),
            "register" => Ok(ReportType::Register),
            _ => Err(format!("Unknown report type {r}").into()),
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
    pub fn from(r: &str) -> Result<Self, tackler::Error> {
        match r {
            "equity" => Ok(ExportType::Equity),
            "identity" => Ok(ExportType::Identity),
            _ => Err(format!("Unknown export type {r}").into()),
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
    fn from(r: &str) -> Result<Self, tackler::Error> {
        match r {
            "tree" => Ok(BalanceType::Tree),
            "flat" => Ok(BalanceType::Flat),
            _ => Err(format!("Unknown balance type {r}").into()),
        }
    }
}

enum Timezone {}

pub(crate) type AccountSelectors = Vec<String>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub(crate) kernel: Kernel,
    pub(crate) price: Price,
    pub(crate) transaction: Transaction,
    pub(crate) report: Report,
    pub(crate) export: Export,
}

impl Config {
    pub fn from<P: AsRef<Path>>(cfg_path: P) -> Result<Config, tackler::Error> {
        let cfg_raw: ConfigRaw = toml::from_str(fs::read_to_string(&cfg_path)?.as_str())?;

        Ok(Config {
            kernel: Kernel::from(&cfg_raw.kernel)?,
            price: cfg_raw.price.map_or(Ok(Price::default()), |raw_price| {
                Price::try_from(&cfg_path, &raw_price)
            })?,
            transaction: Transaction::from(cfg_path, &cfg_raw.transaction)?,
            report: Report::from(&cfg_raw.report)?,
            export: { Export::from(&cfg_raw.export, &cfg_raw.report)? },
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Kernel {
    pub(crate) strict: bool,
    pub(crate) timestamp: Timestamp,
    pub(crate) audit: Audit,
    pub input: Input,
}
impl Kernel {
    fn from(k_raw: &KernelRaw) -> Result<Kernel, tackler::Error> {
        let k = Kernel {
            strict: k_raw.strict,
            timestamp: Timestamp::from(&k_raw.timestamp)?,
            audit: Audit::from(&k_raw.audit)?,
            input: Input::from(&k_raw.input)?,
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
            (None, Some(offset)) => match BrokenDownTime::parse("%:z", offset)?.offset() {
                Some(tm) => tm.to_time_zone(),
                None => {
                    let msg = format!("can't parse offset '{offset}' as valid offset");
                    return Err(msg.into());
                }
            },
            (None, None) => jiff::tz::Offset::UTC.to_time_zone(),
        };
        Ok(tz)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub storage: StorageType,
    pub fs: Option<FS>,
    pub git: Option<Git>,
}
impl Input {
    fn from(input_raw: &InputRaw) -> Result<Input, tackler::Error> {
        // todo: checks
        let i = Input {
            storage: StorageType::from(input_raw.storage.as_str())?,
            fs: match &input_raw.fs {
                Some(fs) => Some(FS::from(fs)?),
                None => None,
            },
            git: match &input_raw.git {
                Some(git) => Some(Git::from(git)?),
                None => None,
            },
        };
        Ok(i)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FS {
    pub dir: String,
    pub suffix: String,
}
impl FS {
    fn from(fs_raw: &FsRaw) -> Result<FS, tackler::Error> {
        let dir = match &fs_raw.path {
            Some(path) => format!("{}/{}", path, fs_raw.dir),
            None => fs_raw.dir.clone(),
        };
        Ok(FS {
            dir,
            suffix: fs_raw.suffix.clone(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Git {
    pub repo: String,
    pub git_ref: String,
    pub dir: String,
    pub suffix: String,
}
impl Git {
    fn from(git_raw: &GitRaw) -> Result<Git, tackler::Error> {
        let repo = match &git_raw.repo {
            Some(repo) => repo.clone(),
            None => match &git_raw.repository {
                Some(repo) => repo.clone(),
                None => {
                    let msg = "Git is missing 'repo' key";
                    return Err(msg.into());
                }
            },
        };
        Ok(Git {
            repo,
            git_ref: git_raw.git_ref.clone(),
            dir: git_raw.dir.clone(),
            suffix: git_raw.suffix.clone(),
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
            NONE_VALUE => match lookup_type {
                PriceLookupType::None => Ok(Price::default()),
                _ => {
                    let msg = "Price database path is 'none' but lookup type is not 'none'";
                    Err(msg.into())
                }
            },
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
        match accs_path_str {
            NONE_VALUE => Ok(Accounts::default()),
            _ => {
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
        match comm_path_str {
            NONE_VALUE => Ok(Commodities {
                permit_empty_commodity: Some(true),
                names: Vec::new(),
            }),
            _ => {
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
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Tags {
    pub(crate) names: Vec<String>,
}

impl Tags {
    fn from<P: AsRef<Path>>(path: P, tags_path_raw: &TagsPathRaw) -> Result<Tags, tackler::Error> {
        let tags_path_str = tags_path_raw.path.as_str();
        match tags_path_str {
            NONE_VALUE => Ok(Tags::default()),
            _ => {
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
}

#[derive(Debug, Clone)]
pub(crate) struct Report {
    pub report_tz: TimeZone,
    pub targets: Vec<ReportType>,
    pub scale: Scale,
    pub commodity: Option<Arc<Commodity>>,
    pub register: Register,
    pub balance_group: BalanceGroup,
    pub balance: Balance,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            report_tz: jiff::tz::TimeZone::UTC,
            targets: Vec::new(),
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
        let trgs = to_report_targets(&report_raw.targets)?;
        Ok(Report {
            report_tz: TimeZone::get(report_raw.report_tz.as_str())?,
            targets: trgs,
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
    acc_sel: &Option<AccountSelectors>,
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
            acc_sel: get_account_selector(&reg_raw.acc_sel, report),
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
                Some(t) => BalanceType::from(t.as_str())?,
                None => BalanceType::default(),
            },
            group_by: GroupBy::from(balgrp_raw.group_by.as_str())?,
            acc_sel: get_account_selector(&balgrp_raw.acc_sel, report),
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
                Some(t) => BalanceType::from(t.as_str())?,
                None => BalanceType::default(),
            },
            acc_sel: get_account_selector(&bal_raw.acc_sel, report),
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
            equity: Equity::from(&export_raw.equity, report)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Equity {
    pub(crate) equity_account: String,
    pub(crate) acc_sel: AccountSelectors,
}

impl Equity {
    fn from(eq_raw: &EquityRaw, report: &ReportRaw) -> Result<Equity, tackler::Error> {
        Ok(Equity {
            equity_account: eq_raw.equity_account.clone(),
            acc_sel: get_account_selector(&eq_raw.acc_sel, report),
        })
    }
}
