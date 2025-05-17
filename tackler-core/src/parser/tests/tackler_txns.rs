/*
 * Tackler-NG 2016-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use super::*;
use crate::kernel::Settings;
use crate::parser;
use indoc::indoc;
use tackler_rs::IndocUtils;

#[test]
#[allow(clippy::too_many_lines)]
fn txn_data_errors() {
    #[rustfmt::skip]
    let txns_str: Vec<(String, &str)> = vec![
        (
            // test: 52836ff9-94de-4575-bfae-6b5afa971351
            // desc: "tackler_txns with string input: notice unbalanced transaction"
            indoc!(
               "|2017-01-01
                | e  1
                | a  1
                |"
            ).strip_margin(),
            "TXN postings do not zero: 2",
        ),
        (
            // test: 4078aee1-b2e7-40fc-ae82-661d29cbaa74
            // desc: account with missing indentation
            indoc!(
               "|2017-01-01
                |a  1
                | e
                |"
            ).strip_margin(),
            "at line 2, column 1",
        ),
        (
            // test: 399fb5f8-0f03-4aa7-8f2e-1ae8ab2d6645
            // desc: missing blank line between txns
            indoc!(
               "|2017-01-10
                | a  1
                | e -1
                |2017-01-11
                | a  1
                | e -1
                |"
            ).strip_margin(),
            "at line 4, column 1",
        ),
        (
            // test: e23d8e3f-db93-425d-8f3d-690f6d8d84a6
            // desc: "#1: catch multispace0 without line ending between txns"
            indoc!(
               "|2017-01-10
                | a  1
                | e -1
                |   2017-01-11
                | a  1
                | e -1
                |"
            ).strip_margin(),
            "at line 4, column 1",
        ),
        (
            // test: efa25d85-96e4-435b-88c1-0728551c2c2a
            // desc: "#2: catch multispace0 without line ending between txns"
            indoc!(
               "|2017-01-10
                | a  1
                | e -1
                |
                |   2017-01-11
                | a  1
                | e -1
                |"
            ).strip_margin(),
            "at line 5, column 1",
        ),
        (
            // test: fd2e49c0-bb60-4f5b-8ddd-f3745fcc6015
            // desc: Ensure code is not over eager in case of txn syntax error
            // sandwich error txn with ok txn
            // If code-parser is too eager, it will resync parsing
            // by using faulty txn as code
            indoc!(
               "|2017-01-10 (123)
                |  a  1
                |  e -1
                |2017-01-11 (123)
                |  a
                |  e -1
                |2017-01-12 (123)
                |  a  1
                |  e -1
                |"
            ).strip_margin(),
            "at line 4, column 1",
        ),
        (
            // test: a9c742e8-9f9a-42dc-8e09-f1ffe5e6e728
            // desc: "txn comment #1"
            indoc!(
               "|2017-01-10 'desc
                |  a  1
                |  e -1
                |
                |; comment
                |"
            ).strip_margin(),
            "at line 5, column 1",
        ),
        (
            // test: 73a17e9d-3a91-4c29-bbc7-bf8c7b1b347e
            // desc: "txn comment #2"
            indoc!(
               "|2017-01-10 'desc
                |  a  1
                |  e -1
                |
                |; comment
                |2017-01-10 'desc
                |  a  1
                |  e -1
                |"
            ).strip_margin(),
            "at line 5, column 1",
        ),
        (
            // test: 132f11c4-facd-4fbc-9550-eafd751a2cd8
            // desc: "txn comment #3"
            indoc!(
               "|; comment
                |2017-01-10 'desc
                |  a  1
                |  e -1
                |"
            ).strip_margin(),
            "at line 1, column 1",
        ),
        (
            // test: 0c1a7d18-90eb-4f2b-b8b6-9bc36cd5ff73
            // desc: mixed commodities are not accepted
            indoc!(
               "|2017-05-05
                | e  1000 USD
                | a -1000 USD
                | e  1000 EUR
                | a -1000 EUR
                |"
            ).strip_margin(),
            "Semantic error: Different commodities without",
        ),
        (
            // test: edf1c7b6-fac6-4b58-8b5b-6c37b59609f5
            // desc: Mixed commodities with value position (third commodity)
            indoc!(
               "|2017-05-04 'sell four ACME at 120.04 EUR
                | ; value position and fixed commodity differs
                | Assets:Stocks -4 ACME @ 120.04 EUR
                | Assets:Cash 480.16 USD
                |"
            ).strip_margin(),
            "Semantic error: Different commodities without",
        ),
        (
            // test: 8994ca76-615f-4977-bb48-299f85b2b861
            // desc: Mixed commodities with value position (original commodity)
            indoc!(
               "|2017-05-04 'sell four ACME at 120.04 EUR
                | ; mis-match with value position commodities
                | Assets:Stocks -4 ACME @ 120.04 EUR
                | Assets:Cash 480.16 ACME
                |"
            ).strip_margin(),
            "Semantic error: Different commodities without",
        ),

    ];
    let mut count = 0;
    let should_be_count = txns_str.len();
    for t in txns_str {
        let res = parser::string_to_txns(&mut t.0.as_ref(), &mut Settings::default());
        assert!(
            res.is_err(),
            "Testing Error: Offending test vector item: {count}"
        );
        assert!(
            res.err().unwrap(/*:test:*/).to_string().contains(t.1),
            "Testing Line: Offending test vector item: {count}"
        );
        count += 1;
    }
    assert_eq!(count, should_be_count);
}

#[test]
fn test_string_txns_are_sorted() {
    // test: 200aad57-9275-4d16-bdad-2f1c484bcf17
    // desc: "tackler_txns with string input: txns_data is sorted"
    #[rustfmt::skip]
    let txns_str = indoc!(
         "|2017-01-03 'txn-3 by str
          | e  1
          | a
          |
          |2017-01-01 'txn-1 by str
          | e  1
          | a
          |
          |2017-01-02 'txn-2 by str
          | e  1
          | a
          |
          |"
    )
    .strip_margin();

    let res = parser::string_to_txns(&mut txns_str.as_ref(), &mut Settings::default());
    assert!(res.is_ok());
    let txn_data = &res.unwrap(/*:test:*/);
    assert_eq!(txn_data.len(), 3);
    let txns = txn_data.get_all().unwrap(/*:test:*/);
    let txn_1: &Transaction = txns.txns[0];
    let txn_3: &Transaction = txns.txns[2];

    assert_eq!(txn_desc_to_string(txn_1), "txn-1 by str");
    assert_eq!(txn_desc_to_string(txn_3), "txn-3 by str");
}
