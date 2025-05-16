/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use jiff::tz::TimeZone;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

/// Txn Posting Account filter
///
/// Select the transaction, if any of its posting match `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingAccount {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterPostingAccount {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Posting Account: \"{}\"",
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
    // test: fdb5c728-1354-4905-8bc0-42c17cc6d948
    // desc: PostingAccount, full haystack match
    fn posting_account_full_haystack() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAccount":{"regex":"o.a"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterPostingAccount(f) = &tf.txn_filter {
            assert!(!f.regex.is_match("foobar"));
            assert!(!f.regex.is_match("obar"));
            assert!(!f.regex.is_match("ooba"));

            assert!(f.regex.is_match("oba"));
        } else {
            panic!(/*:test:*/)
        }
    }

    #[test]
    // test: 44d80d6d-b2cf-47a0-a228-bb2ea068f9f5
    // desc: PostingAccount, JSON
    fn posting_account_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingAccount":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Posting Account: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterPostingAccount(_) = tf.txn_filter {
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
    // test: 382e7f39-90e6-44f0-9162-150e2b353cef
    // desc: PostingAccount, Text
    fn posting_account_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Posting Account: "(abc.*)|(def.*)"
           |    AND
           |      Posting Account: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAccount(TxnFilterPostingAccount {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAccount(TxnFilterPostingAccount {
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
