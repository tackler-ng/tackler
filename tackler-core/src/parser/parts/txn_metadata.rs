/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::parser::Stream;
use crate::parser::parts::txn_meta_location::parse_meta_location;
use crate::parser::parts::txn_meta_tags::parse_meta_tags;
use crate::parser::parts::txn_meta_uuid::parse_meta_uuid;
use std::io;
use std::io::Error;
use tackler_api::location::GeoPoint;
use tackler_api::txn_header::Tags;
use uuid::Uuid;
use winnow::ascii::space1;
use winnow::combinator::{cut_err, fail, peek, repeat};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::any;
use winnow::{ModalResult, Parser, dispatch, seq};

#[derive(Debug)]
pub(crate) struct TxnMeta {
    pub(crate) uuid: Option<Uuid>,
    pub(crate) tags: Option<Tags>,
    pub(crate) location: Option<GeoPoint>,
}
impl TxnMeta {
    fn new() -> TxnMeta {
        TxnMeta {
            uuid: None,
            tags: None,
            location: None,
        }
    }
}

enum MetaItem {
    Uuid(Uuid),
    Location(GeoPoint),
    Tags(Tags),
}

const CTX_LABEL: &str = "txn metadata";

fn p_meta_uuid(is: &mut Stream<'_>) -> ModalResult<MetaItem> {
    let m = parse_meta_uuid.parse_next(is)?;
    Ok(MetaItem::Uuid(m))
}

fn p_meta_tags(is: &mut Stream<'_>) -> ModalResult<MetaItem> {
    let m = parse_meta_tags.parse_next(is)?;
    Ok(MetaItem::Tags(m))
}

fn p_meta_location(is: &mut Stream<'_>) -> ModalResult<MetaItem> {
    let m = parse_meta_location.parse_next(is)?;
    Ok(MetaItem::Location(m))
}

fn p_meta_item(is: &mut Stream<'_>) -> ModalResult<MetaItem> {
    let item = dispatch! {
        peek(any);
        'u' => p_meta_uuid,
        'l' => p_meta_location,
        't' => p_meta_tags,
        _ => cut_err(fail)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid item: 'uuid', 'location' or 'tags'"))),
    }
    .parse_next(is)?;

    Ok(item)
}

pub(crate) fn parse_txn_meta(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let meta = cut_err(
        repeat(
            0..,
            seq!(
                _: space1,
                _: '#',
                _: cut_err(space1)
                    .context(StrContext::Label(CTX_LABEL))
                    .context(StrContext::Expected(StrContextValue::Description("space after '#'"))),
                p_meta_item
            ),
        )
        .try_fold(TxnMeta::new, |mut acc, item| -> Result<_, Error> {
            match item.0 {
                MetaItem::Uuid(u) => {
                    if acc.uuid.is_some() {
                        let msg = "Txn metadata 'uuid' is already defined";
                        return Err(io::Error::other(msg));
                    }
                    acc.uuid = Some(u);
                }
                MetaItem::Tags(t) => {
                    if acc.tags.is_some() {
                        let msg = "Txn metadata 'tags' is already defined";
                        return Err(io::Error::other(msg));
                    }
                    acc.tags = Some(t);
                }
                MetaItem::Location(g) => {
                    if acc.location.is_some() {
                        let msg = "Txn metadata 'location' is already defined";
                        return Err(io::Error::other(msg));
                    }
                    acc.location = Some(g);
                }
            }
            Ok(acc)
        }),
    )
    .parse_next(is)?;

    Ok(meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    struct MetaResult {
        uuid: bool,
        geo: bool,
        tags: bool,
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_parse_txn_meta() {
        #[rustfmt::skip]
        let pok_meta = vec![
            (indoc!(
               "| # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: false, tags: false,}),
            (indoc!(
               "| # location: geo:1.111,2.222,3.333
                |"
            ).strip_margin(),
            MetaResult { uuid: false, geo: true, tags: false,}),
            (indoc!(
               "| # tags: cef, first, second
                |"
            ).strip_margin(),
            MetaResult { uuid: false, geo: false, tags: true,}),

            (indoc!(
               "| # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # location: geo:1.111,2.222,3.333
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: false,}),
            (indoc!(
               "| # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # tags: cef, first, second
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: false, tags: true,}),

            (indoc!(
               "| # location: geo:1.111,2.222,3.333
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: false,}),
            (indoc!(
               "| # location: geo:1.111,2.222,3.333
                | # tags: cef, first, second
                |"
            ).strip_margin(),
            MetaResult { uuid: false, geo: true, tags: true,}),

            (indoc!(
               "| # tags: cef, first, second
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: false, tags: true,}),
            (indoc!(
               "| # tags: cef, first, second
                | # location: geo:1.111,2.222,3.333
                |"
            ).strip_margin(),
            MetaResult { uuid: false, geo: true, tags: true,}),

            (indoc!(
               "| # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # location: geo:1.111,2.222,3.333
                | # tags: cef, first, second
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),
            (indoc!(
               "| # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # tags: cef, first, second
                | # location: geo:1.111,2.222,3.333
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),

            (indoc!(
               "| # location: geo:1.111,2.222,3.333
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # tags: cef, first, second
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),
            (indoc!(
               "| # location: geo:1.111,2.222,3.333
                | # tags: cef, first, second
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),

            (indoc!(
               "| # tags: cef, first, second
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                | # location: geo:1.111,2.222,3.333
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),
            (indoc!(
               "| # tags: cef, first, second
                | # location: geo:1.111,2.222,3.333
                | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
                |"
            ).strip_margin(),
            MetaResult { uuid: true, geo: true, tags: true,}),
        ];

        let mut count = 0;
        for t in pok_meta {
            let mut settings = Settings::default();
            let mut is = Stream {
                input: t.0.as_str(),
                state: &mut settings,
            };

            let res = parse_txn_meta(&mut is);
            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: {}\n",
                count + 1
            );

            let meta = res.unwrap(/*:test:*/);
            assert_eq!(
                meta.uuid.is_some(),
                t.1.uuid,
                "\nUUID: Offending test vector item: {}",
                count + 1
            );
            assert_eq!(
                meta.location.is_some(),
                t.1.geo,
                "\nGEO: Offending test vector item: {}",
                count + 1
            );
            assert_eq!(
                meta.tags.is_some(),
                t.1.tags,
                "\nTAGS: Offending test vector item: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, 15);
    }
}
