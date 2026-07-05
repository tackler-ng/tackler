/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::parser::parts::chars::content_char;
use crate::parser::{Stream, make_semantic_error};
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::cut_err;
use winnow::error::{StrContext, StrContextValue};
use winnow::token::take_while;
use winnow::{ModalResult, Parser, seq};

const CTX_LABEL: &str = "txn metadata ext-id";

pub(crate) fn parse_meta_extid(is: &mut Stream<'_>) -> ModalResult<String> {
    let par_res = seq!(
        _: cut_err("ext-id:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'ext-id:'"))),
        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space after 'ext-id:'"))),
        take_while(0.., content_char),
        _: space0,
        _: cut_err(line_ending)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("line ending"))),

    )
    .parse_next(is)?;

    let extid = par_res.0.trim().to_string();

    if extid.is_empty() {
        Err(make_semantic_error(is, "Empty ext-id"))
    } else {
        Ok(extid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn err_parse_meta_extid() {
        let mut settings = Settings::default();
        let input = "ext-id: \t \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_extid(&mut is);

        assert!(res.is_err());
    }

    #[test]
    fn test_parse_meta_extid() {
        let mut settings = Settings::default();
        let input = "ext-id:    hello \t\t there \t \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_extid(&mut is);

        assert!(res.is_ok());
        let extid = res.unwrap(/*:test:*/);
        assert_eq!(format!("{extid}"), "hello \t\t there");
    }
}
