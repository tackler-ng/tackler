/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::kernel::Settings;
use crate::parser::parts::identifier::p_multi_part_id;
use crate::parser::{Stream, from_error};
use crate::tackler;
use itertools::Itertools;
use tackler_api::txn_header::Tags;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::{cut_err, repeat};
use winnow::error::{StrContext, StrContextValue};
use winnow::{ModalResult, Parser, seq};

const CTX_LABEL: &str = "txn metadata tags";

fn handle_tags(v: Vec<&str>, settings: &mut Settings) -> Result<Tags, tackler::Error> {
    let mut tags = Vec::with_capacity(v.len());

    for t in v {
        let at = settings.get_or_create_tag(t)?; // todo: fix
        tags.push(at);
    }

    if tags.len() != tags.iter().unique().count() {
        let msg = if tags.len() < 1024 {
            format!("txn tags contains duplicate tags: {tags:?}")
        } else {
            format!(
                "txn tags contains duplicate tags, and size of tags is: {}",
                tags.len()
            )
        };
        return Err(msg.into());
    }

    Ok(tags)
}

fn p_tags(is: &mut Stream<'_>) -> ModalResult<Tags> {
    let mut tags = (
        cut_err(p_multi_part_id)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description(
                "tag value after 'tags:'",
            ))),
        repeat(
            0..,
            seq!(
             _: space0,
                _: ',',
                _: space0,
                cut_err(p_multi_part_id)
                    .context(StrContext::Label(CTX_LABEL))
                    .context(StrContext::Expected(StrContextValue::Description("tag after ','"))),
            ),
        )
        .fold(Vec::new, |mut acc, x| {
            acc.push(x.0);
            acc
        }),
    )
        .parse_next(is)?;

    let mut v = Vec::with_capacity(tags.1.len());
    v.push(tags.0);
    v.append(&mut tags.1);

    match handle_tags(v, is.state) {
        Ok(tags) => Ok(tags),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
}

pub(crate) fn parse_meta_tags(is: &mut Stream<'_>) -> ModalResult<Tags> {
    let tags = seq!(
        _: cut_err("tags:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'tags:'"))),
        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space after 'tags:'"))),
        cut_err(p_tags)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid tags"))),
        _: space0,
        _: cut_err(line_ending)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("line ending"))),

    )
    .parse_next(is)?;

    Ok(tags.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_tags() {
        let mut settings = Settings::default();
        let input = "first, second, third";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_tags(&mut is);

        assert!(res.is_ok());
        let tags = res.unwrap(/*:test:*/);
        //assert_eq!(tags, ["first", "second", "third"]);
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_p_tags_err() {
        let mut settings = Settings::default();
        let input = "tags: first, , third \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_tags(&mut is);

        assert!(res.is_err());
    }

    #[test]
    fn test_parse_meta_tags() {
        let mut settings = Settings::default();
        let input = "tags: a, first:second:third \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_tags(&mut is);

        assert!(res.is_ok());
        let tags = res.unwrap(/*:test:*/);
        //assert_eq!(tags, ["a", "first:second:third"]);
        assert_eq!(tags.len(), 2);
    }
}
