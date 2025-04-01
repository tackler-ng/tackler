/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::metadata::Metadata;
use crate::reports::balance_report::BalanceReport;
use serde::Serialize;

/// Balance Group report API object
#[derive(Serialize, Debug)]
pub struct BalanceGroupReport {
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Title of Balance Report
    pub title: String,

    /// Balance Groups
    pub groups: Vec<BalanceReport>,
}
