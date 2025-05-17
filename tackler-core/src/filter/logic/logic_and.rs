/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::logic::TxnFilterAND;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterAND {
    fn eval(&self, txn: &Transaction) -> bool {
        self.txn_filters.iter().all(|f| f.eval(txn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tackler_api::filters::{NullaryFALSE, NullaryTRUE, TxnFilter};

    #[test]
    #[allow(clippy::too_many_lines)]
    fn permutations() {
        let txn = Transaction::default();

        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 2bd7fa78-adda-4f35-93eb-9b602bb3667e
                // desc: AND(false, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 11d4409c-93e2-4670-b2d5-65073980ba2d
                // desc: AND(false, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 7635059e-1828-48f7-9799-5bb0d327f446
                // desc: AND(true, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: bd589c45-4c80-4ccd-9f2f-49caf964d2a5
                // desc: AND(true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 20cb5b36-d9fb-4c63-bd68-37394f2c0524
                // desc: AND(true, true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 80b9bcbc-1274-440b-8e63-4be23bc6caa2
                // desc: AND(false, true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: feb1a75c-cea8-40db-b4bf-ef4d59d49c9e
                // desc: AND(true, false, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 456c6b08-7e61-410b-8a36-c3c47d6355b0
                // desc: AND(true, true, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 87107bc2-3c6d-435c-ac05-9ddade8352be
                // desc: AND(AND(true,false), AND(true,true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // test: d7c618df-3840-4cb3-b703-0896168ab448
                // desc: AND(AND(true,true),  AND(true,false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // test: b48c2765-12a7-4679-82e9-263f023fe731
                // desc: AND(AND(true,true),  AND(true,true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
        ];

        let mut test_count = 0;
        let ref_count = filters.len();
        for tf in filters {
            assert_eq!(tf.0.eval(&txn), tf.1);
            test_count += 1;
        }
        assert_eq!(test_count, ref_count);
    }
}
