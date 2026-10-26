/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */

use winnow::{
    ModalResult, Parser,
    ascii::{line_ending, multispace0, space0, space1},
    combinator::opt,
    error::{StrContext, StrContextValue},
    seq,
};

use crate::model::price_entry::PriceEntry;
use crate::parser::{Stream, from_error, parts::timestamp::parse_timestamp};

use super::{comment::p_comment, identifier::p_identifier, number::p_number};

#[allow(clippy::type_complexity)]
pub(crate) fn parse_price_entry(is: &mut Stream<'_>) -> ModalResult<PriceEntry> {
    let (timestamp, base_commodity, eq_amount, eq_commodity, comments) = seq!(
        _: 'P'.context(StrContext::Expected(StrContextValue::Description("price entry starts with `P`"))),
        _: space1,
        parse_timestamp,
        _: space1,
        p_identifier
            .context(StrContext::Expected(StrContextValue::Description("price entry must have base commodity"))),
        _: space1,
        p_number
            .context(StrContext::Expected(StrContextValue::Description("price entry must have equivalent amount"))),
        _: space1,
        p_identifier
            .context(StrContext::Expected(StrContextValue::Description("price entry must have equivalent commodity"))),
        _: space0,
        opt(p_comment),
        _: line_ending,
        _: multispace0,
    )
    .parse_next(is)?;

    let base_commodity = is
        .state
        .state
        .get_or_create_commodity(base_commodity)
        .map_err(|e| from_error(is, &*e))?;

    let eq_commodity = is
        .state
        .state
        .get_or_create_commodity(eq_commodity)
        .map_err(|e| from_error(is, &*e))?;

    let comments = comments.map(String::from);

    Ok(PriceEntry {
        timestamp,
        base_commodity,
        eq_amount,
        eq_commodity,
        comments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_parse_price_entry() {
        let tests = [
            "P 2024-12-30                XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22       XAU 2659.64 USD ; space\n",
            "P 2024-12-30                XAU 2659.64 USD; no space\n",
            "P 2024-12-30T20:21:22Z      XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22+02:00 XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22.12    XAU 2659.64 USD\n",
        ];

        for s in tests {
            let mut settings = Settings::default();

            let mut is = Stream {
                input: s,
                state: &mut settings,
            };

            let res = parse_price_entry(&mut is);

            assert!(res.is_ok());
        }
    }
}
