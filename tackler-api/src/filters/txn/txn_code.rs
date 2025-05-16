/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::tz::TimeZone;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::IndentDisplay;

/// Txn Code filter
///
/// Select transaction if its code matches specified `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnCode {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterTxnCode {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Txn Code: \"{}\"", peeled_pattern(&self.regex))
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
    use tackler_rs::regex::new_full_haystack_regex;

    #[test]
    // test: 2e005db0-f3f5-4ef4-a574-ed3371d4f5c9
    // desc: TxnCode, full haystack match
    fn txn_code_full_haystack() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnCode":{"regex":"o.a"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnCode(f) = &tf.txn_filter {
            assert!(!f.regex.is_match("foobar"));
            assert!(!f.regex.is_match("obar"));
            assert!(!f.regex.is_match("ooba"));

            assert!(f.regex.is_match("oba"));
        } else {
            panic!(/*:test:*/)
        }
    }

    #[test]
    // test: 928a78b4-0ad7-4909-b145-3826acc75b3d
    // desc: TxnCode, JSON
    fn txn_code_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnCode":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Txn Code: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnCode(_) = tf.txn_filter {
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
    // test: 274ccbb4-dcd7-431d-bf05-5da1b191d74c
    // desc: TxnCode, Text
    fn txn_code_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Txn Code: "(abc.*)|(def.*)"
           |    AND
           |      Txn Code: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnCode(TxnFilterTxnCode {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnCode(TxnFilterTxnCode {
                                regex: new_full_haystack_regex("xyz").unwrap(/*:test:*/),
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
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
    }
}
