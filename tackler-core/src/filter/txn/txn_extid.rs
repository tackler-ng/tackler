/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterTxnExtId;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnExtId {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header
            .extid
            .as_ref()
            .is_some_and(|extid| self.regex.is_match(extid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_extid_txn;
    use crate::filter::tests::{make_code_txn, make_default_txn};

    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 114cf7e0-5163-4607-8b13-e82e8ebdc076
    // desc: filter by txn extid
    fn txn_extid() {
        let tf = TxnFilterTxnExtId {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

        #[allow(clippy::type_complexity)]
        let cases: Vec<(fn(Option<&str>) -> Transaction, Option<&str>, bool)> = vec![
            (make_default_txn, None, false),
            (make_extid_txn, Some("abc"), true),
            (make_extid_txn, Some("foo"), false),
            (make_code_txn, Some("abc"), false),
        ];

        for t in &cases {
            let txn = t.0(t.1);
            assert_eq!(tf.eval(&txn), t.2);
        }

        // test: 5b2547e7-51eb-446c-abd0-0e00c8d9270f
        // desc: TxnFilter::TxnFilterTxnExtId
        let filt = TxnFilter::TxnFilterTxnExtId(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.eval(&txn), t.2);
        }
    }
}
