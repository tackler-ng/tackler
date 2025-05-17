/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserUUIDTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;



    #[test]
    //desc: "check invalid metadata:uuid constructs"
    #[allow(non_snake_case)]
    fn id_49f73bec_afd9_4bef_bf5b_f9439ab2ea47__err_txn_uuid_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01
            | # uid: 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' # uid'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | #:uuid: 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #:'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | #uuid: 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #uuid'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | # uuid:: 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r":"
        ),
        (indoc!(
           "|
            |2017-01-01
            | # uuid 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input '"
        ),
        (indoc!(
           "|
            |2017-01-01
            | ;:uuid: 688fca6a-86e2-4c9d-82a0-1384a386167f
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ';'"
        ),
      ];
        let mut count = 0;
        for t in perr_strings {
            let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
            assert!(res.is_err(),
                    "Testing Error: Offending test vector item: {count}");
            /*
            // todo: parser error messages, error position
            assert!(res.err().unwrap(/*:test:*/).to_string().contains(t.1),
                    "Testing Line: Offending test vector item: {}", count);
            */
            count += 1;
        }
        assert_eq!(count, 6);
    }

    #[test]
    //desc: "accept valid metadata txn::uuid constructs"
    #[allow(non_snake_case)]
    fn id_546e4368_dcfa_44d5_a21d_13f3b8bf51b6__ok_txn_uuid() {
      let  pok_strings: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01
            | # uuid: 0e3f2e08-1789-47ed-b93b-1280994586ac
            | a  1
            | e -1
            |
            |").strip_margin(),
          "0e3f2e08-1789-47ed-b93b-1280994586ac"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #      uuid:     52c319c4-fb42-4a81-bdce-95979b602ba0
             | a  1
             | e -1
             |
             |").strip_margin(),
          "52c319c4-fb42-4a81-bdce-95979b602ba0"
        ),
        (indoc!(
           "|
             |2017-01-01\t
             | #\t\tuuid:\t\t3e75fa97-4be9-4955-acb9-6349223d4cbc
             | a  1
             | e -1
             |
             |").strip_margin(),
          "3e75fa97-4be9-4955-acb9-6349223d4cbc"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #\t \tuuid:\t \t fec05984-b8a6-439d-8bb0-0ac6461fba8e
             | a  1
             | e -1
             |
             |").strip_margin(),
          "fec05984-b8a6-439d-8bb0-0ac6461fba8e"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #\t \tuuid:\t \t 4c5bab64-edf9-4972-bce6-09cdd666f89d\t \t \n\
             | a  1
             | e -1
             |
             |").strip_margin(),
          "4c5bab64-edf9-4972-bce6-09cdd666f89d"
        ),
      ];
      let mut count = 0;
      for t in pok_strings {
        let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
        assert!(res.is_ok(), "Offending test vector item: {count}");
          let txn_data = res.unwrap(/*:test:*/);
          let txns = txn_data.get_all().unwrap(/*:test:*/);
        let txn: &Transaction = txns.txns[0];
        assert_eq!(txn_uuid_to_string(txn), t.1.to_string());
        count += 1;
      }
      assert_eq!(count, 5);
    }
