/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters;
use crate::filters::IndentDisplay;
use filters::TxnFilter;

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
    // todo: functionality, test
    // todo-test: aa8aa459-b100-403e-98ea-7381ca58727d
    // desc: "reject AND filter with only one filter"
    #[doc(hidden)]
    #[serde(rename = "txnFilters")]
    pub txn_filters: Vec<TxnFilter>,
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
