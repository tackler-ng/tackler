/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::chars::{id_char, id_start_char};
use winnow::combinator::{cut_err, repeat};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::{one_of, take_while};
use winnow::{ModalResult, Parser};

const CTX_LABEL: &str = "name";

pub(crate) fn p_id_part<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    take_while(1.., id_char).take().parse_next(is)
}

pub(crate) fn p_identifier<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    let res_str = (one_of(id_start_char), take_while(0.., id_char))
        .take()
        .parse_next(is)?;
    Ok(res_str)
}

fn p_id_part_helper<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    (
        take_while(1, ':'),
        cut_err(p_id_part)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "sub-part of name",
            ))),
    )
        .take()
        .parse_next(is)
}

pub(crate) fn p_multi_part_id<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    let dec_str = (
        p_identifier,
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
        .parse_next(is)?;

    Ok(dec_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_id() {
        let mut settings = Settings::default();
        let input = "abcABCäöåÄÖÅ$€£";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_identifier(&mut is);

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
    #[test]
    fn test_p_sub_id() {
        let mut settings = Settings::default();
        let input = "1234abcABCäöåÄÖÅ$€£";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_id_part(&mut is);

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
}
