/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use winnow::{ModalResult, Parser, seq};

use crate::parser::{Stream, from_error};
use crate::tackler;
use std::str::FromStr;
use winnow::combinator::{alt, cut_err, fail, opt};
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

fn parse_date(is: &mut Stream<'_>) -> ModalResult<jiff::Zoned> {
    let date = p_date(is)?;

    match is.state.get_offset_date(date) {
        Ok(date) => Ok(date),
        Err(err) => Err(from_error(is, err.as_ref())),
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

fn p_datetime(is: &mut Stream<'_>) -> ModalResult<jiff::civil::DateTime> {
    let (date, h, m, s, ns_opt) = seq!(
        p_date,
        _: "T",
        cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("hours format is 'HH'"))),
        _: cut_err(":")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("hours-minutes separator is ':'"))),
        cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("minutes format is 'MM'"))),
        _: cut_err(":")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("minutes-seconds separator is ':'"))),
        cut_err(take_while(2, AsChar::is_dec_digit).try_map(i8::from_str))
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("seconds format is 'SS'"))),
        opt((
            ('.',cut_err(take_while(1..=9, AsChar::is_dec_digit)))
                .context(StrContext::Label(CTX_LABEL))
                .context(StrContext::Expected(StrContextValue::Description("nanoseconds format is '.SSS' (max 9 decimals)"))),
        ))
    )
    .parse_next(is)?;

    let time = match handle_time(h, m, s, ns_opt.map(|x| x.0.1)) {
        Ok(t) => t,
        Err(err) => return Err(from_error(is, err.as_ref())),
    };

    Ok(date.to_datetime(time))
}

fn parse_datetime(is: &mut Stream<'_>) -> ModalResult<jiff::Zoned> {
    let dt_jiff: jiff::civil::DateTime = p_datetime(is)?;

    match is.state.get_offset_datetime(dt_jiff) {
        Ok(ts) => Ok(ts),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
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

fn parse_datetime_tz(is: &mut Stream<'_>) -> ModalResult<jiff::Zoned> {
    let (ts, tz) = seq!(p_datetime, p_zulu_or_offset,).parse_next(is)?;

    let ts_tz = match ts.to_zoned(tz.to_time_zone()) {
        Ok(offset) => offset,
        Err(err) => return Err(from_error(is, &err)),
    };

    Ok(ts_tz)
}

pub(crate) fn parse_timestamp(is: &mut Stream<'_>) -> ModalResult<jiff::Zoned> {
    let ts = alt((
        parse_datetime_tz,
        parse_datetime,
        parse_date,
        fail.context(StrContext::Label("ISO 8601 timestamp"))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "ISO-8601 timestamp at the beginning of the line",
            )))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "YYYY-mm-ddTHH:MM:SS[.SSSSSSSSS][+-]HH:MM",
            )))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "YYYY-mm-ddTHH:MM:SS[.SSSSSSSSS]Z",
            )))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "YYYY-mm-ddTHH:MM:SS[.SSSSSSSSS]",
            )))
            .context(StrContext::Expected(StrContextValue::StringLiteral(
                "YYYY-mm-dd",
            ))),
    ))
    .parse_next(is)?;

    Ok(ts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_date() {
        let mut settings = Settings::default();
        let input = "2024-12-30";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_date(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_milli() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_micro() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345678";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_nano_err() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.1234567890+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_datetime_tz(&mut is).is_err());
    }
}
