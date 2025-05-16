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

    /// Amount of posting
    ///
    /// Commodity Conversion: Original amount before conversion in `base_commodity`
    pub amount: String,

    /// Running total for that account
    ///
    /// Commodity Conversion: In target commodity
    #[serde(rename = "runningTotal")]
    pub running_total: String,

    /// Posting commodity
    ///
    /// Commodity Conversion: Target commodity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<String>,

    /// Commodity Conversion: Rate if valuation is based on Txn Time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,

    /// Commodity Conversion: Original (source) commodity
    #[serde(rename = "baseCommodity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commodity: Option<String>,
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

    /// Register Txn (entry) postings
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
