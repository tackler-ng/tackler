/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use crate::txn_ts::rfc_3339;
use jiff::Timestamp;
use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Txn TS Begin filter
///
/// Select transaction if its timestamp is on or after
/// specified `begin` time.
///
/// `begin` is inclusive timestamp in ISO 8601 format with offset
///
/// Time is expressed as ISO 8601 format with offset,
/// e.g. 2018-01-01T10:11:22.345+02:00
/// or 2018-01-01T08:11:22.345Z.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnTSBegin {
    #[doc(hidden)]
    pub begin: Timestamp,
}

impl IndentDisplay for TxnFilterTxnTSBegin {
    fn i_fmt(&self, indent: &str, tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Txn TS: begin {}",
            rfc_3339(&self.begin.to_zoned(tz))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{
        FilterDefZoned, FilterDefinition, NullaryTRUE, TxnFilter, logic::TxnFilterAND,
    };
    use indoc::indoc;
    use jiff::tz;
    use tackler_rs::IndocUtils;

    #[test]
    // test: baa0038e-45b7-4911-a647-859de2da4716
    // desc: TxnTSBegin, JSON
    fn txn_ts_begin_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.345+02:00"}}}"#;
        let filter_json_zulu_str =
            r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T08:11:22.345Z"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.345+02:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSBegin(_) = tf.txn_filter {
        } else {
            panic!(/*:test:*/)
        }

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::get("Etc/GMT-2").unwrap(/*:test:*/)
                }
            ),
            filter_text_str
        );
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_zulu_str
        );
    }

    #[test]
    // test: 7f2a2dd7-6a5c-4f59-8000-1ee451e1540f
    // desc: TxnTSBegin, JSON with Zulu zone
    fn txn_ts_begin_json_with_zulu() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.345Z"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.345+00:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSBegin(_) = tf.txn_filter {
        } else {
            panic!(/*:test:*/)
        }

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
    }

    #[test]
    // test: 9002e6e1-cee5-4751-a3e0-c64cea0091e6
    // desc: TxnTSBegin, JSON with nanoseconds
    fn txn_ts_begin_json_with_nano() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.123456789-05:00"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.123456789-05:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSBegin(_) = tf.txn_filter {
        } else {
            panic!(/*:test:*/)
        }

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::get("Etc/GMT+5").unwrap(/*:test:*/)
                }
            ),
            filter_text_str
        );
    }

    #[test]
    // test: c01de4f4-0e07-4d8d-a4c8-2d1ad28df264
    // desc: TxnTSBegin, Text
    fn txn_ts_begin_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn TS: begin 2023-02-25T10:11:22.345+02:00
         |    AND
         |      Txn TS: begin 2023-02-25T20:11:22.345+02:00
         |      All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnTSBegin(TxnFilterTxnTSBegin {
                        begin: "2023-02-25T10:11:22.345+02:00".parse()
                            .unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnTSBegin(TxnFilterTxnTSBegin {
                                begin: "2023-02-25T20:11:22.345+02:00".parse()
                                    .unwrap(/*:test:*/),
                            }),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::get("Etc/GMT-2").unwrap(/*:test:*/)
                }
            ),
            filter_text_str
        );
    }
}
