/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserHeaderCodeTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::model::Transaction;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;


    #[test]
    //desc: "check invalid header code constructs"
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_lines)]
    fn id_242aa119_bc5e_4562_9f4a_5feb26d1fba6__err_code_parse() {
      let perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 (123
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (123))
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ')'"
        ),
        (indoc!(
           "|
            |2017-01-01 ((123))
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (123)abc
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input 'abc'"
        ),
        (indoc!(
           "|
            |2017-01-01 (123)a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input 'a'"
        ),
        (indoc!(
           "|
            |2017-01-01 (a'a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a[a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a]a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a{a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a}a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a<a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 (a>a)
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),

        (indoc!(
           "|
            |2017-01-01 ( ' )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( [ )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( ] )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( { )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( } )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( < )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( > )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),


        (indoc!(
           "|
            |2017-01-01 ( [a] )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( {a} )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2017-01-01 ( <a> )
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r"at input ' '"
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
        assert_eq!(count, 22);
    }

    #[test]
    //desc: "accept valid code constructs"
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_lines)]
    fn id_a5450ec6_42a3_4f3b_b989_27eb2949ccad__ok_code() {
      let pok_strings: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 (abc)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01  (abc)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
            "|
             |2017-01-01\t(abc)
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
            "|
             |2017-01-01\t \t (abc)
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
             |2017-01-01 (abc)  \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
             |2017-01-01 (abc)\t \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01 (a c)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "a c"
        ),
        (indoc!(
           "|
            |2017-01-01 ()
            | a 1
            | e -1
            |
            |").strip_margin(),
          ""
        ),
        (indoc!(
           "|
             |2017-01-01 (\t \t )
             | a 1
             | e -1
             |
             |").strip_margin(),
          ""
        ),
        (indoc!(
           "|
            |2017-01-01 ( )
            | a 1
            | e -1
            |
            |").strip_margin(),
          ""
        ),
        (indoc!(
           "|
            |2017-01-01 (!)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "!"
        ),
        (indoc!(
           "|
            |2017-01-01 (*)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "*"
        ),
        (indoc!(
           "|
             |2017-01-01 \t \t   (123)
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
             |2017-01-01 \t \t   (123) \t \t   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
            |2017-01-01 (abc)
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
             |2017-01-01 (\t \t123)
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
             |2017-01-01 (123\t \t )
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
             |2017-01-01 (\t \t123)
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
             |2017-01-01 (\t \t 123\t \t )
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123"
        ),

      ];
      let mut count = 0;
      for t in pok_strings {
        let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
        assert!(res.is_ok(), "Offending test vector item: {count}");
          let txn_data = res.unwrap(/*:test:*/);
          let txns = txn_data.get_all().unwrap(/*:test:*/);
        let txn: &Transaction = txns.txns[0];
        assert_eq!(txn_code_to_string(txn), t.1.to_string());
        count += 1;
      }
      assert_eq!(count, 19);
    }
