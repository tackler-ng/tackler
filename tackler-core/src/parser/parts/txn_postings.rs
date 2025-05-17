/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::model::posting::txn_sum;
use crate::model::{Posting, Posts};
use crate::parser::parts::txn_posting::{parse_txn_last_posting, parse_txn_posting};
use crate::parser::{Stream, from_error};
use std::ops::Neg;
use winnow::combinator::{opt, repeat};
use winnow::{ModalResult, Parser, seq};

pub(crate) fn parse_txn_postings(is: &mut Stream<'_>) -> ModalResult<Posts> {
    let mut postings = seq!(
        repeat(1.., parse_txn_posting).fold(Vec::new, |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }),
        opt(parse_txn_last_posting),
    )
    .parse_next(is)?;

    if let Some(p) = postings.1 {
        let amount = txn_sum(&postings.0).neg();
        let comm = postings.0[0].txn_commodity.clone();

        let acctn = match is.state.get_or_create_txn_account(p.0, &comm) {
            Ok(acctn) => acctn,
            Err(err) => return Err(from_error(is, err.as_ref())),
        };
        let lp = Posting {
            acctn,
            amount,
            txn_amount: amount,
            is_total_amount: false,
            txn_commodity: comm,
            comment: p.1.map(String::from),
        };
        postings.0.push(lp);
    }

    Ok(postings.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    fn test_p_txn_postings_with_last() {
        let mut input = indoc!(
            "| abc 123.4
             | edf
             |"
        )
        .strip_margin();

        let mut settings = Settings::default();
        let mut is = Stream {
            input: &mut input,
            state: &mut settings,
        };

        let res = parse_txn_postings(&mut is);

        assert!(res.is_ok());
        let acc = res.unwrap(/*:test:*/);
        assert_eq!(acc.len(), 2);
    }
    #[test]
    fn test_p_txn_postings_with_values() {
        let mut input = indoc!(
            "| abc 123.4
             | edf -123.4
             |"
        )
        .strip_margin();

        let mut settings = Settings::default();
        let mut is = Stream {
            input: &mut input,
            state: &mut settings,
        };

        let res = parse_txn_postings(&mut is);

        assert!(res.is_ok());
        let acc = res.unwrap(/*:test:*/);
        assert_eq!(acc.len(), 2);
    }
}
