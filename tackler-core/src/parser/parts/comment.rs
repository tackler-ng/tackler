/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::chars::content_char;
use winnow::combinator::{alt, cut_err, peek};
use winnow::stream::AsChar;
use winnow::token::{one_of, take_while};
use winnow::{ModalResult, Parser, seq};
use winnow::{
    ascii::line_ending,
    error::{StrContext, StrContextValue},
};

pub(crate) fn p_comment<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    let m = seq!(
        _: ';',
        cut_err(alt((
            // allow totally empty comment ";\n" - this is important for
            // txn body comments as some editors removes spaces at the end of line
            peek(line_ending).map(|_| {("",)}),
            seq!(
                // this can not be space1 as we must preserve space for equity and identity reports
                _: cut_err(one_of(AsChar::is_space)).context(StrContext::Label("comment"))
                        .context(StrContext::Expected(StrContextValue::Description("space character"))),
                take_while(0.., content_char),
                _: cut_err(peek(line_ending)).context(StrContext::Label("comment"))
                        .context(StrContext::Expected(StrContextValue::Description("valid unicode text and newline")))
            )
        )).map(|x| x.0))
            .context(StrContext::Expected(StrContextValue::Description("comment starts with `;` character, followed by space"))),
    )
    .map(|x| x.0)
    .parse_next(is)?;
    Ok(m)
}

// The line_end handling must work with outer context.
// See also txn_comment.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use crate::parser::tests::content_data;

    #[test]
    // test: 0e43478d-6c0e-4f1d-89c9-08057ece02ce
    fn test_p_comment() {
        let mut count = 0;
        let pok_tests = content_data();

        for t in &pok_tests {
            let mut settings = Settings::default();
            let i = format!("; {t}\n");
            let mut is = Stream {
                input: i.as_str(),
                state: &mut settings,
            };

            let res = p_comment(&mut is);

            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: #{}, '{}'\n",
                count + 1,
                t
            );

            let code = res.unwrap(/*:test:*/);
            assert_eq!(t, code);

            count += 1;
        }
        assert_eq!(count, pok_tests.len());
    }
}
