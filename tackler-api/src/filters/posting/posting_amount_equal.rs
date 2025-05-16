/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::tz::TimeZone;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::{IndentDisplay, posting_filter_indent_fmt};

/// Txn Posting "Amount is equal" filter
///
/// Select the transaction, if its posting match `regex` with exact `amount`
///
/// Q: Why is there also account regex as parameter?
///
/// A: For consistency with less and greater, where it's mandatory.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingAmountEqual {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
    #[doc(hidden)]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

impl IndentDisplay for TxnFilterPostingAmountEqual {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        posting_filter_indent_fmt(
            indent,
            "Posting Amount",
            peeled_pattern(&self.regex),
            "==",
            &self.amount,
            f,
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
    // test: c63d9ff7-6039-474b-8b8a-be6b8927510f
    // desc: PostingAmountEqual, full haystack match
    fn posting_amount_equal_full_haystack() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingAmountEqual":{"regex":"o.a","amount":1}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterPostingAmountEqual(f) = &tf.txn_filter {
            assert!(!f.regex.is_match("foobar"));
            assert!(!f.regex.is_match("obar"));
            assert!(!f.regex.is_match("ooba"));

            assert!(f.regex.is_match("oba"));
        } else {
            panic!(/*:test:*/)
        }
    }

    #[test]
    // test: b7b4543d-2ffa-488f-b251-af5a7ba7204f
    // desc: PostingAmountEqual, JSON
    fn posting_amount_equal_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAmountEqual":{"regex":"(abc.*)|(def.*)","amount":1}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Posting Amount
           |    account: "(abc.*)|(def.*)"
           |    amount == 1
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterPostingAmountEqual(_) = tf.txn_filter {
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
    // test: c0f88d70-c388-4c4f-9cca-f29b921dbc41
    // desc: PostingAmountEqual, Text
    fn posting_amount_equal_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Posting Amount
           |      account: "(abc.*)|(def.*)"
           |      amount == 1
           |    AND
           |      Posting Amount
           |        account: "xyz"
           |        amount == 2
           |      All pass
           |"#}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                        amount: Decimal::from(1),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual {
                                regex: new_full_haystack_regex("xyz").unwrap(/*:test:*/),
                                amount: Decimal::from(2),
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
