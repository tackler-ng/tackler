/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::kernel::Settings;
use crate::model::Posting;
use crate::parser::parts::comment::p_comment;
use crate::parser::parts::identifier::p_multi_part_id;
use crate::parser::parts::posting_value::{ValuePosition, parse_posting_value};
use crate::parser::{Stream, from_error};
use crate::tackler;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::opt;
use winnow::{ModalResult, Parser, seq};
/*
// The old ANTLR Grammar

postings: posting+ (posting|last_posting);

posting:  indent account sp amount opt_unit? (opt_comment | opt_sp) NL;

last_posting: indent account (opt_comment | opt_sp) NL;

opt_unit: sp unit opt_position?;

opt_comment: opt_sp comment;

opt_position: opt_opening_pos
    | opt_opening_pos  closing_pos
    | closing_pos
    ;

opt_opening_pos: sp '{' opt_sp amount sp unit opt_sp '}';

closing_pos: sp ('@' | '=') sp amount sp unit;

account: ID (':' (ID | SUBID | INT))*;

amount: INT | NUMBER;

unit: ID;
 */

pub(crate) fn parse_txn_last_posting<'s>(
    is: &mut Stream<'s>,
) -> ModalResult<(&'s str, Option<&'s str>)> {
    let m = seq!(
        _: space1,
        p_multi_part_id,
        _: space0,
        opt(p_comment),
        _: line_ending
    )
    .parse_next(is)?;

    Ok((m.0, m.1))
}

fn handle_posting(
    acc_id: &str,
    vp: ValuePosition,
    comment: Option<&str>,
    settings: &mut Settings,
) -> Result<Posting, tackler::Error> {
    let comm = vp.post_commodity;
    let acctn = settings.get_or_create_txn_account(acc_id, &comm)?;

    Posting::from(
        acctn,
        vp.post_amount,
        vp.txn_amount,
        vp.total_amount,
        vp.txn_commodity,
        comment.map(String::from),
    )
}

pub(crate) fn parse_txn_posting(is: &mut Stream<'_>) -> ModalResult<Posting> {
    let m = seq!(
        _: space1,
        p_multi_part_id,
        _: space1,
        parse_posting_value,
        _: space0,
        opt(p_comment),
        _: line_ending
    )
    .parse_next(is)?;

    match handle_posting(m.0, m.1, m.2, is.state) {
        Ok(posting) => Ok(posting),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_last_posting() {
        let mut settings = Settings::default();
        let input = " abc\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_txn_last_posting(&mut is);

        assert!(res.is_ok());
        let acc = res.unwrap(/*:test:*/);
        assert_eq!(acc, ("abc", None));
    }

    #[test]
    fn test_p_last_posting_comment() {
        let mut settings = Settings::default();
        let input = " abc; foobar\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_txn_last_posting(&mut is);

        assert!(res.is_ok());
        let acc = res.unwrap(/*:test:*/);
        assert_eq!(acc, ("abc", Some("foobar")));
    }

    #[test]
    fn test_p_posting() {
        let tests = [
            " abc 123\n",
            " abc 123 € \n",
            " abc 26 bar·He_50L @ 1.25 EUR\n",
            " abc 26 bar·He_50L = 32.50 EUR\n",
            " a:b:c -1 ACME·INC {120 EUR} @ 123 EUR\n",
            " a:b:c -1 ACME·INC {120 EUR}\n",
            " a:b:c  1 Au·µg {1 EUR}\n",
        ];

        for s in tests {
            let mut settings = Settings::default();

            let mut is = Stream {
                input: s,
                state: &mut settings,
            };

            let res = parse_txn_posting(&mut is);

            assert!(res.is_ok());
        }
    }
}
