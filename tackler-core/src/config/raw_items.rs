/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::AccountSelectors;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ConfigRaw {
    pub(super) kernel: KernelRaw,
    pub(super) price: Option<PriceRaw>,
    pub(super) transaction: TransactionRaw,
    pub(super) report: ReportRaw,
    pub(super) export: ExportRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct KernelRaw {
    pub(super) strict: bool,
    pub(super) timestamp: TimestampRaw,
    pub(super) audit: AuditRaw,
    pub(super) input: InputRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TimestampRaw {
    #[serde(rename = "default-time")]
    pub(super) default_time: toml::value::Time,
    pub(super) timezone: TimezoneRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TimezoneRaw {
    pub(super) name: Option<String>,
    pub(super) offset: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AuditRaw {
    pub(super) hash: String,
    pub(super) mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct InputRaw {
    pub(super) storage: String,
    pub(super) fs: Option<FsRaw>,
    pub(super) git: Option<GitRaw>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FsRaw {
    // new key
    pub(super) path: Option<String>,
    pub(super) dir: String,
    pub(super) suffix: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct GitRaw {
    // old key
    pub(super) repository: Option<String>,
    // new key
    pub(super) repo: Option<String>,
    #[serde(rename = "ref")]
    pub(super) git_ref: String,
    pub(super) dir: String,
    pub(super) suffix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PriceRaw {
    #[serde(rename = "db-path")]
    pub(super) db_path: String,
    #[serde(rename = "lookup-type")]
    pub(super) lookup_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TransactionRaw {
    pub(super) accounts: AccountsPathRaw,
    pub(super) commodities: CommoditiesPathRaw,
    pub(super) tags: TagsPathRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AccountsPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct CommoditiesPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TagsPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AccountsRaw {
    #[serde(rename = "accounts")]
    pub(super) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct CommoditiesRaw {
    #[serde(rename = "permit-empty-commodity")]
    pub(crate) permit_empty_commodity: Option<bool>,

    #[serde(rename = "commodities")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TagsRaw {
    #[serde(rename = "tags")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ReportRaw {
    #[serde(rename = "report-timezone")]
    pub(super) report_tz: String,
    pub(super) targets: Vec<String>,
    pub(super) accounts: Option<Vec<String>>,
    pub(super) scale: ScaleRaw,
    pub(super) commodity: Option<String>,
    pub(super) register: RegisterRaw,
    #[serde(rename = "balance-group")]
    pub(super) balance_group: BalanceGroupRaw,
    pub(super) balance: BalanceRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ScaleRaw {
    pub(super) min: u32,
    pub(super) max: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BalanceRaw {
    pub(super) title: String,
    #[serde(rename = "type")]
    pub(super) bal_type: Option<String>,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BalanceGroupRaw {
    pub(super) title: String,
    #[serde(rename = "type")]
    pub(super) bal_type: Option<String>,
    #[serde(rename = "group-by")]
    pub(super) group_by: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RegisterRaw {
    pub(super) title: String,
    #[serde(rename = "timestamp-style")]
    pub(super) timestamp_style: Option<String>,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ExportRaw {
    pub(super) targets: Vec<String>,

    pub(super) equity: EquityRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct EquityRaw {
    #[serde(rename = "equity-account")]
    pub(super) equity_account: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}
