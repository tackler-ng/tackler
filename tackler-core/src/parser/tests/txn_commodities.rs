/*
 * Tackler-NG 2017-2025
 * SPDX-License-Identifier: Apache-2.0
 */
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserCommoditiesTest.scala
//
#![cfg_attr(rustfmt, rustfmt_skip)]
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use tackler_rs::IndocUtils;


//
// "Units and Commodities") {
//
    #[test]
    // test: aadbdf7c-c1d0-4e1e-a02f-9ca1b5ab2afc
    // desc: "accept commodity names"
    fn ok_uncommon_accounts() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD
          | a
          |
          |2019-01-01
          | e   1 €
          | a
          |
          |2019-01-01
          | e   1 ¢
          | a
          |
          |2019-01-01
          | e   1 $
          | a
          |
          |2019-01-01
          | e   1 £
          | a
          |
          |2019-01-01
          | e   1 ¥
          | a
          |
          |2019-01-01
          | e   1 ¤
          | a
          |
          |2019-01-01
          | e   1 Au·µg
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 8);
      }

    #[test]
    //desc: "uac ; comment"
    fn ok_commodity_and_comment_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD; comment
          | a
          |
          |2017-01-01
          | e   1 USD ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }

    #[test]
    // test: 5f5dcb57-792d-49df-a491-2923612a0e2f
    // desc: "accepts closing position"
    fn ok_closing_position() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 1.20 EUR
          | a
          |
          |2019-01-01
          | e   1 USD @ 1 €
          | a
          |
          |2019-01-01
          | e   1 € @ 1 $
          | a
          |
          |2019-01-01
          | e   1 $ @ 1 £
          | a
          |
          |2019-01-01
          | e   1 £ @ 1 ¥
          | a
          |
          |2019-01-01
          | e   1 ¥ @ 1 ¢
          | a
          |
          |2019-01-01
          | e   1 ¢ @ 1 Au·µg
          | a
          |
          |2019-01-01
          | e   1 Au·µg @ 1 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 8);
    }

    #[test]
    //desc: "uac closing position ; comment"
    fn ok_commodity_and_comment_closing_pos_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 1.20 EUR; comment
          | a
          |
          |2017-01-01
          | e   1 USD @ 1.20 EUR ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }
//
//  describe("Profit and Loss parsing") {
//
    #[test]
    // test: 9f711991-c9ae-4558-923c-95a69faff8bc
    // desc: "opening with PnL"
    fn ok_opening_with_pnl() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR}
          | a
          |
          |2017-01-01
          | e   -1 USD {1.20 EUR}
          | a
          |
          |2019-01-01
          | e   1 USD {1 €}
          | a
          |
          |2019-01-01
          | e   1 € { 1 $ }
          | a
          |
          |2019-01-01
          | e   1 $ {1 £ }
          | a
          |
          |2019-01-01
          | e   1 £ { 1 ¥}
          | a
          |
          |2019-01-01
          | e   1 ¥ {1 ¢}
          | a
          |
          |2019-01-01
          | e   1 ¢ {1 Au·µg}
          | a
          |
          |2019-01-01
          | e   1 Au·µg {1 EUR}
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 9);
    }

    #[test]
    // test: 92f75975-061b-4867-87f5-e25cf5b13d40
    // desc: "opening with PnL ; comment"
    fn ok_closing_position_2() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR}; comment
          | a
          |
          |2017-01-01
          | e   1 USD {1.20 EUR} ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }

    #[test]
    // test: 84d81380-8664-45d7-a9e1-523c38c7a963
    // desc: "closing position with PnL"
    fn ok_closing_position_3() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR
          | a
          |
          |2017-01-01
          | e   -1 USD {1.20 EUR} @ 1.09 EUR
          | a
          |
          |2019-01-01
          | e   1 USD {1 €} @ 1.09 €
          | a
          |
          |2019-01-01
          | e   1 € { 1 $ } @ 1.09 $
          | a
          |
          |2019-01-01
          | e   1 $ {1 £ } @ 1.09 £
          | a
          |
          |2019-01-01
          | e   1 £ { 1 ¥} @ 1.09  ¥
          | a
          |
          |2019-01-01
          | e   1 ¥ {1 ¢} @ 1.09 ¢
          | a
          |
          |2019-01-01
          | e   1 ¢ {1 Au·µg} @ 1.09 Au·µg
          | a
          |
          |2019-01-01
          | e   1 ⁴ {1 EUR} @ 1.09 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 9);
    }

    #[test]
    // test: c1fbac7b-e924-4eee-aed3-b11b51116f1a
    // desc: "closing position with PnL ; comment"
    fn ok_closing_position_4() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR; comment
          | a
          |
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }


//
//  describe("Invalid inputs and errors") {
//
//    describe("Logical errors") {
//

      #[test]
      // test: 5af5d0d8-ca6e-4a03-a939-99d9d2a4ec43
      // desc: "Unit cost '{ ... }' with negative value"
      fn err_unit_cost() {
        let  txns_str =
    indoc!("|
            |2017-01-01
            | e   1.12 USD {-1.00 EUR}
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          // let msg = res.err().unwrap(/*:test:*/).to_string();
          // todo: assert!(msg.contains("Unit cost"));
          // todo: assert!(msg.contains("is negative"));
      }

      #[test]
      // test: a27b166c-e9c9-432c-bb9d-91915b51d76b
      // desc: "Unit price '@' with negative value"
      fn err_unit_price() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ -1.2 $
            | a 1.2 $
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Unit price"));
          assert!(msg.contains("is negative"));
      }

      #[test]
      // test: 6d1868da-3b9f-45e4-a2c6-db003da4c720
      // desc: "Unit price '@' with same primary and secondary commodity"
      fn err_unit_price_commodity() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          //let msg = res.err().unwrap(/*:test:*/).to_string();
          //todo: assert!(msg.contains("Both commodities are same for value position [€]"));
      }

      #[test]
      // test: fe246259-2280-4d42-8360-6dd3e280b30a
      // desc: "Unit price '@' with discrepancy of commodities"
      fn err_unit_price_commodities() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ 1 $
            | a 1 € @ 1 £
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Different commodities without"));
      }

      #[test]
      // test: 6f45f594-c4e6-449a-b6d2-7f25e9479bd5
      // desc: "Total cost '=' with different sign (-1st vs. +2nd)"
      fn err_total_cost() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e -1 $ = 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Total cost"));
          assert!(msg.contains("different sign"));
      }

      #[test]
      // test: aaf50217-1d04-49bd-a873-43a53be1c99f
      // desc: "Total cost '=' with different sign (+1st vs. -2nd)"
      fn err_total_cost_2() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 $ = -1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Total cost"));
          assert!(msg.contains("different sign"));
      }


      #[test]
      // test: aa52ac0a-278a-49e4-abad-fc2f00416a41
      // desc: "Total cost '=' with same primary and secondary commodity"
      fn err_total_cost_3() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € = 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          assert!(res.err().unwrap(/*:test:*/).to_string().contains("Both commodities are same for value position [€]"));
      }

      #[test]
      // test: 20b89e3e-a987-4e83-bd89-2cbf288caecc
      // desc: "Total cost '=' with discrepancy of commodities"
      fn err_total_cost_4() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € = 1 $
            | a 1 € = 1 £
            |
            |").strip_margin();

          let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
          assert!(res.is_err());
          assert!(res.err().unwrap(/*:test:*/).to_string().contains("Different commodities without"));
      }

    #[test]
    // test: 4babf379-9d88-49f3-8158-b9b7ff4e6eed
    // desc: "perr: duplicate commodity"
    fn err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: e24aacdf-fba2-4dc7-8165-4270c8822559
    // desc: "perr: value position, no primary commodity"
    fn err_parse_2() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 @ 1 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: 0d1beaf2-c30c-4008-943f-46aaf44e4f76
    // desc: "perr: value position, no secondary commodity"
    fn err_parse_3() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 2
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: 3152ec2f-4d5f-4a0a-b88c-68f17bccf7c6
    //desc: "perr: missing value pos value"
    fn err_parse_4() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: bed02ea9-4191-4c98-b847-6b4e2a0fcb2d
    // desc: "perr: with opening (comm)"
    fn err_parse_5() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.00} @ 1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: ac4a6183-fb21-4847-8b3e-912f21fe5a6b
    //desc: "perr: with opening (value)"
    fn err_parse_6() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {EUR} @ 1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    // test: 436d9ed5-b7a0-4e37-a7b4-86b00eb60e83
    // desc: "perr: with missing @"
    fn err_parse_7() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.00 EUR}  1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&mut txns_str.as_str(), &mut Settings::default());
        assert!(res.is_err());
        // todo: assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }
