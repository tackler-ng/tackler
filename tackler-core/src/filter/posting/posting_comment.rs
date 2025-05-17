/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::posting::TxnFilterPostingComment;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterPostingComment {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.comment.as_ref().is_some_and(|c| self.regex.is_match(c)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::model::{AccountTreeNode, Commodity, Posting, Transaction, TxnAccount};
    use regex::Regex;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use tackler_api::filters::TxnFilter;
    use tackler_api::txn_header::TxnHeader;

    pub(crate) fn make_posts_comment_txn(
        comment: Option<&str>,
        a: &str,
        a_value: i64,
        e: &str,
    ) -> Transaction {
        let e_v = Decimal::new(a_value, 0);
        let e_acctn = Arc::new(AccountTreeNode::from(e).unwrap(/*:test:*/));
        let e_txntn = TxnAccount {
            atn: e_acctn,
            comm: Arc::new(Commodity::default()),
        };

        let e_p = Posting::from(e_txntn, e_v, e_v, false, Arc::new(Commodity::default()), comment.map(str::to_string)).unwrap(/*:test:*/);

        let a_v = Decimal::new(-a_value, 0);
        let a_acctn = Arc::new(AccountTreeNode::from(a).unwrap(/*:test:*/));
        let a_txntn = TxnAccount {
            atn: a_acctn,
            comm: Arc::new(Commodity::default()),
        };
        let a_p = Posting::from(a_txntn, a_v, a_v, false, Arc::new(Commodity::default()), None).unwrap(/*:test:*/);

        Transaction::try_from(TxnHeader::default(), vec![e_p, a_p]).unwrap(/*:test:*/)
    }

    #[test]
    // test: 0c1dcffe-152d-4959-89bb-2c48677ad171
    // desc: filter by posting comments
    fn posting_comment() {
        let tf = TxnFilterPostingComment {
            regex: Regex::new("abc.*").unwrap(/*:test:*/),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (
                make_posts_comment_txn(Some(""), "a:the:abc", 123, "e:the:def"),
                false,
            ),
            (
                make_posts_comment_txn(None, "a:the:abc", 123, "e:the:def"),
                false,
            ),
            (make_posts_comment_txn(Some("abc"), "a", 123, "e"), true),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: e8517888-b459-4ecf-a622-592fd16aa067
        // desc: TxnFilter::TxnFilterPostingComment
        let filt = TxnFilter::TxnFilterPostingComment(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
