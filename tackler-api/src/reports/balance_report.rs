/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::metadata::Metadata;
use serde::Serialize;

/// One item / row in the balance report
#[derive(Serialize, Debug)]
pub struct BalanceItem {
    /// Sum of txns for this account
    #[serde(rename = "accountSum")]
    pub account_sum: String,

    /// Recursive sum of all txns for this account and all of it's children
    #[serde(rename = "accountTreeSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tree_sum: Option<String>,

    /// Full account name
    pub account: String,

    /// Optional commodity for this balance row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<String>,
}

/// One delta item / row of balance report (per commodity)
#[derive(Serialize, Debug)]
pub struct Delta {
    /// Amount of Delta (difference)
    pub delta: String,
    /// Optional commodity, if it's multi-currency balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<String>,
}

/// Balance report API object
#[derive(Serialize, Debug)]
pub struct BalanceReport {
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Title of Balance Report
    pub title: String,

    /// Balance rows / items
    pub balances: Vec<BalanceItem>,

    /// Balance deltas rows / items
    pub deltas: Vec<Delta>,
}
