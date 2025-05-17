/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::txn_meta_location::parse_meta_location;
use crate::parser::parts::txn_meta_tags::parse_meta_tags;
use crate::parser::parts::txn_meta_uuid::parse_meta_uuid;
use tackler_api::location::GeoPoint;
use tackler_api::txn_header::Tags;
use uuid::Uuid;
use winnow::combinator::{alt, opt};
use winnow::{ModalResult, Parser, seq};

pub(crate) struct TxnMeta {
    pub(crate) uuid: Option<Uuid>,
    pub(crate) tags: Option<Tags>,
    pub(crate) location: Option<GeoPoint>,
}

fn permutation_uuid(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = parse_meta_uuid.parse_next(is)?;
    Ok(TxnMeta {
        uuid: Some(m),
        tags: None,
        location: None,
    })
}

fn permutation_uuid_tags_o_location(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_uuid, parse_meta_tags, opt(parse_meta_location)).parse_next(is)?;
    Ok(TxnMeta {
        uuid: Some(m.0),
        tags: Some(m.1),
        location: m.2,
    })
}
fn permutation_uuid_location_o_tags(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_uuid, parse_meta_location, opt(parse_meta_tags),).parse_next(is)?;
    Ok(TxnMeta {
        uuid: Some(m.0),
        tags: m.2,
        location: Some(m.1),
    })
}
fn permutation_tags(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = parse_meta_tags.parse_next(is)?;
    Ok(TxnMeta {
        uuid: None,
        tags: Some(m),
        location: None,
    })
}
fn permutation_tags_uuid_o_location(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_tags, parse_meta_uuid, opt(parse_meta_location)).parse_next(is)?;
    Ok(TxnMeta {
        uuid: Some(m.1),
        tags: Some(m.0),
        location: m.2,
    })
}
fn permutation_tags_location_o_uuid(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_tags, parse_meta_location, opt(parse_meta_uuid),).parse_next(is)?;
    Ok(TxnMeta {
        uuid: m.2,
        tags: Some(m.0),
        location: Some(m.1),
    })
}

fn permutation_location(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = parse_meta_location.parse_next(is)?;
    Ok(TxnMeta {
        uuid: None,
        tags: None,
        location: Some(m),
    })
}
fn permutation_location_uuid_o_tags(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_location, parse_meta_uuid, opt(parse_meta_tags)).parse_next(is)?;
    Ok(TxnMeta {
        uuid: Some(m.1),
        tags: m.2,
        location: Some(m.0),
    })
}
fn permutation_location_tags_o_uuid(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    let m = seq!(parse_meta_location, parse_meta_tags, opt(parse_meta_uuid),).parse_next(is)?;
    Ok(TxnMeta {
        uuid: m.2,
        tags: Some(m.1),
        location: Some(m.0),
    })
}

pub(crate) fn parse_txn_meta(is: &mut Stream<'_>) -> ModalResult<TxnMeta> {
    /*
     * ANTLR definition for metadata
     *
     * txn_meta [i32 u, i32 l, i32 t]:  (
     *         {$u < 1}? txn_meta_uuid NL      { let tmp = $u; $u = (tmp+1); }
     *      |  {$l < 1}? txn_meta_location NL  { let tmp = $l; $l = (tmp+1); }
     *      |  {$t < 1}? txn_meta_tags NL      { let tmp = $t; $t = (tmp+1); }
     *      )+;
     */

    // todo: meta permutation: is there better way?
    //
    // "The Winner Takes It All"
    //
    // Alt: Pick the first successful parser, so try
    // the combinations in descending order of (common, length)
    let meta = alt((
        // uuid
        permutation_uuid_tags_o_location,
        permutation_uuid_location_o_tags,
        permutation_uuid,
        // tags
        permutation_tags_uuid_o_location,
        permutation_tags_location_o_uuid,
        permutation_tags,
        // location
        permutation_location_uuid_o_tags,
        permutation_location_tags_o_uuid,
        permutation_location,
    ))
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
