/*
 * Tackler-NG 2022
 * SPDX-License-Identifier: Apache-2.0
 */

//! Txn Set and Report metadata
//!
pub mod items;

use items::MetadataItem;
use items::MetadataItems;
use items::Text;
use jiff::tz::TimeZone;
use serde::Serialize;

/// Metadata of Inputs, Txn Set, Reporting parameters, etc.
///
#[derive(Serialize, Debug, Clone, Default)]
pub struct Metadata {
    // todo: fix pub access
    #[doc(hidden)]
    pub items: MetadataItems,
}

impl Metadata {
    /// Get new empty `metadata`
    #[must_use]
    pub fn new() -> Metadata {
        Metadata { items: Vec::new() }
    }

    /// Get new metadata with existing Metadata item
    #[must_use]
    pub fn from_mdi(mdi: MetadataItem) -> Metadata {
        let items = vec![mdi];

        Metadata { items }
    }

    /// Get new metadata from existing Metadata.
    ///
    /// If there is an existing [`TxnSetChecksum`](items::TxnSetChecksum) metadata item,
    /// it will be removed from the new set.
    #[must_use]
    pub fn from_metadata(md: &Metadata) -> Metadata {
        let mut metadata = Metadata::new();
        for mdi in &md.items {
            match mdi {
                MetadataItem::TxnSetChecksum(_) => (),
                _ => metadata.push(mdi.clone()),
            }
        }
        metadata
    }

    /// Add metadata item into metadata
    pub fn push(&mut self, mdi: MetadataItem) {
        self.items.push(mdi);
    }

    /// Get textual representation of Metadata
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn text(&self, tz: TimeZone) -> String {
        let ts = self
            .items
            .iter()
            .flat_map(|item| {
                let mut vs = item.text(tz.clone());
                // put a newline between metadata items
                vs.push(String::default());
                vs
            })
            .collect::<Vec<String>>();
        ts.join("\n")
    }

    /// Test if metadata is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Generic checksum value
#[derive(Serialize, Debug, Clone)]
pub struct Checksum {
    /// used hash algorithm
    pub algorithm: String,
    /// hexadecimal hash value
    pub value: String,
}
