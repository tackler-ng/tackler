/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserHeaderTimestampTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::model::Transaction;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;



    #[test]
    // test: 4ff959f7-c2bd-4750-8664-f46ce50a7c7b
    // desc: "check invalid timestamp constructs"
    #[allow(clippy::too_many_lines)]
    fn err_timestamp_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![

        (indoc!(
           "|
            |2017
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017'"
        ),
        (indoc!(
           "|
            |2017-1
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-1'"
        ),
        (indoc!(
           "|
            |2017-01
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01'"
        ),
        (indoc!(
          "|
            |2017-1-1
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-1-1'"
        ),
        (indoc!(
           "|
            |2017-01-1
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-1'"
        ),
        (indoc!(
           "|
            |2017-01-01+0200
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '+'"
        ),
        (indoc!(
           "|
            |2017-01-01T14+02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-01T14'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00+02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-01T14'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00:00+0200
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '+'"
        ),
        (indoc!(
           "|
            |2017-01-01-04:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-01-04'"
        ),
        (indoc!(
           "|
            |2017-01-01T14-04:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-01T14-04'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00-04:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '2017-01-01T14'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00:00-0400
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input '-0400'"
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
        assert_eq!(count, 13);
    }

    #[test]
    // test: 294a4d37-2911-4c0f-9024-0c79bf3c99ba
    // desc: "check invalid timestamp constructs with format v2"
    fn err_ts_format_v2_parse() {
    let  perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 Z
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' Z'"
        ),
        (indoc!(
           "|
            |2017-01-01 +02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' +'"
        ),
        (indoc!(
           "|
            |2017-01-01 -04:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' -04'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00:00 Z
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' Z'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00:00 +02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          "at input ' +'"
        ),
        (indoc!(
           "|
            |2017-01-01T14:00:00 -04:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' -04'"
        ),
        (indoc!(
           "|
            |2017-01-01 T 14:00:00+02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' T'"
        ),
        (indoc!(
           "|
            |2017-01-01 T 14:00:00 +02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' T'"
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
        assert_eq!(count, 8);
    }

    #[test]
    // test: 2c0ee1a2-1a23-4427-a6dc-6156abc36272
    // desc: "accept valid timestamp constructs"
    #[allow(clippy::too_many_lines)]
    fn ok_timestamp() {
      let pok_strings: Vec<(String, &str)> = vec![

        (indoc!(
           "|
            |2017-06-24
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T00:00:00+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T00:00:00+00:00"
        ),
        (indoc!(
          "|
             |2017-06-24\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T00:00:00+00:00"
        ),
        (indoc!(
           "|
            |2017-06-24T14:01:02
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
        (indoc!(
           "|
            |2017-06-24T14:01:02Z
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02Z   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02Z\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02+00:00"
        ),
            // 10
        (indoc!(
           "|
            |2017-06-24T14:01:10+02:00
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:10+02:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10+02:00   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10+02:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10+02:00\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10+02:00"
        ),

        (indoc!(
           "|
            |2017-06-24T14:01:10-04:00
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:10-04:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10-04:00   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10-04:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10-04:00\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10-04:00"
        ),

        /*
         * NANO SECOND
         */
         (indoc!(
            "|
             |2017-06-24T14:01:02.123456789
             | a 1
             | e -1
             |
             |").strip_margin(),
           "2017-06-24T14:01:02.123456789+00:00"
         ),
         (indoc!(
             "|
              |2017-06-24T14:01:02.123456789   \n\
              | a 1
              | e -1
              |
              |").strip_margin(),
           "2017-06-24T14:01:02.123456789+00:00"
         ),
        (indoc!(
            "|
             |2017-06-24T14:01:02.123456789\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02.123456789+00:00"
        ),
        (indoc!(
           "|
            |2017-06-24T14:01:02.123456789Z
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:02.123456789+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02.123456789Z   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02.123456789+00:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:02.123456789Z \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:02.123456789+00:00"
        ),
        (indoc!(
           "|
            |2017-06-24T14:01:10.123456789+02:00
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:10.123456789+02:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10.123456789+02:00   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10.123456789+02:00"
        ),
        (indoc!(
            "|
             |2017-06-24T14:01:10.123456789+02:00\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10.123456789+02:00"
        ),

        (indoc!(
           "|
            |2017-06-24T14:01:10.123456789-04:00
            | a 1
            | e -1
            |
            |").strip_margin(),
          "2017-06-24T14:01:10.123456789-04:00"
        ),
        (indoc!(
           "|
             |2017-06-24T14:01:10.123456789-04:00   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10.123456789-04:00"
        ),
        (indoc!(
           "|
             |2017-06-24T14:01:10.123456789-04:00\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "2017-06-24T14:01:10.123456789-04:00"
        ),

      ];

        let mut count = 0;
        for t in pok_strings {
            let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
            assert!(res.is_ok(), "is it ok: Offending test vector item: {count}");
            let txn_data = res.unwrap(/*:test:*/);
            let txns = txn_data.get_all().unwrap(/*:test:*/);
            let txn: &Transaction = txns.txns[0];
            assert_eq!(txn_ts_to_string(txn), t.1.to_string(), "Testing value: offending test vector item: {count}");
            count += 1;
        }
        assert_eq!(count, 27);
    }

