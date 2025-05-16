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

/// Logical OR-filter
///
/// If any of the filters selects a transaction, then it will be selected.
///
/// Actual filtering implementation is done by Trait [`FilterTxn`]
///
/// [`FilterTxn`]: ../../../tackler_core/filter/index.html
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterOR {
    // todo: functionality, test
    // todo-test: f9088d6f-d3ae-4120-b420-e77d0ea26f11
    // desc: "reject OR filter with only one filter"
    #[doc(hidden)]
    #[serde(rename = "txnFilters")]
    pub txn_filters: Vec<TxnFilter>,
}

impl IndentDisplay for TxnFilterOR {
    fn i_fmt(&self, indent: &str, tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        filters::logic_filter_indent_fmt("OR", indent, tz, &self.txn_filters, f)
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
    // test: eddb393f-b8a4-4189-9280-40a911417b70
    // desc: OR, JSON
    fn or_filt_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterOR":{"txnFilters":[{"NullaryTRUE":{}},{"NullaryFALSE":{}}]}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  OR
         |    All pass
         |    None pass
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterOR(_) = tf.txn_filter {
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
    // test: 18959315-233a-4ede-8ec9-537951d45c6d
    // desc: OR, Text
    fn or_filt_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  OR
         |    All pass
         |    OR
         |      All pass
         |      None pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterOR(TxnFilterOR {
                txn_filters: vec![
                    TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    TxnFilter::TxnFilterOR(TxnFilterOR {
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
