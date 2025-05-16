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

/// Txn TS End filter
///
/// Select transaction if its timestamp is before
/// specified `end` time.
///
/// `end` is exclusive timestamp in ISO 8601 format with zone
///
/// Time is expressed as ISO 8601 format with zone,
/// e.g. 2018-01-01T10:11:22.345+02:00
/// or 2018-01-01T08:11:22.345Z.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnTSEnd {
    #[doc(hidden)]
    pub end: Timestamp,
}

impl IndentDisplay for TxnFilterTxnTSEnd {
    fn i_fmt(&self, indent: &str, tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Txn TS: end   {}",
            rfc_3339(&self.end.to_zoned(tz))
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
    // test: db171b86-7435-4e9b-bfa0-4288c720289c
    // desc: TxnTSEnd, JSON
    fn txn_ts_end_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSEnd":{"end":"2023-02-25T10:11:22.345+02:00"}}}"#;
        let filter_json_zulu_str =
            r#"{"txnFilter":{"TxnFilterTxnTSEnd":{"end":"2023-02-25T08:11:22.345Z"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: end   2023-02-25T10:11:22.345+02:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSEnd(_) = tf.txn_filter {
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
    // test: 5a195125-4fd0-413b-984b-4e3ca0899edb
    // desc: TxnTSEnd, JSON with Zulu zone
    fn txn_ts_end_json_with_zulu_zone() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSEnd":{"end":"2023-02-25T10:11:22.345Z"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: end   2023-02-25T10:11:22.345+00:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSEnd(_) = tf.txn_filter {
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
    // test: 98df4e3a-2ca4-4b55-b4a8-358a46331e65
    // desc: TxnTSEnd, JSON with nanoseconds
    fn txn_ts_end_json_with_nano() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSEnd":{"end":"2023-02-25T10:11:22.123456789-05:00"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: end   2023-02-25T10:11:22.123456789-05:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnTSEnd(_) = tf.txn_filter {
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
    // test: ef2348e6-3684-4a13-85e9-5aec89a9e3bb
    // desc: TxnTSEnd, Text
    fn txn_ts_end_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn TS: end   2023-02-25T10:11:22.345+02:00
         |    AND
         |      Txn TS: end   2023-02-25T20:11:22.345+02:00
         |      All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnTSEnd(TxnFilterTxnTSEnd {
                        end: "2023-02-25T10:11:22.345+02:00".parse()
                            .unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnTSEnd(TxnFilterTxnTSEnd {
                                end: "2023-02-25T20:11:22.345+02:00".parse()
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
