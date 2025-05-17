/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterTxnCode;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnCode {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header
            .code
            .as_ref()
            .is_some_and(|code| self.regex.is_match(code))
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
    // test: 54c746cf-916f-4c24-8e53-d4306917a200
    // desc: filter by txn code
    fn txn_code() {
        let tf = TxnFilterTxnCode {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

        #[allow(clippy::type_complexity)]
        let cases: Vec<(fn(Option<&str>) -> Transaction, Option<&str>, bool)> = vec![
            (make_default_txn, None, false),
            (make_code_txn, Some(""), false),
            (make_code_txn, Some("abc"), true),
            (make_code_txn, Some("foo"), false),
            (make_desc_txn, Some("abc"), false),
        ];

        for t in &cases {
            let txn = t.0(t.1);
            assert_eq!(tf.eval(&txn), t.2);
        }

        // test: e8addeae-4f6c-46d4-9031-bced93e2b07b
        // desc: TxnFilter::TxnFilterTxnCode
        let filt = TxnFilter::TxnFilterTxnCode(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.eval(&txn), t.2);
        }
    }
}
