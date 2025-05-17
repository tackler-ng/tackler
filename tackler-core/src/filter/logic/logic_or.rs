/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::logic::TxnFilterOR;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterOR {
    fn eval(&self, txn: &Transaction) -> bool {
        self.txn_filters.iter().any(|f| f.eval(txn))
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
                // test: c6036b88-6032-4005-84d5-a9d29cc4b283
                // desc: OR(false, false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 0e03ed8a-23ad-48f1-af49-2b0967d573e3
                // desc: OR(false, true)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 9aefdc26-b4bc-4e42-b0a8-ea2aefec7cde
                // desc: OR(true, false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                true,
            ),
            (
                // test: ace886f3-a1cb-454e-9f7f-3c4c449a5ab2
                // desc: OR(true, true)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 99741d27-f4f1-4f2d-acee-925605c5b9ef
                // desc: OR(false, false, false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: a17735b6-6847-4eaa-b66e-1eb27c81f73a
                // desc: OR(true, false, false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 8b5afb02-b3f1-4b2b-a599-dda2f5b95884
                // desc: OR(false, true, false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 0666ff4f-88af-42af-b415-1b73658731c7
                // desc: OR(false, false, true)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // test: 4ca33e34-ee6d-4ba8-9bc2-3e5c1a98d5d0
                // desc: OR(OR(false,false), OR(true,false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: c2ea859a-1daa-4c9c-8bdf-278ce74dfc02
                // desc: OR(OR(true,false),  OR(false,false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: ce4115c4-9051-4e9e-9a95-71de92f61520
                // desc: OR(OR(false,false), OR(false,false)
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                false,
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
