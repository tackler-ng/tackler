/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */

mod txns_extid {
    use indoc::formatdoc;
    use std::fmt::Write;
    use tackler_core::kernel::Settings;
    use tackler_core::parser;
    use tackler_rs::IndocUtils;

    const EXT_ID_01: &str = "ext-id #001";
    const EXT_ID_02: &str = "ext-id #002";
    const EXT_ID_03: &str = "ext-id #003";

    #[rustfmt::skip]
    fn make_extids() -> String {
        formatdoc!(
            "2026-07-19 'txn01
            | # ext-id: {EXT_ID_01}
            | e  1
            | a
            |
            |2026-07-19 'txn02
            | # ext-id: {EXT_ID_02}
            | e  1
            | a
            |
            |2026-07-19 'txn03
            | # ext-id: {EXT_ID_03}
            | e  1
            | a
            |"
        ).strip_margin()
    }

    #[rustfmt::skip]
    fn dup_extid() -> String {
        formatdoc!(
            "2026-07-19 'txn04
            | # ext-id: {EXT_ID_02}
            | e  1
            | a
            |"
        ).strip_margin()
    }

    #[rustfmt::skip]
    fn make_dups_extid() -> String {
        let mut s = make_extids();
        _ = writeln!(s);
        _ = writeln!(s, "{}", dup_extid());
        s
    }

    #[test]
    // test: f3c3f4fb-2c58-47d8-82a6-82b04a752e2e
    // desc: try_from accepts duplicate ext-ids
    fn txns_try_from_accepts_dup_extid() {
        let txns =
            parser::string_to_txns(&mut make_dups_extid().as_str(), &mut Settings::default());

        assert!(txns.is_ok());
    }

    #[test]
    // test: c4905afd-ea7a-460f-8f0b-ab46803f63be
    // desc: try_from detects duplicate ext-ids
    fn txns_try_from_detects_dup_extid() {
        let txns = parser::string_to_txns(
            &mut make_dups_extid().as_str(),
            &mut Settings::default_extid(),
        );

        let err_msg = txns.expect_err("test case went wonky").to_string();

        assert!(err_msg.contains("Found 1 duplicate"));
        assert!(err_msg.contains("ext-id #002"));
    }

    #[test]
    // test: 843948ac-6d7a-402e-9fe3-931615fd9565
    // desc: append accepts duplicate ext-ids
    fn txns_append_accepts_dup_extid() {
        let mut txns = parser::string_to_txns(
            &mut make_extids().as_str(), &mut Settings::default()).unwrap(/*:test:*/);

        let mut txns_dup = parser::string_to_txns(
            &mut dup_extid().as_str(), &mut Settings::default()).unwrap(/*:test:*/);

        let res = txns.append(&mut txns_dup);

        assert!(res.is_ok());
    }

    #[test]
    // test: 1c8f8fb1-d96b-4661-8597-7ddda75194d5
    // desc: append detects duplicate ext-ids
    fn txns_append_detects_dup_extid() {
        let mut txns = parser::string_to_txns(
            &mut make_extids().as_str(), &mut Settings::default_extid()).unwrap(/*:test:*/);

        let mut txns_dup = parser::string_to_txns(
            &mut dup_extid().as_str(), &mut Settings::default()).unwrap(/*:test:*/);

        let err = txns.append(&mut txns_dup);
        let err_msg = err.expect_err("test case went wonky").to_string();

        assert!(err_msg.contains("Found 1 duplicate"));
        assert!(err_msg.contains("ext-id #002"));
    }
}
