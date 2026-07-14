/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::tz::TimeZone;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::IndentDisplay;

/// Txn External ID metadata filter
///
/// Select transaction if its txn external id matches specified `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnExtId {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterTxnExtId {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Txn Ext-Id: \"{}\"", peeled_pattern(&self.regex))
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
    // test: f5c2294f-074d-4cf5-a93e-8557e667574f
    // desc: TxnExtId, full haystack match
    fn txn_extid_full_haystack() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnExtId":{"regex":"o.a"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnExtId(f) = &tf.txn_filter {
            assert!(!f.regex.is_match("foobar"));
            assert!(!f.regex.is_match("obar"));
            assert!(!f.regex.is_match("ooba"));

            assert!(f.regex.is_match("oba"));
        } else {
            panic!(/*:test:*/)
        }
    }

    #[test]
    // test: 250895f4-e228-4f40-baaf-51cea984f01f
    // desc: TxnExtId, JSON
    fn txn_extid_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnExtId":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Txn Ext-Id: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnExtId(_) = tf.txn_filter {
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
    // test: a1bd4d35-644c-403a-9271-b856fcf82450
    // desc: TxnExtId, Text
    fn txn_extid_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Txn Ext-Id: "(abc.*)|(def.*)"
           |    AND
           |      Txn Ext-Id: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnExtId(TxnFilterTxnExtId {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnExtId(TxnFilterTxnExtId {
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
