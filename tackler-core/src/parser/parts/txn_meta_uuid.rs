/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::{Stream, make_semantic_error};
use uuid::Uuid;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::cut_err;
use winnow::error::{StrContext, StrContextValue};
use winnow::seq;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{ModalResult, Parser};

const CTX_LABEL: &str = "txn metadata uuid";
const UUID_HELP: &str = " # uuid: d77b6b92-42f1-419d-834c-66d69f155ad6";

fn p_uuid(is: &mut Stream<'_>) -> ModalResult<Uuid> {
    let uuid_str = seq!(
        cut_err(take_while(8, AsChar::is_hex_digit))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "1st group (8 hex digits)"
            ))),
        cut_err('-')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "'-' separator"
            ))),
        cut_err(take_while(4, AsChar::is_hex_digit))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "2nd group (4 hex digits)"
            ))),
        cut_err('-')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "'-' separator"
            ))),
        cut_err(take_while(4, AsChar::is_hex_digit))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "3th group (4 hex digits)"
            ))),
        cut_err('-')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "'-' separator"
            ))),
        cut_err(take_while(4, AsChar::is_hex_digit))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "4th group (4 hex digits)"
            ))),
        cut_err('-')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "'-' separator"
            ))),
        cut_err(take_while(12, AsChar::is_hex_digit))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "5th group (12 hex digits)"
            ))),
    )
    .take()
    .parse_next(is)?;

    match Uuid::parse_str(uuid_str) {
        Ok(uuid) => Ok(uuid),
        Err(err) => Err(make_semantic_error(is, err.to_string().as_str())),
    }
}

pub(crate) fn parse_meta_uuid(is: &mut Stream<'_>) -> ModalResult<Uuid> {
    let uuid = seq!(
        _: cut_err("uuid:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'uuid:'"))),
        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space after 'uuid:'"))),
        cut_err(p_uuid)
            .context(StrContext::Expected(StrContextValue::Description(UUID_HELP))),
        _: space0,
        _: cut_err(line_ending)
    )
    .parse_next(is)?;

    Ok(uuid.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_uuid() {
        let mut settings = Settings::default();
        let input = "e009c181-45f3-4286-bd4c-b0e091c3ba47";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_uuid(&mut is);

        assert!(res.is_ok());
        let uuid = res.unwrap(/*:test:*/);
        assert_eq!(
            uuid,
            Uuid::parse_str("e009c181-45f3-4286-bd4c-b0e091c3ba47").unwrap(/*:test:*/)
        );
    }

    #[test]
    fn test_parse_meta_uuid() {
        let mut settings = Settings::default();
        let input = "uuid: c51270e7-305d-40a3-a132-f9ed4b135da7\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_uuid(&mut is);

        assert!(res.is_ok());
        let uuid = res.unwrap(/*:test:*/);
        assert_eq!(
            uuid,
            Uuid::parse_str("c51270e7-305d-40a3-a132-f9ed4b135da7").unwrap(/*:test:*/)
        );
    }
}
