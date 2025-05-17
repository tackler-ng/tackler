/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use std::cmp::Ordering;
use tackler_api::filters::txn::TxnFilterTxnTSEnd;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnTSEnd {
    fn eval(&self, txn: &Transaction) -> bool {
        match txn.header.timestamp.timestamp().cmp(&self.end) {
            Ordering::Less => true,
            Ordering::Equal | Ordering::Greater => false,
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
    // test: 42a42f07-dea5-45ee-b563-187f9121e1e1
    // desc: filter by date
    fn filter_by_date() {
        let tf = TxnFilterTxnTSEnd {
            end: "2018-02-01T00:00:00+00:00".parse().unwrap(),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T00:00:00+00:00", true),
            ("2018-02-01T00:00:00+00:00", false),
            ("2018-03-01T00:00:00+00:00", false),
        ];

        for t in &cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }

        // test: 3d3d06cb-fcf2-422a-b57d-1bdb08f7d65e
        // desc: TxnFilter::TxnFilterTxnTSEnd
        let filt = TxnFilter::TxnFilterTxnTSEnd(tf);
        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(filt.eval(&txn), t.1);
        }
    }

    #[test]
    // test: 4e566d2b-da32-4336-9b7f-d7c4a59658d2
    // desc: filter by time
    fn filter_by_time() {
        let tf = TxnFilterTxnTSEnd {
            end: "2018-01-01T23:00:00+00:00".parse().unwrap(),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T11:00:00+00:00", true),
            ("2018-01-01T23:00:00+00:00", false),
            ("2018-01-02T00:00:00+00:00", false),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: f6081a60-92a9-4051-85d7-c993e3cc03be
    // desc: filter by nanoseconds
    fn filter_by_nanosecond() {
        let tf = TxnFilterTxnTSEnd {
            end: "2018-01-01T14:00:00.123456788+00:00".parse().unwrap(),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T14:00:00.123456787+00:00", true),
            ("2018-01-01T14:00:00.123456788+00:00", false),
            ("2018-01-01T14:00:00.123456789+00:00", false),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: ab53df34-d22a-4256-9c4d-6d1ccf0ef32e
    // desc: filter by timezone
    fn filter_by_timezone() {
        let tf = TxnFilterTxnTSEnd {
            end: "2018-01-04T00:00:00+00:00".parse().unwrap(),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-04T09:00:00+10:00", true),
            ("2018-01-03T18:00:00-06:00", false),
            ("2018-01-04T00:00:00+00:00", false),
        ];

        for t in cases {
            let txn = make_ts_txn(rfc3339_to_zoned(t.0).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }
}
