/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use crate::{filters, tackler};
use filters::TxnFilter;

use crate::filters::logic::logic_serde;
use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Logical AND-filter
///
/// All filters must be select a transaction, so that it will be selected.
///
/// Actual filtering implementation is done by Trait [`FilterTxn`]
///
/// [`FilterTxn`]: ../../../tackler_core/filter/index.html
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterAND {
    #[doc(hidden)]
    #[serde(rename = "txnFilters", with = "logic_serde")]
    pub txn_filters: Vec<TxnFilter>,
}

impl TxnFilterAND {
    /// Create a new AND filter based on provided filters
    ///
    /// # Errors
    /// Return Err if there are less than two filters
    pub fn new(filters: Vec<TxnFilter>) -> Result<TxnFilterAND, tackler::Error> {
        if filters.len() > 1 {
            Ok(TxnFilterAND {
                txn_filters: filters,
            })
        } else {
            let msg = "Expected multiple filters for logical AND filter";
            Err(msg.into())
        }
    }
}

impl IndentDisplay for TxnFilterAND {
    fn i_fmt(&self, indent: &str, tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        filters::logic_filter_indent_fmt("AND", indent, tz, &self.txn_filters, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefZoned, FilterDefinition, NullaryFALSE, NullaryTRUE};
    use indoc::indoc;
    use jiff::tz;
    use tackler_rs::IndocUtils;

    #[test]
    // test: aa8aa459-b100-403e-98ea-7381ca58727d
    // desc: reject AND filter with only one filter
    fn and_with_one_filter() {
        let v = vec![TxnFilter::NullaryTRUE(NullaryTRUE {})];
        let tf_res: Result<TxnFilterAND, tackler::Error> = TxnFilterAND::new(v);
        assert!(
            tf_res.unwrap_err(/*:test:*/).to_string().contains("Expected multiple filters for logical AND filter")
        );
    }

    #[test]
    // test: 2671b0ff-8b8d-42c8-95ae-e2dcf4d15ab0
    // desc: reject AND filter with only one filter (JSON)
    fn and_with_one_filter_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterAND":{"txnFilters":[{"NullaryTRUE":{}}]}}}"#;
        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(
            tf_res.unwrap_err(/*:test:*/).to_string().contains("Expected multiple filters for logical filter")
        );
    }

    #[test]
    // test: caa264f6-719f-49e9-9b56-3bdf0b0941ec
    // desc: AND, JSON
    fn and_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterAND":{"txnFilters":[{"NullaryTRUE":{}},{"NullaryFALSE":{}}]}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    All pass
         |    None pass
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterAND(_) = tf.txn_filter {
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
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: deda9918-cba5-4b3d-85db-61a3a7e1128f
    // desc: AND, Text
    fn and_filt_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    All pass
         |    AND
         |      All pass
         |      None pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
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
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
    }
}
