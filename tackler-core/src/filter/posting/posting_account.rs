/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingAccount;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingAccount {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| self.regex.is_match(&p.acctn.atn.account))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_posts_commodity_txn;
    use crate::filter::tests::make_posts_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::{TxnFilter, posting::TxnFilterPostingAccount};

    #[test]
    // test: 7784049f-ef3e-4185-8d33-f8c78478eef1
    // desc: "filter by account name with wildcard at begin"
    fn posting_account() {
        let tf = TxnFilterPostingAccount {
            regex: Regex::new(".*:abc").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (make_posts_txn("a:the:abc", 123, "e:the:def"), true),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: c10b209c-7da7-4e44-acb1-a7b739ccddd5
        // desc: TxnFilter::TxnFilterPostingAccount
        let filt = TxnFilter::TxnFilterPostingAccount(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }

    #[test]
    // test: 23ffc331-dc31-4fe8-9783-ba2e26f3365e
    // desc: verify independence between account and commodity (account)
    fn posting_account_and_commodity() {
        let tf = TxnFilterPostingAccount {
            regex: Regex::new("^a:b:c$").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_posts_txn("a:b:c", 123, "e:the:def"), true),
            (
                make_posts_commodity_txn(None, "a:b:c", 123, "e:the:def"),
                true,
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
