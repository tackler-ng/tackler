/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserMetadataTests.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;



      #[test]
      // test: b88d6733-2acf-4021-a3d7-deaf58b518a6
      // desc: "reject invalid metadata constructions"
      #[allow(clippy::too_many_lines)]
      fn err_metadata_parse() {
        let  perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2019-05-01
            | ; metadata must be first
            | # uuid: 2c01d889-c928-477b-bf53-55e19887d34b
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 4",
          r" input ' #'"
        ),
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:60,25
            | ; no comments between metadata
            | # uuid: f0cf7f01-4af9-41b9-82ae-9601d9e05186
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 5",
          r" input ' #'"
        ),
        (indoc!(
           "|
            |2019-05-01
            | # uuid: ff46c6d0-c42f-4dfd-a176-beabe95d84a2
            | # uuid: e1bbad16-05ef-4366-8adc-8717b3bb5f38
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 4",
          r" input ' "
        ),
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:60,25
            | # location: geo:61,25
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 4",
          r" input ' "
        ),
        (indoc!(
           "|
            |2024-12-25
            | # tags: tuv
            | # tags: tuv
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 4",
          r" input ' "
        ),
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:60,25
            | # uuid: ea23a28b-a99e-4af4-8f87-c011d606efd7
            | # location: geo:61,25
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 5",
          r" input ' "
        ),
        (indoc!(
           "|
            |2019-05-01
            | # uuid: 5e6ab503-b85b-48ba-bc49-8ed0db2a2ce1
            | # location: geo:60,25
            | # uuid: 552ac798-5807-4875-b64a-e63d02c255d0
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 5",
          r" input ' "
        ),
        (indoc!(
           "|
            |2019-05-01
            | # tags: tuv
            | # location: geo:60,25
            | # tags: tuv
            | a  1
            | e -1
            |
            |").strip_margin(),
          "line: 5",
          r" input ' "
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
    // test: 5bb95c2e-2fad-4584-9380-e6cafe732cf6
    // desc: "accepts multiple metadata items"
    fn ok_metadata_multiple_items() {
      #[allow(clippy::type_complexity)]
      let  pok_strings: Vec<(String, i32, Vec<(&str, fn(&Transaction) -> String)>)> = vec![
        (indoc!(
           "|
            |2019-05-01
            | # uuid: 68ddc754-40ad-4d73-824c-17e75e59c731
            | # location: geo:60,25
            | a  1
            | e -1
            |
            |").strip_margin(),
          2, vec![
          ("68ddc754-40ad-4d73-824c-17e75e59c731", txn_uuid_to_string),
          ("geo:60,25", txn_geo_to_string)]
        ),
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:61,25
            | # uuid: c075a1a4-37d5-4d79-a92b-5cbb323519f0
            | a  1
            | e -1
            |
            |").strip_margin(),
          2, vec![
          ("c075a1a4-37d5-4d79-a92b-5cbb323519f0", txn_uuid_to_string),
          ("geo:61,25", txn_geo_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # location: geo:61,25
            | # uuid: c075a1a4-37d5-4d79-a92b-5cbb323519f0
            | # tags: a:b:c
            | a  1
            | e -1
            |
            |").strip_margin(),
          3, vec![
          ("c075a1a4-37d5-4d79-a92b-5cbb323519f0", txn_uuid_to_string),
          ("geo:61,25", txn_geo_to_string),
          ("a:b:c", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # tags: a:b:c
            | # location: geo:61,25
            | # uuid: c075a1a4-37d5-4d79-a92b-5cbb323519f0
            | a  1
            | e -1
            |
            |").strip_margin(),
          3, vec![
          ("c075a1a4-37d5-4d79-a92b-5cbb323519f0", txn_uuid_to_string),
          ("geo:61,25", txn_geo_to_string),
          ("a:b:c", txn_tags_to_string)]
        ),
        (indoc!(
           "|
            |2020-12-24
            | # location: geo:61,25
            | # tags: a:b:c
            | # uuid: c075a1a4-37d5-4d79-a92b-5cbb323519f0
            | a  1
            | e -1
            |
            |").strip_margin(),
          3, vec![
          ("c075a1a4-37d5-4d79-a92b-5cbb323519f0", txn_uuid_to_string),
          ("geo:61,25", txn_geo_to_string),
          ("a:b:c", txn_tags_to_string)]
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
