/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserLocationTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;



     #[test]
     //desc: "various valid geo uris"
     #[allow(non_snake_case)]
     #[allow(clippy::too_many_lines)]
     fn id_bc98cc89_d3b2_468d_9508_8d7a55924178__ok_geo_uris() {
       let  txn_strs: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:60.170833,24.9375
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:60.170833,24.9375",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:66.5436,25.84715,160
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:66.5436,25.84715,160",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:66.5436,25.84715,160.0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:66.5436,25.84715,160.0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:59.90735,16.57532,-155
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:59.90735,16.57532,-155",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:59.90735,16.57532,-155.0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:59.90735,16.57532,-155.0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:0,0,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:0,0,0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:-90,0,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:-90,0,0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:-90,25,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:-90,25,0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:90,0,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:90,0,0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:90,25,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:90,25,0"
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:66.56,180,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:66.56,180,0",
        ),
        (indoc!(
           "|
            |2019-04-01
            | # location: geo:-66.56,-180.0,0
            | e 1
            | a
            |
            |").strip_margin(),
          "geo:-66.56,-180.0,0",
        )
      ];
         let mut count = 0;
         let ref_count = txn_strs.len();
         for t in txn_strs {
             let res = parser::string_to_txns(&mut t.0.as_str(), &mut Settings::default());
             let t_ref = t.1.to_string();
             assert!(res.is_ok(), "Offending test vector item: {count}");
             let txn_data = res.unwrap(/*:test:*/);
             let txns = txn_data.get_all().unwrap(/*:test:*/);
             let txn: &Transaction = txns.txns[0];
             assert_eq!(txn_geo_to_string(txn), t_ref);
             count += 1;
         }
         assert_eq!(count, ref_count);
    }

    #[test]
    //desc: "perr: detect invalid geo uris"
    #[allow(non_snake_case)]
    fn id_c8e7cdf6_3b30_476c_84f0_f5a19812cd28__err_location_parse() {
      let  perr_strings: Vec<(String, &str, &str)> = vec![
          // perr: missing geo-uri
      (indoc!(
           "|
            |2019-05-01
            | # location:
            | e 1
            | a
            |
            |").strip_margin(),
          "line: 3",
          r"at input '"
        ),
          // perr: no 'geo'
        (indoc!(
           "|
            |2019-05-01
            | # location: 60.170833,24.9375
            | e 1
            | a
            |
            |").strip_margin(),
          "line: 3",
          r"at input '"
        ),
          // perr: decimal ','
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:0.0,0.0,0,0
            | e 1
            | a
            |
            |").strip_margin(),
          "line: 3",
          r"at input ' "
        ),
          // perr: missing lat/lon
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:0
            | e 1
            | a
            |
            |").strip_margin(),
          "line: 3",
          r"geo:0"
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
    //desc: "detect semantically invalid geo uris"
    #[allow(non_snake_case)]
    fn id_fc711c0d_2820_4f72_8b4c_1219ef578363__err_location_semantics() {
      let  perr_strings: Vec<(String, &str)> = vec![
          // latitude out of spec 1/2
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:-90.1,0
            | e 1
            | a
            |
            |").strip_margin(),
          r"for Latitude: -90.1"
        ),
          // latitude out of spec 2/2
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:90.1,0
            | e 1
            | a
            |
            |").strip_margin(),
          r"for Latitude: 90.1"
        ),
          // longitude out of spec 1/2
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:0,-180.1
            | e 1
            | a
            |
            |").strip_margin(),
          r"for Longitude: -180.1"
        ),
          // longitude out of spec 2/2
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:0,180.1
            | e 1
            | a
            |
            |").strip_margin(),
          r"for Longitude: 180.1"
        ),
          // altitude out of spec
          // Jules Verne: Voyage au centre de la Terre
        (indoc!(
           "|
            |2019-05-01
            | # location: geo:64.8,-23.783333,-6378137.1
            | e 1
            | a
            |
            |").strip_margin(),
          r"for Altitude: -6378137.1"
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
