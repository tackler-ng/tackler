/*
 * Tackler-NG 2020-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserTagsTests.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;


    #[test]
    // test: 4d364251-f578-4c00-8390-9d8b5feea90b
    // desc: "reject invalid tags metadata constructions"
    #[allow(clippy::too_many_lines)]
    fn err_tags_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2020-12-24
            | # tags: ,tuv
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ','"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tuv,
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"no viable alternative at input"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tuv,,
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ','"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tuv, ,
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ','"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tu v
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' '"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: :tuv
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input ':'"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tuv:
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input 'tuv'"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: tu::v
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 3",
          r"at input 'tu'"
        ),

        (indoc!(
           "|
            |2020-12-24
            | ; metadata must be first
            | # tags: t,us
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 4",
          r"at input ' #'"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: t,u,v
            | ; no comments between metadata
            | # uuid: ff692918-290e-4b45-b78e-dba45619eec2
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 5",
          r"at input ' #'"
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: t,u
            | # tags: v,x
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 4",
          r"at input ' "
        ),
        (indoc!(
           "|
            |2020-12-24
            | # location: geo:60,25
            | # tags: tuv
            | # location: geo:61,25
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 5",
          r"at input ' "
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: t,u
            | # location: geo:60,25
            | # tags: x,y
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 5",
          r"at input ' "
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

    #[test]
    // test: 32e2d33d-f357-4751-8286-605cee07ea78
    // desc: "reject duplicate tags in txn tags set"
    fn err_tags_semantics_dups() {
      let  perr_strings: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2023-01-29
            | # tags: a, b, c, a
            | e 1
            | a
            |
            |").strip_margin(),
          "duplicate",
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

     #[test]
     // test: df593f17-2c74-4657-8da9-afc9ba445755
     // desc: "accepts tags metadata"
     #[allow(clippy::too_many_lines)]
     fn ok_tags() {
       #[allow(clippy::type_complexity)]
       let  pok_strings: Vec<(String, i32, Vec<(&str, fn(&Transaction) -> String)>)> = vec![
        (indoc!(
           "|
            |2020-12-24
            | # location: geo:61,25
            | # uuid: 369d63de-7a3b-4a3f-a741-a592fad19b9f
            | # tags: a:b:c
            | a  1
            | e -1
            |
            |").strip_margin(),
          3, vec![
            ("369d63de-7a3b-4a3f-a741-a592fad19b9f", txn_uuid_to_string),
            ("geo:61,25", txn_geo_to_string),
            ("a:b:c", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a, b
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a, b", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a, b, c
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a, b, c", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a, b, c, d
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a, b, c, d", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a, b, c, d, e
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a, b, c, d, e", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: e, c, a:b, b, d
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("e, c, a:b, b, d", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a:b:c, d, e
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a:b:c, d, e", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a:b:c , d ,e
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a:b:c, d, e", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | #\t \t tags:\t \t a:b:c \t \t , \t \td\t \t , \t \t e \t \t \n\
            | a  1
            | e -1
            |
            |").strip_margin(),
          1, vec![
          ("a:b:c, d, e", txn_tags_to_string)]
        ),
      ];
         let mut count = 0;
         let ref_count = pok_strings.len();
         for t in pok_strings {
             let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
             //println!("{:#?}", &t.0);
             //println!("{:#?}", res);
             assert!(res.is_ok(), "Offending test vector item: {count}");
             let txn_data = res.unwrap(/*:test:*/);
             let txns = txn_data.get_all().unwrap(/*:test:*/);
             let txn: &Transaction = txns.txns[0];
             let validators = t.2;
             let mut val_count = 0;
             for v in validators {
                 let v_func = v.1;
                 let v_ref = v.0.to_string();
                 assert_eq!(v_func(txn), v_ref);
                 val_count += 1;
             }
             assert_eq!(val_count, t.1);
             count += 1;
         }
         assert_eq!(count, ref_count);
    }
