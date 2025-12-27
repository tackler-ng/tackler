/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserAccountsTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use tackler_rs::IndocUtils;



    #[test]
    // test: c6584dc1-3a9d-4bb6-8619-0ced9c7c6a17
    // desc: "accept valid uncommon account names"
    fn ok_uncommon_accounts() {
      let  txns_str =
      indoc!(
         "|
          |2019-01-01
          | e 1
          | a
          |
          |2019-01-01
          | $¢£¤¥ 1
          | a
          |
          |2019-01-01
          | µ 1
          | a
          |
          |2019-01-01
          | ¼½¾⅐Ⅶ 1
          | a
          |
          |2019-01-01
          | ° 1
          | a
          |
          |2019-01-01
          | ¹²³⁴ 1
          | a
          |
          |").strip_margin();

      let res = parser::string_to_txns(&mut txns_str.as_ref(), &mut Settings::default());
      assert!(res.is_ok());
      assert_eq!(res.unwrap(/*:test:*/).len(), 6);
    }

    #[test]
    // test: 9c836932-718c-491d-8cf0-30e35a0d1533
    // desc: "reject invalid sub-account constructs"
    fn err_sub_accounts_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
          // perr: '::'
   (indoc!("|
            |2017-01-01
            | a::b  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input 'a'"
        ),
          // perr: :a
   (indoc!("|
            |2017-02-02
            | :a  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' :'"
        ),
          // perr: a:
   (indoc!("|
            |2017-03-03
            | a:  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input 'a'"
        ),
          // perr: '×' U+00D7
   (indoc!("|
            |2017-03-03
            | a×b  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input '×'"
        ),
          // perr: '÷' U+00F7
   (indoc!("|
            |2017-03-03
            | a÷b  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input '÷'"
        ),
          // perr: ';' U+037E
          #[allow(clippy::unicode_not_nfc)]
   (indoc!("|
            |2017-03-03
            | a;b  1
            | e
            |
            |").strip_margin(),
          "line: 3",
          r"at input ';'"
        ),
      ];
        let mut count = 0;
        let should_be_count = perr_strings.len();
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
        assert_eq!(count, should_be_count);
    }

    //
    // "Numeric accounts names"
    //
    #[test]
    // test: 385f7a60-9618-40e4-9f3e-8e28c76a8872
    // desc: "check invalid numeric top-account names"
    fn err_numeric_accounts_parse_1() {
        let  perr_strings:Vec<(String,)> = vec![
 (indoc!("|
          |2019-03-14
          | 0a 1
          | s
          |
          |").strip_margin(),
     ),
 (indoc!("|
          |2019-03-14
          | 0 1
          | s
          |
          |").strip_margin(),
 ),

 (indoc!("|
          |2019-03-14
          | 0:0 1
          | s
          |
          |").strip_margin(),
     ),
 (indoc!("|
          |2019-03-14
          | _0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | _0:a 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | ·0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | ·0:a 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | -0 1
          | s
          |
          |").strip_margin(),
      ),
        ];
        let mut count = 0;
        let should_be_count = perr_strings.len();
        for t in perr_strings {
            let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
            assert!(res.is_err(),
                    "Testing Error: Offending test vector item: {count}");
            /*
            // todo: parser error messages, error position
            assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"),
                    "Testing Line: Offending test vector item: {}", count);
            */
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }

    #[test]
    // test: b160ec62-6254-45c8-ac3c-ef0ee41c95b1
    // desc: "reject invalid numeric sub-account names"
    fn err_numeric_accounts_parse_2() {
        let  perr_strings:Vec<(String,)> = vec![
 (indoc!("|
          |2019-03-14
          | a:0.0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:0,0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:-0:a 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:_0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:_0:a 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:·0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:·0:a 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:-0 1
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a:-0:a 1
          | s
          |
          |").strip_margin(),
      ),
            ];
        let mut count = 0;
        let should_be_count = perr_strings.len();
        for t in perr_strings {
            let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
            assert!(res.is_err(),
                    "Testing Error: Offending test vector item: {count}");
            /*
            // todo: parser error messages, error position
            assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"),
                    "Testing Line: Offending test vector item: {}", count);
            */
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }


    #[test]
    // test: 78a4af97-a876-4a13-9d67-b7e0ef86ed44
    // desc: "reject invalid commodity names"
    fn err_commodities_parse() {
        let  perr_strings:Vec<(String,)> = vec![
 (indoc!("|
          |2019-03-14
          | a 1 0coin
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 0000
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 a0.000
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 a0,000
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 au:oz
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 _0
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 ·0
          | s
          |
          |").strip_margin(),
 ),
 (indoc!("|
          |2019-03-14
          | a 1 -0
          | s
          |
          |").strip_margin(),
      ),
            ];
        let mut count = 0;
        let should_be_count = perr_strings.len();
        for t in perr_strings {
            let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
            assert!(res.is_err(),
                    "Testing Error: Offending test vector item: {count}");
            /*
            // todo: parser error messages, error position
            assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"),
                    "Testing Line: Offending test vector item: {}", count);
            */
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }
