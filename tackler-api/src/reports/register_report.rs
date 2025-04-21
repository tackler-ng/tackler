/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::metadata::Metadata;
use crate::txn_header::TxnHeader;
use serde::Serialize;

/// Register posting API object
#[derive(Serialize, Debug)]
pub struct RegisterPosting {
    /// Account for register posting
    pub account: String,

    /// Amount of that txn
    pub amount: String,

    /// Running total for that account
    #[serde(rename = "runningTotal")]
    pub running_total: String,

    /// Posting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<String>,
}

/// Register transaction API object
#[derive(Serialize, Debug)]
pub struct RegisterTxn {
    /// Txn timestamp in display format
    /// This is controlled by conf key `report.register.timestamp-style`
    #[serde(rename = "displayTime")]
    pub display_time: String,
    /// Transaction header
    pub txn: TxnHeader,

    /// Txn postings
    pub postings: Vec<RegisterPosting>,
}

/// Register report API object
#[derive(Serialize, Debug)]
pub struct RegisterReport {
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Title of Balance Report
    pub title: String,

    /// Balance rows / items
    pub transactions: Vec<RegisterTxn>,
}
