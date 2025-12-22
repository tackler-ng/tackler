/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

mod txns_checksum {
    use indoc::{formatdoc, indoc};
    use tackler_api::filters::logic::TxnFilterAND;
    use tackler_api::filters::txn::{TxnFilterTxnTSBegin, TxnFilterTxnTSEnd};
    use tackler_api::filters::{FilterDefinition, TxnFilter};
    use tackler_api::metadata::items::MetadataItem;
    use tackler_core::kernel::Settings;
    use tackler_core::model::TxnData;
    use tackler_core::{parser, tackler};
    use tackler_rs::IndocUtils;

    const UUID_01: &str = "72f7b85b-42ce-4fa2-971e-5ba5fc196d9d";
    const UUID_02: &str = "3A9A2AE9-7AA3-4556-848A-40F5B5E52BE6";
    const UUID_03: &str = "dd3bf34b-78e9-4a80-9072-8503c462f7c3";

    const TXN_SET_ALL_CHECKSUM: &str =
        "e0bac92d09748d4b98e8d4c4419c6f0b1f628cfec95728e4fc7f81f67f3db3f1";

    const TXN_02_CHECKSUM: &str =
        "f561963b1dd6941054145e47e8e02720a323d98a8a9f2ebc910586469f5e26c7";

    fn verify_checksum(result: &Result<TxnData, tackler::Error>, checksum: &str) {
        match result {
            Ok(txn_data) => {
                let txn_set = txn_data.get_all().unwrap(/*:test:*/);
                match txn_set.metadata() {
                    Some(md) => {
                        assert_eq!(md.items.len(), 1, "Metadata Item count is wrong");
                        match &md.items[0] {
                            MetadataItem::TxnSetChecksum(tscsmd) => {
                                assert_eq!(tscsmd.hash.value, checksum);
                            }
                            _ => {
                                panic!(
                                    /*:test:*/
                                    "The first item is not Txn Set Checksum Metadata item"
                                )
                            }
                        }
                    }
                    None => {
                        panic!(/*:test:*/ "no metadata")
                    }
                }
            }
            Err(err) => {
                panic!(/*:test:*/ "{err:#?}");
            }
        }
    }

    #[test]
    // test: cc98b4da-aa26-44e1-ba76-ca702a99add6
    // desc: detects missing uuid from existing txns
    fn txns_missing_uuid() {
        #[rustfmt::skip]
        let str_uuids = formatdoc!(
            "2019-01-01 'txn01
            | # uuid: {UUID_01}
            | e  1
            | a
            |
            |2019-02-01 'txn02
            | # uuid: {UUID_02}
            | e  1
            | a
            |
            |2019-03-01 'txn03
            | # uuid: {UUID_03}
            | e  1
            | a
            |
            |"
        ).strip_margin();

        #[rustfmt::skip]
        let str_no_uuid = indoc!(
            "2019-04-01 'txn04
            | e  1
            | a
            |"
        ).strip_margin();

        let mut txns_audit = parser::string_to_txns(
            &mut str_uuids.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);

        let mut txns_plain = parser::string_to_txns(
            &mut str_no_uuid.as_str(), &mut Settings::default()).unwrap(/*:test:*/);

        let err = txns_audit.append(&mut txns_plain);
        let err_msg = err.expect_err("test case went wonky").to_string();

        assert!(err_msg.contains("without UUID"));
        assert!(err_msg.contains("checksum"));
    }

    #[test]
    // test: 0a31ea4f-cb4c-4b5a-8ea4-1786feeb32a4
    // desc: detects duplicate uuid from existing txns
    fn txns_duplicate_uuid() {
        #[rustfmt::skip]
        let str_uuids = formatdoc!(
            "2019-01-01 'txn01
            | # uuid: {UUID_01}
            | e  1
            | a
            |
            |2019-02-01 'txn02
            | # uuid: {UUID_02}
            | e  1
            | a
            |
            |2019-03-01 'txn03
            | # uuid: {UUID_03}
            | e  1
            | a
            |
            |"
        ).strip_margin();

        #[rustfmt::skip]
        let str_duplicate_uuid = formatdoc!(
            "2019-04-01 'txn04
            | # uuid: {UUID_02}
            | e  1
            | a
            |"
        ).strip_margin();

        let mut txns_audit = parser::string_to_txns(
            &mut str_uuids.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);

        let mut txns_dup = parser::string_to_txns(
            &mut str_duplicate_uuid.as_str(), &mut Settings::default()).unwrap(/*:test:*/);

        let err = txns_audit.append(&mut txns_dup);
        let err_msg = err.expect_err("test case went wonky").to_string();

        assert!(err_msg.contains("Found 1 duplicate"));
        assert!(err_msg.contains("3a9a2ae9-7aa3-4556-848a-40f5b5e52be6"));
    }

    #[test]
    // test: fed114d9-5b82-4821-81a4-38782f927d74
    // desc: Checksum when appending TxnSets; Upper case UUID
    fn txns_append_checksum() {
        fn all_txns() -> Result<TxnData, tackler::Error> {
            #[rustfmt::skip]
            let str_txn_01 = formatdoc!(
                "2019-01-01 'txn01
                | # uuid: {UUID_01}
                | e  1
                | a
                |"
            ).strip_margin();

            #[rustfmt::skip]
            let str_txn_02 = formatdoc!(
                "2019-02-01 'txn02
                | # uuid: {UUID_02}
                | e  1
                | a
                |"
            ).strip_margin();

            #[rustfmt::skip]
            let str_txn_03 = formatdoc!(
                "2019-03-01 'txn03
                | # uuid: {UUID_03}
                | e  1
                | a
                |"
            ).strip_margin();

            let mut txns_01 = parser::string_to_txns(
                &mut str_txn_01.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);
            let mut txns_02 = parser::string_to_txns(
                &mut str_txn_02.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);
            let mut txns_03 = parser::string_to_txns(
                &mut str_txn_03.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);

            txns_01.append(&mut txns_02)?.append(&mut txns_03)?;
            Ok(txns_01)
        }

        let txns = all_txns();

        verify_checksum(&txns, TXN_SET_ALL_CHECKSUM);
    }

    #[test]
    // test: 0e76295f-aee6-47bc-ae6f-7fba5ce6d818
    // desc: Checksum with filters
    fn txns_checksum_with_filters() {
        #[rustfmt::skip]
        let str_uuids = formatdoc!(
            "2019-01-01 'txn01
            | # uuid: {UUID_01}
            | e  1
            | a
            |
            |2019-02-01 'txn02
            | # uuid: {UUID_02}
            | e  1
            | a
            |
            |2019-03-01 'txn03
            | # uuid: {UUID_03}
            | e  1
            | a
            |"
        ).strip_margin();

        let txns_all = parser::string_to_txns(
            &mut str_uuids.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);
        //verify_checksum(&Ok(txns_all), TXN_SET_ALL_CHECKSUM);

        let ts_begin = TxnFilterTxnTSBegin {
            begin: "2019-02-01 00:00:00Z".parse().unwrap(/*:test:*/),
        };
        let ts_end = TxnFilterTxnTSEnd {
            end: "2019-03-01 00:00:00Z".parse().unwrap(/*:test:*/),
        };

        let span = TxnFilterAND {
            txn_filters: vec![
                TxnFilter::TxnFilterTxnTSBegin(ts_begin),
                TxnFilter::TxnFilterTxnTSEnd(ts_end),
            ],
        };

        let filter = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(span),
        };

        let txn_set = txns_all.filter(&filter).unwrap(/*:test:*/);
        match txn_set.metadata() {
            Some(md) => {
                assert_eq!(md.items.len(), 2, "Metadata Item count is wrong");
                match &md.items[0] {
                    MetadataItem::TxnSetChecksum(tscsmd) => {
                        assert_eq!(tscsmd.hash.value, TXN_02_CHECKSUM);
                    }
                    _ => {
                        panic!(
                            /*:test:*/
                            "The first item is not Txn Set Checksum Metadata item"
                        )
                    }
                }
                match &md.items[1] {
                    MetadataItem::TxnFilterDescription(_) => {}
                    _ => {
                        panic!(
                            /*:test:*/
                            "The first item is not Txn Set Checksum Metadata item"
                        )
                    }
                }
            }
            None => {
                panic!(/*:test:*/ "no metadata")
            }
        }
    }
    #[test]
    // test: 283d64f6-4508-48ac-89a3-e70e25784330
    // desc: decode working filter from JSON
    fn txn_checksum_with_json_filters() {
        #[rustfmt::skip]
        let str_uuids = formatdoc!(
            "2019-01-01 'txn01
            | # uuid: {UUID_01}
            | e  1
            | a
            |
            |2019-02-01 'txn02
            | # uuid: {UUID_02}
            | e  1
            | a
            |
            |2019-03-01 'txn03
            | # uuid: {UUID_03}
            | e  1
            | a
            |"
        ).strip_margin();

        let txns_all = parser::string_to_txns(
            &mut str_uuids.as_str(), &mut Settings::default_audit()).unwrap(/*:test:*/);

        let filter_json_str = r#"{"txnFilter":{"TxnFilterAND":{"txnFilters":[
            {"TxnFilterTxnTSBegin":{"begin":"2019-02-01T00:00:00Z"}},
            {"TxnFilterTxnTSEnd":{"end":"2019-03-01T00:00:00Z"}}
        ]}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let filter = tf_res.unwrap(/*:test:*/);

        let txn_set = txns_all.filter(&filter).unwrap(/*:test:*/);
        match txn_set.metadata() {
            Some(md) => {
                assert_eq!(md.items.len(), 2, "Metadata Item count is wrong");
                match &md.items[0] {
                    MetadataItem::TxnSetChecksum(tscsmd) => {
                        assert_eq!(tscsmd.hash.value, TXN_02_CHECKSUM);
                    }
                    _ => {
                        panic!(
                            /*:test:*/
                            "The first item is not Txn Set Checksum Metadata item"
                        )
                    }
                }
                match &md.items[1] {
                    MetadataItem::TxnFilterDescription(_) => {}
                    _ => {
                        panic!(
                            /*:test:*/
                            "The first item is not Txn Set Checksum Metadata item"
                        )
                    }
                }
            }
            None => {
                panic!(/*:test:*/ "no metadata")
            }
        }
    }
}
