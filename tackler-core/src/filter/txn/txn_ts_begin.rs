/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use std::cmp::Ordering;
use tackler_api::filters::txn::TxnFilterTxnTSBegin;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnTSBegin {
    fn eval(&self, txn: &Transaction) -> bool {
        match self.begin.cmp(&txn.header.timestamp.timestamp()) {
            Ordering::Less | Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_ts_txn;
    use tackler_api::filters::TxnFilter;
    use tackler_api::txn_ts::rfc3339_to_zoned;

    #[test]
    // test: 701b2c27-d33c-4460-9a5e-64316c6ed946
    // desc: filter by date
    fn filter_by_date() {
        let tf = TxnFilterTxnTSBegin {
            begin: "2018-02-01T00:00:00+00:00".parse().unwrap(/*:test:*/),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T00:00:00+00:00", false),
            ("2018-02-01T00:00:00+00:00", true),
            ("2018-03-01T00:00:00+00:00", true),
        ];

        for t in &cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }

        // test: 42dfcaca-b407-437e-9bc0-7f9618c1636e
        // desc: TxnFilter::TxnFilterTxnTSBegin
        let filt = TxnFilter::TxnFilterTxnTSBegin(tf);
        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(filt.eval(&txn), t.1);
        }
    }

    #[test]
    // test: ec7cf2bd-e10e-4f46-9baa-4096881a5fbb
    // desc: filter by time
    fn filter_by_time() {
        let tf = TxnFilterTxnTSBegin {
            begin: "2018-01-01T23:00:00+00:00".parse().unwrap(/*:test:*/),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T11:00:00+00:00", false),
            ("2018-01-01T23:00:00+00:00", true),
            ("2018-01-02T00:00:00+00:00", true),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: f1623bd0-f767-458e-bc68-6eadfa113fd1
    // desc: filter by nanoseconds
    fn filter_by_nanosecond() {
        let tf = TxnFilterTxnTSBegin {
            begin: "2018-01-01T14:00:00.123456788+00:00".parse().unwrap(),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T14:00:00.123456787+00:00", false),
            ("2018-01-01T14:00:00.123456788+00:00", true),
            ("2018-01-01T14:00:00.123456789+00:00", true),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: 960cb7e7-b180-4276-a43b-714e53e1789b
    // desc: filter by timezone
    fn filter_by_timezone() {
        let tf = TxnFilterTxnTSBegin {
            begin: "2018-01-04T00:00:00+00:00".parse().unwrap(/*:test:*/),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-04T09:00:00+10:00", false),
            ("2018-01-03T18:00:00-06:00", true),
            ("2018-01-04T00:00:00+00:00", true),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }
}
