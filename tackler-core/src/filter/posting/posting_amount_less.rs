/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingAmountLess;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingAmountLess {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.amount < self.amount && self.regex.is_match(&p.acctn.atn.account))
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
    // test: 315d5ac3-28cf-417e-98bb-b738f209f5da
    // desc: filter by posting amount (less)
    fn posting_amount_less() {
        let tf = TxnFilterPostingAmountLess {
            regex: Regex::new("e:.*:abc").unwrap(/*:test:*/),
            amount: Decimal::new(3, 0),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (make_posts_txn("e:the:abc", -5, "a:the:def"), true),
            (make_posts_txn("e:the:abc", 2, "a:the:def"), true),
            (make_posts_txn("e:not:b:c", 2, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 3, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 4, "a:the:def"), false),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: c245f18e-44e1-4d89-ba2f-0e6283fd5c37
        // desc: TxnFilter::TxnFilterPostingAmountLess
        let filt = TxnFilter::TxnFilterPostingAmountLess(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
