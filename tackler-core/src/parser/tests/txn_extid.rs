/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;


    #[test]
    // test:
    // desc:
    #[allow(clippy::too_many_lines)]
    fn err_txn_extid_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
          // test:
          // desc: detect missing ext-id data
          (indoc!(
           "|
            |2026-07-05
            | # ext-id:
            | e 1
            | a -1
            |
            |").strip_margin(),
           "line: 3",
           r"at input ' # uid'"
          ),
        (indoc!(
           "|
            |2026-07-05
            | # extid: misspelled
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' # uid'"
        ),
        (indoc!(
           "|
            |2026-07-05
            | #:ext-id: hello there
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #:'"
        ),
        (indoc!(
           "|
            |2026-07-05
            | #ext-id: hello there
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' #uuid'"
        ),
        (indoc!(
           "|
            |2026-07-05
            | # ext-id:: hello there
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r":"
        ),
        (indoc!(
           "|
            |22026-07-05
            | # ext-id hello there
            | e 1
            | a -1
            |
            |").strip_margin(),
          "line: 3",
          r"at input '"
        ),
        (indoc!(
           "|
            |2026-07-05
            | ;:ext-id: hello there
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
        assert_eq!(count, 7);
    }

    #[test]
    // test:
    // desc: ""
    fn ok_txn_extid_parse() {
      let  pok_strings: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01
            | # ext-id: 0e3f2e08-1789-47ed-b93b-1280994586ac
            | a  1
            | e -1
            |
            |").strip_margin(),
          "0e3f2e08-1789-47ed-b93b-1280994586ac"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #      ext-id:     hello there
             | a  1
             | e -1
             |
             |").strip_margin(),
          "hello there"
        ),
        (indoc!(
           "|
             |2017-01-01\t
             | #\t\text-id:\t\thello there
             | a  1
             | e -1
             |
             |").strip_margin(),
         "hello there"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #\t \text-id:\t \t hello there
             | a  1
             | e -1
             |
             |").strip_margin(),
         "hello there"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #\t \text-id:\t \t hello there\t \t \n\
             | a  1
             | e -1
             |
             |").strip_margin(),
         "hello there"
        ),
        (indoc!(
           "|
             |2017-01-01
             | #\t \text-id:\t \t hello\t \t there\t \t \n\
             | a  1
             | e -1
             |
             |").strip_margin(),
         "hello\t \t there"
        ),
      ];
      let mut count = 0;
      for t in pok_strings {
          let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
          assert!(res.is_ok(), "Offending test vector item: {count}");

          let txn_data = res.unwrap(/*:test:*/);
          let txns = txn_data.get_all().unwrap(/*:test:*/);
          let txn: &Transaction = txns.txns[0];
          let extid = txn.header.extid.as_ref().unwrap(/*:test:*/);
        assert_eq!(extid, t.1);
        count += 1;
      }
      assert_eq!(count, 6);
    }
