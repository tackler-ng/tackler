/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::Predicate;
use tackler_api::filters::TxnFilter;

use crate::model::Transaction;
mod filter_definition;
mod logic;
mod nullary_false;
mod nullary_true;
mod posting;
mod txn;

/// Actual filtering implementation for [`TxnFilter`]
///
/// [`TxnFilter`]: ../tackler_api/filters/index.html
impl Predicate<Transaction> for TxnFilter {
    fn eval(&self, txn: &Transaction) -> bool {
        match self {
            // special nullary filters
            TxnFilter::NullaryTRUE(tf) => tf.eval(txn),
            TxnFilter::NullaryFALSE(tf) => tf.eval(txn),

            // logic filters
            TxnFilter::TxnFilterAND(tf) => tf.eval(txn),
            TxnFilter::TxnFilterOR(tf) => tf.eval(txn),
            TxnFilter::TxnFilterNOT(tf) => tf.eval(txn),

            // txn header filters
            TxnFilter::TxnFilterTxnTSBegin(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnTSEnd(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnCode(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnDescription(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnUUID(tf) => tf.eval(txn),
            TxnFilter::TxnFilterBBoxLatLon(tf) => tf.eval(txn),
            TxnFilter::TxnFilterBBoxLatLonAlt(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnTags(tf) => tf.eval(txn),
            TxnFilter::TxnFilterTxnComments(tf) => tf.eval(txn),

            // txn posting filters
            TxnFilter::TxnFilterPostingAccount(tf) => tf.eval(txn),
            TxnFilter::TxnFilterPostingComment(tf) => tf.eval(txn),
            TxnFilter::TxnFilterPostingAmountEqual(tf) => tf.eval(txn),
            TxnFilter::TxnFilterPostingAmountLess(tf) => tf.eval(txn),
            TxnFilter::TxnFilterPostingAmountGreater(tf) => tf.eval(txn),
            TxnFilter::TxnFilterPostingCommodity(tf) => tf.eval(txn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Predicate;
    use crate::model::TxnAccount;
    use crate::model::{AccountTreeNode, Commodity, Posting};
    use jiff::Zoned;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use tackler_api::filters::{
        NullaryFALSE, NullaryTRUE, TxnFilter, logic::TxnFilterAND, logic::TxnFilterNOT,
        logic::TxnFilterOR,
    };
    use tackler_api::location::GeoPoint;
    use tackler_api::txn_header::TxnHeader;
    use uuid::Uuid;

    pub(crate) fn make_ts_txn(ts: Zoned) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: ts,
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_default_txn(_str: Option<&str>) -> Transaction {
        Transaction::default()
    }

    pub(crate) fn make_default_v_txn(_str: Option<Vec<&str>>) -> Transaction {
        Transaction::default()
    }

    pub(crate) fn make_code_txn(code: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: code.map(str::to_string),
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_desc_txn(desc: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: desc.map(str::to_string),
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_uuid_txn(uuid: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: None,
                uuid: uuid.map(|uuid_str| Uuid::parse_str(uuid_str).unwrap(/*:test:*/)),
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_geo_txn(lat: Decimal, lon: Decimal, alt: Option<Decimal>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: None,
                uuid: None,
                location: GeoPoint::from(lat, lon, alt).ok(),
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_tags_txn(tags: Option<Vec<&str>>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: tags.map(|tags| tags.iter().map(|t| Arc::new(str::to_string(*t))).collect()),
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_comments_txn(cmts: Option<Vec<&str>>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: cmts
                    .map(|comments| comments.iter().map(|t| str::to_string(*t)).collect()),
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_posts_txn(e: &str, e_value: i64, a: &str) -> Transaction {
        let e_v = Decimal::new(e_value, 0);
        let e_acctn = Arc::new(AccountTreeNode::from(e).unwrap(/*:test:*/));
        let e_txntn = TxnAccount {
            atn: e_acctn,
            comm: Arc::new(Commodity::default()),
        };

        let e_p = Posting::from(e_txntn, e_v, e_v, false, Arc::new(Commodity::default()), Some("comment".to_string())).unwrap(/*:test:*/);

        let a_v = Decimal::new(-e_value, 0);
        let a_acctn = Arc::new(AccountTreeNode::from(a).unwrap(/*:test:*/));
        let a_txntn = TxnAccount {
            atn: a_acctn,
            comm: Arc::new(Commodity::default()),
        };
        let a_p = Posting::from(a_txntn, a_v, a_v, false, Arc::new(Commodity::default()), Some("comment".to_string())).unwrap(/*:test:*/);

        Transaction::try_from(
            TxnHeader {
                timestamp: Zoned::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            vec![e_p, a_p],
        )
        .unwrap(/*:test:*/)
    }

    pub(crate) fn make_posts_commodity_txn(
        c: Option<&str>,
        a: &str,
        a_value: i64,
        e: &str,
    ) -> Transaction {
        fn make_commodity(c: Option<&str>) -> Arc<Commodity> {
            match c {
                Some(name) => Arc::new(Commodity::from(name.to_string()).unwrap(/*:test:*/)),
                None => Arc::new(Commodity::default()),
            }
        }

        let e_v = Decimal::new(a_value, 0);
        let e_acctn = Arc::new(AccountTreeNode::from(e).unwrap(/*:test:*/));
        let e_txntn = TxnAccount {
            atn: e_acctn,
            comm: make_commodity(c),
        };
        let e_p = Posting::from(
            e_txntn,
            e_v,
            e_v,
            false,
            make_commodity(Some("txn_comm")),
            None,
        )
        .unwrap(/*:test:*/);

        let a_v = Decimal::new(-a_value, 0);
        let a_acctn = Arc::new(AccountTreeNode::from(a).unwrap(/*:test:*/));
        let a_txntn = TxnAccount {
            atn: a_acctn,
            comm: make_commodity(c),
        };
        let a_p = Posting::from(
            a_txntn,
            a_v,
            a_v,
            false,
            make_commodity(Some("txn_comm")),
            None,
        )
        .unwrap(/*:test:*/);

        Transaction::try_from(TxnHeader::default(), vec![e_p, a_p]).unwrap(/*:test:*/)
    }

    #[test]
    fn complex_and() {
        let txn = Transaction::default();
        //
        // AND(AND(...) style tests are done by logic_and::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 54cbd549-5567-4b19-bc20-a3de146fff40
                // desc: "AND(filter, AND(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // test: 6e544624-ad3e-4920-9946-7eaf94febfb5
                // desc: "AND(filter, OR(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: ef81d4c1-9d5e-47f2-ab7c-646fbc49e268
                // desc: "AND(filter, NOT(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: b2e5d857-e02c-4313-9ca7-9aa765033343
                // desc: "AND(AND(...), OR(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: dab44c95-834c-438a-8543-a73547284f03
                // desc: "AND(filter, AND(...), OR(...), NOT(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
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

    #[test]
    fn complex_or() {
        let txn = Transaction::default();
        //
        // OR(OR(...) style tests are done by logic_or::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: b75466f3-f7bf-4e7f-9865-e2937a5d968d
                // desc: "OR(filter, AND(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
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
            (
                // test: 9029ad79-bbea-4c0c-a0e0-09c8b1b04188
                // desc: "OR(filter, OR(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: b01bfc0d-0f6d-409b-8101-4647c70d1409
                // desc: "OR(filter, NOT(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: e8c40011-4aef-4639-98e2-1362a0961db8
                // desc: "OR(AND(...), OR(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
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
                // test: 4b127707-c83b-418b-9703-849ee304a19c
                // desc: "OR(filter, AND(...), OR(...), NOT(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryTRUE(NullaryTRUE {})),
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

    #[test]
    fn complex_not() {
        let txn = Transaction::default();
        //
        // NOT(NOT(...) style tests are done by logic_not::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 3e03d091-4f06-44d3-8bf5-285c85178ff9
                // desc: "NOT(OR(...))"
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterOR(TxnFilterOR {
                        txn_filters: vec![
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    })),
                }),
                false,
            ),
            (
                // test: 0c549c6e-f3b7-4614-b874-31db1110c41c
                // desc: "NOT(AND(...))"
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    })),
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
