/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
mod account_spec;

use crate::config::AccountSelectors;
pub(crate) use crate::config::spec_items::account_spec::AccountSpec;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ConfigSpec {
    pub(super) kernel: KernelSpec,
    pub(super) price: Option<PriceSpec>,
    pub(super) transaction: TransactionSpec,
    pub(super) report: ReportSpec,
    pub(super) export: ExportSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct KernelSpec {
    pub(super) strict: bool,
    pub(super) timestamp: TimestampSpec,
    pub(super) audit: AuditSpec,
    #[serde(rename = "ext-id")]
    pub(super) extid: Option<ExtIdSpec>,
    pub(super) input: InputSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TimestampSpec {
    #[serde(rename = "default-time")]
    pub(super) default_time: toml::value::Time,
    pub(super) timezone: TimezoneSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TimezoneSpec {
    pub(super) name: Option<String>,
    pub(super) offset: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditSpec {
    pub(super) hash: String,
    pub(super) mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "ext-id")]
pub(super) struct ExtIdSpec {
    pub(super) unique: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct InputSpec {
    pub(super) storage: String,
    pub(super) fs: Option<FsSpec>,
    pub(super) git: Option<GitSpec>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FsSpec {
    // new key
    pub(super) path: Option<String>,
    pub(super) dir: String,
    // old key
    pub(super) suffix: Option<String>,
    // new key
    pub(super) ext: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GitSpec {
    // old key
    pub(super) repository: Option<String>,
    // new key
    pub(super) repo: Option<String>,
    #[serde(rename = "ref")]
    pub(super) git_ref: String,
    pub(super) dir: String,
    // old key
    pub(super) suffix: Option<String>,
    // new key
    pub(super) ext: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PriceSpec {
    #[serde(rename = "db-path")]
    pub(super) db_path: String,
    #[serde(rename = "lookup-type")]
    pub(super) lookup_type: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TransactionSpec {
    pub(super) accounts: AccountsPathSpec,
    pub(super) commodities: CommoditiesPathSpec,
    pub(super) tags: TagsPathSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AccountsPathSpec {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CommoditiesPathSpec {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TagsPathSpec {
    pub(super) path: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AccountsSpec {
    #[serde(rename = "accounts")]
    pub(super) items: Vec<AccountSpec>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CommoditiesSpec {
    #[serde(rename = "permit-empty-commodity")]
    pub(crate) permit_empty_commodity: Option<bool>,

    #[serde(rename = "commodities")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TagsSpec {
    #[serde(rename = "tags")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReportSpec {
    #[serde(rename = "report-timezone")]
    pub(super) report_tz: String,
    pub(super) targets: Vec<String>,
    pub(super) formats: Option<Vec<String>>,
    pub(super) accounts: Option<Vec<String>>,
    pub(super) scale: ScaleSpec,
    pub(super) commodity: Option<String>,
    pub(super) register: RegisterSpec,
    #[serde(rename = "balance-group")]
    pub(super) balance_group: BalanceGroupSpec,
    pub(super) balance: BalanceSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ScaleSpec {
    pub(super) min: u32,
    pub(super) max: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BalanceSpec {
    pub(super) title: String,
    #[serde(rename = "type")]
    pub(super) bal_type: Option<String>,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BalanceGroupSpec {
    pub(super) title: String,
    #[serde(rename = "type")]
    pub(super) bal_type: Option<String>,
    #[serde(rename = "group-by")]
    pub(super) group_by: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RegisterSpec {
    pub(super) title: String,
    #[serde(rename = "timestamp-style")]
    pub(super) timestamp_style: Option<String>,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ExportSpec {
    pub(super) targets: Vec<String>,

    pub(super) equity: EquitySpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct EquitySpec {
    #[serde(rename = "equity-account")]
    pub(super) equity_account: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}
