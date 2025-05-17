/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingAmountGreater;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingAmountGreater {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.amount > self.amount && self.regex.is_match(&p.acctn.atn.account))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_posts_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use rust_decimal::Decimal;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: b94b99d7-acfa-4a4b-871f-c1b6282738ff
    // desc: filter by posting amount (greater)
    fn posting_amount_greater() {
        let tf = TxnFilterPostingAmountGreater {
            regex: Regex::new("e:.*:abc").unwrap(/*:test:*/),
            amount: Decimal::new(3, 0),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (make_posts_txn("e:the:abc", -5, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 2, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 3, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 4, "a:the:def"), true),
            (make_posts_txn("e:not:b:c", 4, "a:the:def"), false),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: dde614b5-d368-4550-98bd-dc2e2e36aa9e
        // desc: TxnFilter::TxnFilterPostingAmountGreater
        let filt = TxnFilter::TxnFilterPostingAmountGreater(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
