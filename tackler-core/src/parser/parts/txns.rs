/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use itertools::Itertools;
use winnow::{ModalResult, Parser, seq};

use crate::model::{Transaction, Txns};
use crate::parser::parts::txn_header::parse_txn_header;
use crate::parser::parts::txn_postings::parse_txn_postings;
use crate::parser::{Stream, from_error, make_semantic_error};
use winnow::ascii::{line_ending, space0};
use winnow::combinator::alt;
use winnow::combinator::{cut_err, eof, opt, preceded, repeat, repeat_till};
use winnow::error::StrContext;

pub(crate) fn multispace0_line_ending<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    // space0 can't be multispace0 as it's greedy and eats away the last line ending
    repeat(1.., (space0, line_ending))
        .map(|()| ())
        .parse_next(is)?;
    Ok("")
}

fn parse_txn(is: &mut Stream<'_>) -> ModalResult<Transaction> {
    let txn = seq!(
        cut_err(parse_txn_header)
            .context(StrContext::Label("Txn Header")),
        cut_err(parse_txn_postings)
            .context(StrContext::Label("Txn Postings")),
        _: alt((multispace0_line_ending, eof)),
    )
    .context(StrContext::Label("Transaction"))
    .parse_next(is)?;

    if txn.1.iter().map(|p| &p.txn_commodity.name).unique().count() > 1 {
        let msg = format!(
            "Different commodities without value positions are not allowed inside single transaction.{}",
            txn.0
                .uuid
                .map(|u| format!("\n   txn uuid: {u}"))
                .unwrap_or_default()
        );
        return Err(make_semantic_error(is, msg.as_str()));
    }

    match Transaction::try_from(txn.0, txn.1) {
        Ok(txn) => Ok(txn),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
}

pub(crate) fn parse_txns(input: &mut Stream<'_>) -> ModalResult<Txns> {
    let txns: (Vec<Transaction>, &str) = preceded(
        opt(multispace0_line_ending),
        repeat_till(1.., parse_txn, eof),
    )
    .parse_next(input)?;

    Ok(txns.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    fn test_parse_txns() {
        #[rustfmt::skip]
        let pok_txns = vec![
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|  \t \n\
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|\t \n\
                | \t  \t
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a
                |
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a
                |\t \n\
                | \t  \t
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |
                |2025-01-03
                | e 1
                | a
                |
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |\t \n\
                | \t  \t
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 2usize),
        ];

        let mut count = 0;
        for t in pok_txns {
            let mut settings = Settings::default();
            let mut is = Stream {
                input: t.0.as_str(),
                state: &mut settings,
            };

            let res = parse_txns(&mut is);
            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: {}\n",
                count + 1
            );

            let txns = res.unwrap(/*:test:*/);
            assert_eq!(
                txns.len(),
                t.1,
                "\nWrong Txns count: Offending test vector item: {}",
                count + 1
            );

            count += 1;
        }
        assert_eq!(count, 8);
    }
}
