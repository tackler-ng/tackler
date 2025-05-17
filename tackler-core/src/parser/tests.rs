/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::transaction::Transaction;
use tackler_api::txn_ts;

fn txn_ts_to_string(txn: &Transaction) -> String {
    txn_ts::rfc_3339(&txn.header.timestamp)
}
fn txn_code_to_string(txn: &Transaction) -> String {
    txn.header.code.as_ref().unwrap(/*:test:*/).to_string()
}
fn txn_desc_to_string(txn: &Transaction) -> String {
    txn.header.description.as_ref().unwrap(/*:test:*/).to_string()
}
fn txn_uuid_to_string(txn: &Transaction) -> String {
    txn.header.uuid.as_ref().unwrap(/*:test:*/).to_string()
}
fn txn_geo_to_string(txn: &Transaction) -> String {
    format!("{}", &txn.header.location.as_ref().unwrap(/*:test:*/))
}
fn txn_tags_to_string(txn: &Transaction) -> String {
    txn.header.tags_to_string()
}

mod tackler_txns;
mod txn_accounts;
mod txn_commodities;
mod txn_header_code;
mod txn_header_desc;
mod txn_header_timestamp;
mod txn_location;
mod txn_metadata;
mod txn_tags;
mod txn_uuid;

#[cfg(test)]
pub(crate) fn content_data() -> Vec<String> {
    let data = [
        r##" !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##, // this has to be first for txn code test
        r##"!"#$%&*+,-./:;=?@\^_`|~"##,
        r"azAZ09",
        r"äöåÄÖÅèé",
        r"¼£$€¥",
        r"¡¿·",
        r"μ",
        r"─├└",
        "a b\tc",
        r"アイスクリーム",
        r"風空",
        r"мороженое",
        r"🦀🦀",
    ];

    data.iter().map(std::string::ToString::to_string).collect()
}
