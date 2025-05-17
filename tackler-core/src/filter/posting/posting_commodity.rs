/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingCommodity;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingCommodity {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| self.regex.is_match(&p.acctn.comm.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_posts_commodity_txn;
    use crate::filter::tests::{make_default_txn, make_posts_txn};
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: cfb795cd-d323-4181-a76a-1e5ce957add7
    // desc: filter by posting commodity
    fn posting_commodity() {
        let tf = TxnFilterPostingCommodity {
            regex: Regex::new("EU.*").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (
                make_posts_commodity_txn(None, "a:the:EUR", 123, "e:the:def"),
                false,
            ),
            (make_posts_commodity_txn(Some("EUR"), "a", 123, "e"), true),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: 50edcbf2-5373-45f5-8c66-8ec7471001fe
        // desc: TxnFilter::TxnFilterPostingCommodity
        let filt = TxnFilter::TxnFilterPostingCommodity(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }

    #[test]
    // test: 49129f9c-f75d-464f-b8d0-485d8880b6b2
    // desc: verify independence between account and commodity (commodity I)
    fn posting_account_and_commodity_1() {
        let tf = TxnFilterPostingCommodity {
            regex: Regex::new("^EUR$").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_posts_txn("EUR", 123, "e:the:def"), false),
            (
                make_posts_commodity_txn(Some("EUR"), "a:b:c", 123, "e:the:def"),
                true,
            ),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }
    }

    #[test]
    // test: 3689e065-0372-46d0-a912-73a08a9b5fa2
    // desc: verify independence between account and commodity (commodity II)
    fn posting_account_and_commodity_2() {
        let tf = TxnFilterPostingCommodity {
            regex: Regex::new(".*EUR.*").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_posts_txn("EUR", 123, "e:the:def"), false),
            (
                make_posts_commodity_txn(Some("USD"), "EUR", 123, "e:the:def"),
                false,
            ),
            (
                make_posts_commodity_txn(Some("EUR"), "a:b:c", 123, "e:the:def"),
                true,
            ),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }
    }
}
