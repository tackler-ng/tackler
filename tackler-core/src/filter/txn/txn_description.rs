/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterTxnDescription;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnDescription {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header
            .description
            .as_ref()
            .is_some_and(|desc| self.regex.is_match(desc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_code_txn;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_desc_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 59157c61-0ced-4b3a-ab8d-ec5edf7aafb4
    // desc: filter by txn description
    fn txn_description() {
        let tf = TxnFilterTxnDescription {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

        #[allow(clippy::type_complexity)]
        let cases: Vec<(fn(Option<&str>) -> Transaction, Option<&str>, bool)> = vec![
            (make_default_txn, None, false),
            (make_desc_txn, Some(""), false),
            (make_desc_txn, Some("abc"), true),
            (make_desc_txn, Some("foo"), false),
            (make_code_txn, Some("abc"), false),
        ];

        for t in &cases {
            let txn = t.0(t.1);
            assert_eq!(tf.eval(&txn), t.2);
        }

        // test: 3bca6d7b-e515-42d8-b65b-2780b0c0d7e0
        // desc: TxnFilter::TxnFilterTxnCode
        let filt = TxnFilter::TxnFilterTxnDescription(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.eval(&txn), t.2);
        }
    }
}
