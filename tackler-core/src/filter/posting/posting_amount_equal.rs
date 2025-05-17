/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingAmountEqual;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingAmountEqual {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.amount == self.amount && self.regex.is_match(&p.acctn.atn.account))
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
    use tackler_api::filters::{TxnFilter, posting::TxnFilterPostingAmountEqual};

    #[test]
    // test: de72fb67-14a7-4032-b2c2-b1049ecd0c35
    // desc: filter by posting amount (exact)
    fn posting_amount_exact() {
        let tf = TxnFilterPostingAmountEqual {
            regex: Regex::new("e:.*:abc").unwrap(/*:test:*/),
            amount: Decimal::new(3, 0),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (make_posts_txn("e:the:abc", -3, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 2, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 3, "a:the:def"), true),
            (make_posts_txn("e:not:b:c", 3, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 4, "a:the:def"), false),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: 57673de0-cd11-491d-98f6-a0bb3b44df80
        // desc: TxnFilter::TxnFilterPostingAmountEqual
        let filt = TxnFilter::TxnFilterPostingAmountEqual(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
