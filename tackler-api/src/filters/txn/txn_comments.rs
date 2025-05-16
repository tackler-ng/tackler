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

/// Txn Comment filter
///
/// Select transaction if its comments matches specified `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnComments {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterTxnComments {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Txn Comments: \"{}\"",
            peeled_pattern(&self.regex)
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
    use tackler_rs::regex::new_full_haystack_regex;

    #[test]
    // test: 3debf0d5-599f-41a2-9d5f-e16a54cb1e3e
    // desc: TxnComment, full haystack match
    fn txn_comments_full_haystack() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnComments":{"regex":"o.a"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnComments(f) = &tf.txn_filter {
            assert!(!f.regex.is_match("foobar"));
            assert!(!f.regex.is_match("obar"));
            assert!(!f.regex.is_match("ooba"));

            assert!(f.regex.is_match("oba"));
        } else {
            panic!(/*:test:*/)
        }
    }

    #[test]
    // test: de0054ff-92e2-4837-b223-40cbbeaa90de
    // desc: TxnComments, JSON
    fn txn_comments_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnComments":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn Comments: \"(abc.*)|(def.*)\"
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnComments(_) = tf.txn_filter {
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
    // test: 5f08fe58-4451-4659-a684-d9725259ce2d
    // desc: TxnComments, Text
    fn txn_comments_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Txn Comments: "(abc.*)|(def.*)"
           |    AND
           |      Txn Comments: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnComments(TxnFilterTxnComments {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnComments(TxnFilterTxnComments {
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
