/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::combinator::cut_err;
use winnow::error::StrContext;
use winnow::error::StrContextValue;

use crate::parser::parts::chars::content_char;
use winnow::ascii::line_ending;
use winnow::combinator::peek;
use winnow::token::take_while;
use winnow::{ModalResult, Parser, seq};

pub(crate) fn parse_txn_description<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    let desc = seq!(
        _: '\'',
        take_while(0.., content_char),
        _: cut_err(peek(line_ending)).context(StrContext::Label("transaction description"))
            .context(StrContext::Expected(StrContextValue::Description("valid unicode text and newline"))))
    .parse_next(is)?;

    Ok(desc.0.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use crate::parser::tests::content_data;

    #[test]
    fn test_txn_description() {
        let mut settings = Settings::default();
        let input = "''hello winnow!  \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };
        let res = parse_txn_description(&mut is);
        assert_eq!(res.ok(), Some("'hello winnow!"));
    }

    // test: 641c44ab-0ac3-4247-b16e-a4acea5a78ec
    #[test]
    fn test_parse_txn_description() {
        let mut count = 0;
        let pok_tests = content_data();

        for t in &pok_tests {
            let mut settings = Settings::default();
            let i = format!("'{t}\n");
            let mut is = Stream {
                input: i.as_str(),
                state: &mut settings,
            };

            let res = parse_txn_description(&mut is);

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
