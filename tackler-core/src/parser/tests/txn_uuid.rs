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
    // test: 49f73bec-afd9-4bef-bf5b-f9439ab2ea47
    // desc: check invalid metadata:uuid constructs
    #[allow(clippy::too_many_lines)]
    fn err_txn_uuid_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
          // test: 4391990c-83f4-4ea2-8c25-78a87beae219
          // desc: detect missing uuid
          (indoc!(
           "|
            |2017-01-01
            | # uuid:
            | e 1
            | a -1
            |
            |").strip_margin(),
           "line: 3",
           r"at input ' # uid'"
          ),
        (indoc!(
           "|
            |2017-01-01
            | # uid: 2c01d889-c928-477b-bf53-55e19887d34b
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' # uid'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | #:uuid: 2c01d889-c928-477b-bf53-55e19887d34b
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #:'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | #uuid: 2c01d889-c928-477b-bf53-55e19887d34b
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #uuid'"
        ),
        (indoc!(
           "|
            |2017-01-01
            | # uuid:: 2c01d889-c928-477b-bf53-55e19887d34b
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r":"
        ),
        (indoc!(
           "|
            |2017-01-01
            | # uuid 2c01d889-c928-477b-bf53-55e19887d34b
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input '"
        ),
        (indoc!(
           "|
            |2017-01-01
            | ;:uuid: 688fca6a-86e2-4c9d-82a0-1384a386167f
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ';'"
        ),
          // test: 56042ba1-89ca-48da-a55a-d6fea2946c59
          // desc: notice invalid uuid 1
          // | # uuid: 77356f17-98c9-43c6b9a7-bfc7436b77c8
          (indoc!(
           "|
            |2017-01-01
            | # uuid: 77356f17-98c9-43c6b9a7-bfc7436b77c8
            | e 1
            | a -1
            |
            |").strip_margin(),
           "line: 3",
           r"at input ';'"
          ),
          // test: 08e6dcf3-29b2-44d7-8fb0-af3fc6d74e0c
          // desc: notice invalid uuid 2
          //
          // https://bugs.openjdk.java.net/browse/JDK-8159339
          // https://bugs.openjdk.java.net/browse/JDK-8165199
          // https://bugs.openjdk.java.net/browse/JDK-8216407
          (indoc!(
           "|
            |2017-01-01
            | # uuid: 694aaaaa39222-4d8b-4d0e-8204-50e2a0c8b664
            | e 1
            | a -1
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
        assert_eq!(count, 9);
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
