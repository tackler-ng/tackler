/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module contains various Metadata items
//!

use crate::filters::{FilterDefZoned, FilterDefinition};
use crate::metadata::Checksum;
use crate::txn_ts;
use jiff::Zoned;
use jiff::tz::TimeZone;
use serde::Serialize;

#[doc(hidden)]
pub type MetadataItems = Vec<MetadataItem>;

#[doc(hidden)]
pub trait Text: std::fmt::Debug {
    /// Get metadata item as text
    #[must_use]
    fn text(&self, tz: TimeZone) -> Vec<String>;
}

#[doc(hidden)]
#[derive(Serialize, Debug, Clone)]
pub enum MetadataItem {
    #[doc(hidden)]
    TxnSetChecksum(TxnSetChecksum),
    #[doc(hidden)]
    TimeZoneInfo(TimeZoneInfo),
    #[doc(hidden)]
    AccountSelectorChecksum(AccountSelectorChecksum),
    #[doc(hidden)]
    GitInputReference(GitInputReference),
    #[doc(hidden)]
    TxnFilterDescription(TxnFilterDescription),
    #[doc(hidden)]
    PriceRecords(PriceRecords),
}

impl MetadataItem {
    pub const ITEM_PAD: usize = 15;
}

impl Text for MetadataItem {
    fn text(&self, tz: TimeZone) -> Vec<String> {
        match self {
            Self::GitInputReference(gif) => gif.text(tz),
            Self::TxnSetChecksum(tscs) => tscs.text(tz),
            Self::TimeZoneInfo(tzinfo) => tzinfo.text(tz),
            Self::AccountSelectorChecksum(asc) => asc.text(tz),
            Self::TxnFilterDescription(tfd) => tfd.text(tz),
            Self::PriceRecords(pr) => pr.text(tz),
        }
    }
}

/// Txn Set Checksum metadata item
#[derive(Serialize, Debug, Clone)]
pub struct TxnSetChecksum {
    /// size of transaction set
    pub size: usize,
    /// hash of Txn Set Checksum
    pub hash: Checksum,
}
impl Text for TxnSetChecksum {
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = MetadataItem::ITEM_PAD;
        vec![
            format!("Txn Set Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
            format!("{:>pad$} : {}", "set size", self.size),
        ]
    }
}

/*
/// Report timezone information

#[derive(Serialize, Debug, Clone)]
pub struct TimeZoneInfo {
    #[serde(rename = "zoneId")]
    /// IANA ZoneID
    pub zone_id: String,
}
*/

/// Account Selector Checksum item
#[derive(Serialize, Debug, Clone)]
pub struct AccountSelectorChecksum {
    /// Account selector checksum
    pub hash: Checksum,
    /// Account selectors
    pub selectors: Vec<String>,
}
impl Text for AccountSelectorChecksum {
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = MetadataItem::ITEM_PAD;
        let mut t = vec![
            format!("Account Selector Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
        ];
        if !self.selectors.is_empty() {
            let sel_txt = if self.selectors.len() > 1 {
                "selectors"
            } else {
                "selector"
            };
            let l = format!(
                "{:>pad$} : '{}'",
                sel_txt,
                &self.selectors.first().unwrap(/*:ok*/)
            );
            t.push(l);
            for s in self.selectors.iter().skip(1) {
                let l = format!("{:>pad$} | '{}'", "", s);
                t.push(l);
            }
        }
        t
    }
}

/// Report timezone item
#[derive(Serialize, Debug, Clone)]
pub struct TimeZoneInfo {
    /// Timezone name
    #[serde(rename = "zoneId")]
    pub zone_id: String,
}
impl Text for TimeZoneInfo {
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;
        vec![
            "Report Time Zone".to_string(),
            format!("{:>pad$} : {}", "TZ name", &self.zone_id),
        ]
    }
}
/// Metadata information about active Txn Filters
///
#[derive(Serialize, Debug, Clone)]
pub struct TxnFilterDescription {
    #[doc(hidden)]
    #[serde(rename = "txnFilterDef")]
    txn_filter_def: FilterDefinition,
}

impl TxnFilterDescription {
    /// Make Txn filter Description from Filter Definition
    ///
    #[must_use]
    pub fn from(tf: FilterDefinition) -> TxnFilterDescription {
        TxnFilterDescription { txn_filter_def: tf }
    }
}
impl Text for TxnFilterDescription {
    fn text(&self, tz: TimeZone) -> Vec<String> {
        // todo: TxnFilterDescription needs proper implementation for Text
        //       See equity_exporter::write_export
        format!(
            "{}",
            FilterDefZoned {
                filt_def: &self.txn_filter_def,
                tz
            }
        )
        .trim_end()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>()
    }
}

/// Metadata information about Git Txn input
///
#[derive(Serialize, Debug, Clone)]
pub struct GitInputReference {
    /// commit id
    pub commit: String,

    /// Symbolic git reference `main`, `Y2023`, etc.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Transaction directory ("journal") inside repository
    pub dir: String,

    /// Extension of journal filenames
    pub extension: String,

    /// Commit author
    pub author: String,

    /// Commit date
    pub date: String,

    /// Subject line of selected commit
    pub subject: String,
}

impl Text for GitInputReference {
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;
        vec![
            format!("Git Storage"),
            format!(
                "{:>pad$} : {}",
                "reference",
                self.reference
                    .as_ref()
                    .unwrap_or(&"FIXED by commit".to_string())
            ),
            format!("{:>pad$} : {}", "directory", self.dir),
            format!("{:>pad$} : {}", "extension", self.extension),
            format!("{:>pad$} : {}", "commit", self.commit),
            format!("{:>pad$} : {}", "author", self.author),
            format!("{:>pad$} : {}", "date", self.date),
            format!("{:>pad$} : {}", "subject", self.subject),
        ]
    }
}

/// Metadata item for one commodity conversion
#[derive(Serialize, Debug, Clone)]
pub struct PriceRecord {
    /// Time of price record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<Zoned>,
    /// Source (from) commodity
    pub source: String,
    /// Conversion rate (value in target commodity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Target (to) commodity
    pub target: String,
}
impl Text for PriceRecord {
    fn text(&self, tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;
        vec![
            format!(
                "{:>pad$} : {}",
                "Time",
                self.ts.as_ref().map_or("At txn time".to_string(), |ts| {
                    txn_ts::as_tz_full(ts, tz)
                })
            ),
            format!("{:>pad$} : {}", "Commodity", self.source),
            format!(
                "{:>pad$} : {} {}",
                "Value",
                self.rate.clone().map_or("-".to_string(), |v| v),
                self.target
            ),
        ]
    }
}
/// Metadata information of used commodity conversions
#[derive(Serialize, Debug, Clone)]
pub struct PriceRecords {
    /// Collection of used commodity conversions prices / rates
    pub rates: Vec<PriceRecord>,
}
impl Text for PriceRecords {
    fn text(&self, tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;

        let mut txt = Vec::new();

        if let Some(pr) = self.rates.first() {
            txt.push("Commodity Prices".to_string());
            txt.extend(pr.text(tz.clone()));

            if self.rates.len() > 1 {
                for pr in &self.rates[1..] {
                    txt.push(format!("{:>pad$} -", ""));
                    txt.extend(pr.text(tz.clone()));
                }
            }
        }
        txt
    }
}
