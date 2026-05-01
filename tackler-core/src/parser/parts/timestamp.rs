/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use winnow::{ModalResult, Parser, seq};

use crate::parser::{Stream, from_error, make_semantic_error};
use crate::tackler;
use std::str::FromStr;
use winnow::combinator::{alt, cut_err, opt};
use winnow::error::{StrContext, StrContextValue};
use winnow::stream::AsChar;
use winnow::token::take_while;

const CTX_LABEL: &str = "ISO 8601 timestamp";

fn p_date(is: &mut Stream<'_>) -> ModalResult<jiff::civil::Date> {
    let (y, m, d) = seq!(
        take_while(4, AsChar::is_dec_digit).try_map(i16::from_str)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("year should be 'YYYY'"))),
        _: cut_err("-")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("separator should be '-'"))),
        cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("month should be 'MM'"))),
        _: cut_err("-")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("separator should be '-'"))),
        cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("day should be 'DD'"))),
    )
    .parse_next(is)?;

    match jiff::civil::Date::new(y, m, d) {
        Ok(d) => Ok(d),
        Err(err) => Err(from_error(is, &err)),
    }
}

#[allow(clippy::cast_possible_truncation)]
fn handle_time(
    h: i8,
    m: i8,
    s: i8,
    ns_opt: Option<&str>,
) -> Result<jiff::civil::Time, tackler::Error> {
    let t = match ns_opt {
        Some(ns_str) => {
            let left_ns = i32::from_str(ns_str)?;
            let ns_len = ns_str.chars().count();
            assert!(ns_len <= 9);

            let ns = left_ns * 10i32.pow(9 - ns_len as u32);
            jiff::civil::Time::new(h, m, s, ns)?
        }
        None => jiff::civil::Time::new(h, m, s, 0)?,
    };
    Ok(t)
}

fn p_time(is: &mut Stream<'_>) -> ModalResult<jiff::civil::Time> {
    let (h, m, s, ns_opt) = seq!(
    cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
        .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description("hours format is 'hh'"))),
    _: cut_err(":")
    .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description("hours-minutes separator is ':'"))),
    cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
        .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description("minutes format is 'mm'"))),
    _: cut_err(":")
    .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description("minutes-seconds separator is ':'"))),
    cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
        .context(StrContext::Label(CTX_LABEL))
        .context(StrContext::Expected(StrContextValue::Description("seconds format is 'ss'"))),
    opt((
        ('.',cut_err(take_while(1.., AsChar::is_dec_digit)))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("nanoseconds format is '.sss' (max 9 decimals)"))),
    )))
    .parse_next(is)?;

    let ns_str = if let Some(ns) = ns_opt {
        if ns.0.1.chars().count() > 9 {
            let err = make_semantic_error(is, "nanoseconds format is '.sss' (max 9 decimals)");
            return Err(err);
        }
        Some(ns.0.1)
    } else {
        None
    };

    let time = match handle_time(h, m, s, ns_str) {
        Ok(t) => t,
        Err(err) => return Err(from_error(is, err.as_ref())),
    };

    Ok(time)
}

fn p_offset(is: &mut Stream<'_>) -> ModalResult<jiff::tz::Offset> {
    #[rustfmt::skip]
    let (sign, h, m) =
        seq!(
            alt(('+'.value(1i32), '-'.value(-1i32))),
            cut_err(take_while(2, AsChar::is_dec_digit))
                .context(StrContext::Label(CTX_LABEL))
                .context(StrContext::Expected(StrContextValue::Description("offset hours 'HH' for offset 'HH:MM'")))
        .try_map(i32::from_str),
            _: cut_err(":")
                .context(StrContext::Label(CTX_LABEL))
                .context(StrContext::Expected(StrContextValue::Description("offset separator ':' for offset 'HH:MM'"))),
            cut_err(take_while(2, AsChar::is_dec_digit))
                .context(StrContext::Label(CTX_LABEL))
                .context(StrContext::Expected(StrContextValue::Description("offset minutes 'MM' for offset 'HH:MM'")))
            .try_map(i32::from_str),
        )
        .parse_next(is)?;

    match jiff::tz::Offset::from_seconds(sign * (h * 60 * 60 + m * 60)) {
        Ok(offset) => Ok(offset),
        Err(err) => Err(from_error(is, &err)),
    }
}

fn p_zulu_or_offset(is: &mut Stream<'_>) -> ModalResult<jiff::tz::Offset> {
    #[rustfmt::skip]
    let res = alt((
        'Z'.map(|_| jiff::tz::Offset::UTC),
        p_offset
    )).parse_next(is)?;

    Ok(res)
}

pub(crate) fn parse_timestamp(is: &mut Stream<'_>) -> ModalResult<jiff::Zoned> {
    let ts_result = seq!(
        cut_err(p_date)
            .context(StrContext::Label("ISO 8601 timestamp"))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "Accepted ISO-8601 timestamp formats are:
   YYYY-MM-DD
   YYYY-MM-DDThh:mm:ss[.sss]
   YYYY-MM-DDThh:mm:ss[.sss]Z
   YYYY-MM-DDThh:mm:ss[.sss][+-]HH:MM
up to nanosecond precision (e.g. the optional .sss part with 9 decimals)
",
            ))),
        opt(seq!(
            _: 'T',
            p_time,
            opt(p_zulu_or_offset)
        ))
    )
    .parse_next(is)?;

    if let Some((time, offset_opt)) = ts_result.1 {
        let dt = ts_result.0.to_datetime(time);
        if let Some(tz) = offset_opt {
            match dt.to_zoned(tz.to_time_zone()) {
                Ok(ts) => Ok(ts),
                Err(err) => Err(from_error(is, &err)),
            }
        } else {
            match is.state.get_offset_datetime(dt) {
                Ok(ts) => Ok(ts),
                Err(err) => Err(from_error(is, err.as_ref())),
            }
        }
    } else {
        match is.state.get_offset_date(ts_result.0) {
            Ok(ts) => Ok(ts),
            Err(err) => Err(from_error(is, err.as_ref())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_date() {
        let mut settings = Settings::default();
        let input = "2024-12-30";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_date(&mut is).is_ok());
    }

    #[test]
    fn test_datetime() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }

    #[test]
    fn test_datetime_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }

    #[test]
    fn test_datetime_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }

    #[test]
    fn test_datetime_milli() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }
    #[test]
    fn test_datetime_micro() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }
    #[test]
    fn test_datetime_nano() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345678";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }
    #[test]
    fn test_datetime_nano_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }
    #[test]
    fn test_datetime_nano_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_ok());
    }

    #[test]
    fn test_datetime_nano_err() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.1234567890+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_timestamp(&mut is).is_err());
    }
}
