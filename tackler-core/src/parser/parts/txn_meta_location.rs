/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::parts::number::p_number;
use crate::parser::{Stream, from_error};
use tackler_api::location::GeoPoint;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::{cut_err, opt, preceded};
use winnow::error::{StrContext, StrContextValue};
use winnow::{ModalResult, Parser, seq};

const CTX_LABEL: &str = "txn metadata location";

fn p_geo_uri(is: &mut Stream<'_>) -> ModalResult<GeoPoint> {
    let (lat, lon, alt) = seq!(
        _: cut_err("geo:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'geo:' uri specifier'")))
            .context(StrContext::Expected(StrContextValue::Description(" # location: geo: lat, lon [,alt]"))),
        _: space0,
        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid latitude"))),
        _: space0,
        _: cut_err(',')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("',' after latitude"))),
        _: space0,
        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid longitude"))),
        _: space0,
        opt(preceded(
            ',',
            preceded(
                space0,
                cut_err(p_number)
                    .context(StrContext::Label(CTX_LABEL))
                    .context(StrContext::Expected(StrContextValue::Description("valid altitude"))),
            )))
    )
    .parse_next(is)?;

    match GeoPoint::from(lat, lon, alt) {
        Ok(point) => Ok(point),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
}

pub(crate) fn parse_meta_location(is: &mut Stream<'_>) -> ModalResult<GeoPoint> {
    let geo = seq!(
        _: cut_err("location:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'location:'"))),
        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space after 'location:'"))),
        cut_err(p_geo_uri)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid geo uri"))),
        _: space0,
        _: cut_err(line_ending)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("line ending"))),
    )
    .parse_next(is)?;

    Ok(geo.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_geo_uri() {
        let mut settings = Settings::default();
        let input = "geo:66.5436,25.84715,160";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_geo_uri(&mut is);

        assert!(res.is_ok());
        let _geo = res.unwrap(/*:test:*/);
        //assert_eq!(geo, "geo:66.5436,25.84715,160");
    }

    #[test]
    fn test_parse_meta_location() {
        let mut settings = Settings::default();
        let input = "location: geo:66.5436,25.84715,160\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_location(&mut is);

        assert!(res.is_ok());
        let _geo = res.unwrap(/*:test:*/);
        //assert_eq!(format!("{geo}"), "geo:66.5436,25.84715,160");
    }
}
