/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::chars::content_char;
use winnow::combinator::cut_err;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::token::take_while;
use winnow::{ModalResult, Parser, seq};

fn valid_code_char(c: char) -> bool {
    !matches!(
        c,
        ')' | '\'' | '(' | '[' | ']' | '{' | '}' | '<' | '>' | '\r' | '\n'
    ) && content_char(c)
}

pub(crate) fn parse_txn_code<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
    let code = seq!(
        _: '(',
        take_while(0.., valid_code_char),
        _: cut_err(')')
            .context(StrContext::Label("transaction code"))
            .context(StrContext::Expected(StrContextValue::Description(
"valid unicode text and closing ')'
Invalid characters for code value are:
    '\\'', '(', ')', '[', ']', '{', '}', '<', '>', \\r', '\\n'
"
        ))),
    )
    .parse_next(is)?;

    Ok(code.0.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use crate::parser::tests::content_data;

    #[test]
    fn test_txn_code_basic() {
        let mut settings = Settings::default();
        let input = "(#foo)";
        let mut is = Stream {
            input,
            state: &mut settings,
        };
        let res = parse_txn_code(&mut is);
        assert_eq!(res.ok(), Some("#foo"));
    }

    // test: 0ac0ac5a-2a90-4bab-a044-38e13e96c443
    #[test]
    fn test_parse_txn_code() {
        let mut count = 0;
        let pok_tests = content_data();

        // Skip first test vector as it contains full ascii punctuation,
        // which is not valid for code
        for t in pok_tests.iter().skip(1) {
            let mut settings = Settings::default();
            let i = format!("({t})");
            let mut is = Stream {
                input: i.as_str(),
                state: &mut settings,
            };
            let res = parse_txn_code(&mut is);
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
        assert_eq!(count, pok_tests.len() - 1);
    }
}
