/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::chars::{id_char, id_start_char, sub_id_start_char};
use winnow::combinator::{cut_err, repeat};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::{one_of, take_while};
use winnow::{ModalResult, Parser};

const CTX_LABEL: &str = "name";

pub(crate) fn p_id_part<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    (one_of(sub_id_start_char), take_while(0.., id_char))
        .take()
        .parse_next(input)
}

pub(crate) fn parse_identifier<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    let res_str = (one_of(id_start_char), take_while(0.., id_char))
        .take()
        .parse_next(input)?;
    Ok(res_str)
}
pub(crate) fn p_identifier<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    parse_identifier(&mut is.input)
}

fn p_id_part_helper<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    (
        take_while(1, ':'),
        cut_err(p_id_part)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "sub-part of name",
            ))),
    )
        .take()
        .parse_next(input)
}
pub(crate) fn parse_multi_part_id<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    let dec_str = (
        parse_identifier,
        cut_err(
            repeat(0.., p_id_part_helper).fold(String::new, |mut string, s| {
                string.push_str(s);
                string
            }),
        )
        .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description(
            "for multi part name",
        ))),
    )
        .take()
        .parse_next(input)?;

    Ok(dec_str)
}

pub(crate) fn p_multi_part_id<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    parse_multi_part_id(&mut is.input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_id() {
        let input = "abcABCäöåÄÖÅ$€£".to_string();

        let res = parse_identifier(&mut input.as_str());

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
    #[test]
    fn test_p_sub_id() {
        let input = "1234abcABCäöåÄÖÅ$€£".to_string();
        let res = p_id_part(&mut input.as_str());

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
}
