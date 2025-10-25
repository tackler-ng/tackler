/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! Timestamp utilities
//!
//! `txn_ts` is collection of utilities to generate
//! different representations of Txn timestamps.
//!
use crate::tackler;
use jiff::Zoned;
use jiff::fmt::strtime;
use jiff::tz::{Offset, TimeZone};

/// UTC Timezone
pub static TZ_UTC: Offset = jiff::tz::Offset::UTC;

/// Time stamp style
#[derive(Debug, Copy, Clone, Default)]
pub enum TimestampStyle {
    /// 2024-11-09T14:15:16.123456789 -> 2024-11-09
    Date,
    /// 2024-11-09T14:15:16.123456789 -> 2024-11-09 14:15:16
    Secodns,
    /// This is the default
    ///
    /// 2024-11-09T14:15:16.123456789 -> 2024-11-09 14:15:16.123456789
    #[default]
    Full,
}

impl TimestampStyle {
    /// UI/CFG string for date 'timestamp style' -selector
    pub const DATE: &'static str = "date";

    /// UI/CFG string for seconds 'timestamp style' -selector
    pub const SECONDS: &'static str = "seconds";

    /// UI/CFG string for full 'timestamp style' -selector
    pub const FULL: &'static str = "full";

    /// Get Timestamp style by name
    ///
    /// # Errors
    ///
    /// Returns `Err` if the timestamp style is unknown
    pub fn from(name: &str) -> Result<Self, tackler::Error> {
        match name {
            TimestampStyle::DATE => Ok(TimestampStyle::Date),
            TimestampStyle::SECONDS => Ok(TimestampStyle::Secodns),
            TimestampStyle::FULL => Ok(TimestampStyle::Full),
            _ => Err(format!(
                "Unknown timestamp style: '{name}'. Valid options are: {}, {}, {}",
                Self::DATE,
                Self::SECONDS,
                Self::FULL
            )
            .into()),
        }
    }
}

/// Time granularity selector for `GroupBy` operations
#[derive(Debug, Clone, Copy, Default)]
pub enum GroupBy {
    /// Group by year
    Year,
    /// Group by year-month
    /// This is the default
    #[default]
    Month,
    /// Group by year-month-day
    Date,
    /// Group by ISO week (year-week)
    IsoWeek,
    /// Group by ISO week date (year-week-day)
    IsoWeekDate,
}

impl GroupBy {
    /// UI/CFG string for Year (2024) 'group by' -selector
    pub const YEAR: &'static str = "year";

    /// UI/CFG string for Month (2024-12) 'group by' -selector
    pub const MONTH: &'static str = "month";

    /// UI/CFG string for Date (2024-12-31) 'group by' -selector
    pub const DATE: &'static str = "date";

    /// UI/CFG string for ISO-Week (2024-W51) 'group by' -selector
    pub const ISO_WEEK: &'static str = "iso-week";

    /// UI/CFG string for ISO-Week-Date (2024-W51-5) 'group by' -selector
    pub const ISO_WEEK_DATE: &'static str = "iso-week-date";

    /// Get 'group by' -selector based on UI/CFG name
    ///
    /// # Errors
    ///
    /// Returns `Err` if group-by selector is unknown.
    pub fn from(group_by: &str) -> Result<GroupBy, tackler::Error> {
        match group_by {
            GroupBy::ISO_WEEK_DATE => Ok(GroupBy::IsoWeekDate),
            GroupBy::ISO_WEEK => Ok(GroupBy::IsoWeek),
            GroupBy::DATE => Ok(GroupBy::Date),
            GroupBy::MONTH => Ok(GroupBy::Month),
            GroupBy::YEAR => Ok(GroupBy::Year),
            _ => {
                let msg = format!(
                    "Unknown group-by selector: '{group_by}'. Valid options are: {}, {}, {}, {}, {}",
                    GroupBy::ISO_WEEK_DATE,
                    GroupBy::ISO_WEEK,
                    GroupBy::DATE,
                    GroupBy::MONTH,
                    GroupBy::YEAR
                );
                Err(msg.into())
            }
        }
    }
}
/// Get zoned ts from RFC 3339 string
///
/// # Errors
///
/// Returns `Err` the string is not in valid RFC 3339 format
pub fn rfc3339_to_zoned(rfc3339_str: &str) -> Result<Zoned, tackler::Error> {
    strtime::parse("%Y-%m-%dT%H:%M:%S%.f%:z", rfc3339_str)?
        .to_zoned()
        .map_err(|e| {
            let msg = format!("Can't parse ts as rfc3339: '{rfc3339_str}', error: {e}");
            msg.into()
        })
}
/// RFC-3339 timestamp as string
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-24T14:15:16+02:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::rfc_3339(&ts), "2022-12-24T14:15:16+02:00");
///
/// let ns: Zoned = "2022-06-24T14:15:16.123456789+03:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::rfc_3339(&ns), "2022-06-24T14:15:16.123456789+03:00");
///  # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn rfc_3339(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%dT%H:%M:%S%.f%:z", ts)
        .unwrap_or_else(|err| format!("IE: rfc_3339, frmt error: {err}"))
}

/// Human-readable timestamp with seconds precision and with zone
///
/// This is ISO-8601 style timestamp with space separator between components.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-24T14:15:16+02:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::seconds_tz(&ts), "2022-12-24 14:15:16 +02:00");
///
/// let ns: Zoned = "2022-06-24T14:15:16.123456789+03:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::seconds_tz(&ns), "2022-06-24 14:15:16 +03:00");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn seconds_tz(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%d %H:%M:%S %:z", ts)
        .unwrap_or_else(|err| format!("IE: seconds_tz, frmt error: {err}"))
}

/// Human-readable timestamp with full precision and with zone
///
/// This is ISO-8601 style timestamp with space separator between components.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-24T14:15:16+02:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::full_tz(&ts), "2022-12-24 14:15:16 +02:00");
///
/// let ns: Zoned = "2022-06-24T14:15:16.123456789+03:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::full_tz(&ns), "2022-06-24 14:15:16.123456789 +03:00");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn full_tz(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%d %H:%M:%S%.f %:z", ts)
        .unwrap_or_else(|err| format!("IE: full_tz, frmt error: {err}"))
}

fn fmt_seconds(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%d %H:%M:%S", ts)
        .unwrap_or_else(|err| format!("IE: fmt_seconds, frmt error: {err}"))
}

fn fmt_full(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%d %H:%M:%S%.f", ts)
        .unwrap_or_else(|err| format!("IE: fmt_full, frmt error: {err}"))
}

fn fmt_date(ts: &Zoned) -> String {
    strtime::format("%Y-%m-%d", ts).unwrap_or_else(|err| format!("IE: fmt_date, frmt error: {err}"))
}

fn fmt_month(ts: &Zoned) -> String {
    strtime::format("%Y-%m", ts).unwrap_or_else(|err| format!("IE: fmt_month, frmt error: {err}"))
}

fn fmt_year(ts: &Zoned) -> String {
    strtime::format("%Y", ts).unwrap_or_else(|err| format!("IE: fmt_year, frmt error: {err}"))
}

fn fmt_week(ts: &Zoned) -> String {
    let iso_date = ts.date().iso_week_date();
    let y = iso_date.year();
    let w = iso_date.week();

    format!("{y}-W{w:02}")
}

fn fmt_week_date(ts: &Zoned) -> String {
    let iso_date = ts.date().iso_week_date();
    let y = iso_date.year();
    let w = iso_date.week();
    let wd = iso_date.weekday().to_monday_one_offset();

    format!("{y}-W{w:02}-{wd}")
}

/// Human-readable timestamp with seconds precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use tackler_api::txn_ts;
///
/// let ts = "2022-12-24T14:15:16+02:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::as_utc_seconds(&ts), "2022-12-24 12:15:16");
///
/// let ns = "2022-06-24T14:15:16.123456789+03:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::as_utc_seconds(&ns), "2022-06-24 11:15:16");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_seconds(ts: &Zoned) -> String {
    fmt_seconds(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Human-readable timestamp with full precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-24T14:15:16+02:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::as_utc_full(&ts), "2022-12-24 12:15:16");
///
/// let ns: Zoned  = "2022-06-24T14:15:16.123456789+03:00[Europe/Helsinki]".parse()?;
/// assert_eq!(txn_ts::as_utc_full(&ns), "2022-06-24 11:15:16.123456789");
///  # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_full(ts: &Zoned) -> String {
    fmt_full(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Human readable timestamp with date (day) precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-31T19:00:00-05:00[America/Montreal]".parse()?;
/// assert_eq!(txn_ts::as_utc_date(&ts), "2023-01-01");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_date(ts: &Zoned) -> String {
    fmt_date(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Human-readable timestamp with month precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
///  # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-31T19:00:00-05:00[America/Toronto]".parse()?;
/// assert_eq!(txn_ts::as_utc_month(&ts), "2023-01");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_month(ts: &Zoned) -> String {
    fmt_month(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Human-readable timestamp with year precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2022-12-31T19:00:00-05:00[America/Toronto]".parse()?;
/// assert_eq!(txn_ts::as_utc_year(&ts), "2023");
///  # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_year(ts: &Zoned) -> String {
    fmt_year(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Timestamp with ISO-8601 week precision in UTC timezone
///
/// Timestamp is converted into UTC timezone.
///
/// # Examples
/// ```
///  # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2010-01-03T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week(&ts), "2009-W53");
/// let ts: Zoned = "2010-01-04T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week(&ts), "2010-W01");
///
/// let ny: Zoned = "2010-01-03T19:00:00-05:00[America/Toronto]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week(&ny), "2010-W01");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_iso_week(ts: &Zoned) -> String {
    fmt_week(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Timestamp with ISO-8601 week-date precision in UTC timezone
///
/// Timestamp is converted into UTC timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::Zoned;
/// use tackler_api::txn_ts;
///
/// let ts: Zoned = "2010-01-03T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week_date(&ts), "2009-W53-7");
/// let ts: Zoned = "2010-01-04T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week_date(&ts), "2010-W01-1");
///
/// let ny: Zoned = "2010-01-03T19:00:00-05:00[America/Toronto]".parse()?;
/// assert_eq!(txn_ts::as_utc_iso_week_date(&ny), "2010-W01-1");
///# Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_utc_iso_week_date(ts: &Zoned) -> String {
    fmt_week_date(&ts.with_time_zone(TZ_UTC.to_time_zone().clone()))
}

/// Human readable timestamp with seconds precision in provided timezone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2022-12-24T12:15:16+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_seconds(&ts, new_york_tz.clone()), "2022-12-24 07:15:16");
/// assert_eq!(txn_ts::as_tz_seconds(&ts, helsinki_tz.clone()), "2022-12-24 14:15:16");
///
/// let ns: Zoned = "2022-06-24T12:15:16.123456789+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_seconds(&ns, helsinki_tz.clone()), "2022-06-24 15:15:16");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_seconds(ts: &Zoned, tz: TimeZone) -> String {
    fmt_seconds(&ts.with_time_zone(tz))
}

/// Human readable timestamp with full precision in provided timezone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2022-12-24T12:15:16+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_full(&ts, new_york_tz.clone()), "2022-12-24 07:15:16");
/// assert_eq!(txn_ts::as_tz_full(&ts, helsinki_tz.clone()), "2022-12-24 14:15:16");
///
/// let ns: Zoned = "2022-06-24T12:15:16.123456789+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_full(&ns, helsinki_tz.clone()), "2022-06-24 15:15:16.123456789");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_full(ts: &Zoned, tz: TimeZone) -> String {
    fmt_full(&ts.with_time_zone(tz))
}

/// Human-readable timestamp with date precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2022-12-23T22:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_date(&ts, new_york_tz.clone()), "2022-12-23");
/// assert_eq!(txn_ts::as_tz_date(&ts, helsinki_tz.clone()), "2022-12-24");
///
/// let ns: Zoned = "2022-06-23T21:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_date(&ns, helsinki_tz.clone()), "2022-06-24");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_date(ts: &Zoned, tz: TimeZone) -> String {
    fmt_date(&ts.with_time_zone(tz))
}

/// Human-readable timestamp with month precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2022-12-31T22:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_month(&ts, new_york_tz.clone()), "2022-12");
/// assert_eq!(txn_ts::as_tz_month(&ts, helsinki_tz.clone()), "2023-01");
///
/// let ns: Zoned  = "2022-06-30T21:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_month(&ns, helsinki_tz.clone()), "2022-07");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_month(ts: &Zoned, tz: TimeZone) -> String {
    fmt_month(&ts.with_time_zone(tz))
}

/// Human-readable timestamp with year precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2022-12-31T22:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_year(&ts, new_york_tz.clone()), "2022");
/// assert_eq!(txn_ts::as_tz_year(&ts, helsinki_tz.clone()), "2023");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_year(ts: &Zoned, tz: TimeZone) -> String {
    fmt_year(&ts.with_time_zone(tz))
}

/// Timestamp with ISO-8601 week precision in provided timezone
///
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2010-01-04T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_iso_week(&ts, new_york_tz.clone()), "2009-W53");
/// assert_eq!(txn_ts::as_tz_iso_week(&ts, helsinki_tz.clone()), "2010-W01");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_iso_week(ts: &Zoned, tz: TimeZone) -> String {
    fmt_week(&ts.with_time_zone(tz))
}

/// Timestamp with ISO-8601 week date precision in provided timezone
///
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// use jiff::{tz, Zoned};
/// use tackler_api::txn_ts;
///
/// let new_york_tz = tz::TimeZone::get("America/New_York")?;
/// let helsinki_tz = tz::TimeZone::get("Europe/Helsinki")?;
///
/// let ts: Zoned = "2010-01-04T00:00:00+00:00[UTC]".parse()?;
/// assert_eq!(txn_ts::as_tz_iso_week_date(&ts, new_york_tz.clone()), "2009-W53-7");
/// assert_eq!(txn_ts::as_tz_iso_week_date(&ts, helsinki_tz.clone()), "2010-W01-1");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[must_use]
pub fn as_tz_iso_week_date(ts: &Zoned, tz: TimeZone) -> String {
    fmt_week_date(&ts.with_time_zone(tz))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test: f0e2f23c-7cc6-4610-80c0-8f1e3a6555c7
    fn timestampstyle_full() {
        let tss = TimestampStyle::from("full");
        match tss {
            Ok(TimestampStyle::Full) => (),
            _ => panic!("/*:test:*/"),
        }
    }

    #[test]
    // test: abec5673-f55e-427d-9db6-cdf865100a21
    fn timestampstyle_seconds() {
        let tss = TimestampStyle::from("seconds");
        match tss {
            Ok(TimestampStyle::Secodns) => (),
            _ => panic!("/*:test:*/"),
        }
    }

    #[test]
    // test: 0b8dd80a-e826-4107-9892-7db04e3a9f59
    fn timestampstyle_date() {
        let tss = TimestampStyle::from("date");
        match tss {
            Ok(TimestampStyle::Date) => (),
            _ => panic!("/*:test:*/"),
        }
    }

    #[test]
    // test: fc6c569a-1ab4-4fde-bed6-1593116f617d
    fn timestampstyle_invalid() {
        let tss = TimestampStyle::from("minutes");
        assert!(tss.is_err());
    }

    fn txt2ts(txt_ts: &str) -> Zoned {
        strtime::parse("%Y-%m-%dT%H:%M:%S%.f%:z", txt_ts)
            .unwrap(/*:test:*/)
            .to_zoned()
            .unwrap(/*:test:*/)
    }

    #[test]
    fn doc_test() {
        let ts = txt2ts("2022-12-24T13:14:15+02:00");
        assert_eq!(rfc_3339(&ts), "2022-12-24T13:14:15+02:00");
    }

    #[test]
    fn test_rfc_3339() {
        assert_eq!(
            rfc_3339(&txt2ts("2010-01-02T13:14:15+16:00")),
            "2010-01-02T13:14:15+16:00"
        );
        assert_eq!(
            rfc_3339(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24T01:02:03.456+16:00"
        );
        assert_eq!(
            rfc_3339(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24T01:02:03.456789+16:00"
        );
        assert_eq!(
            rfc_3339(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24T01:02:03.7-16:00"
        );

        assert_eq!(
            rfc_3339(&txt2ts("2010-12-24T00:00:00+00:00")),
            "2010-12-24T00:00:00+00:00"
        );
        assert_eq!(
            rfc_3339(&txt2ts("2020-12-31T23:58:59+00:00")),
            "2020-12-31T23:58:59+00:00"
        );
        assert_eq!(
            rfc_3339(&txt2ts("2020-12-31T23:58:59+00:00")),
            "2020-12-31T23:58:59+00:00"
        );
    }

    #[test]
    fn test_seconds_tz() {
        assert_eq!(
            seconds_tz(&txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00 +00:00"
        ); // todo: time: Z
        assert_eq!(
            seconds_tz(&txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15 +16:00"
        );
        assert_eq!(
            seconds_tz(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03 +16:00"
        );
        assert_eq!(
            seconds_tz(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03 +16:00"
        );
        assert_eq!(
            seconds_tz(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59 +00:00"
        ); // todo: time: Z
        assert_eq!(
            seconds_tz(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03 -16:00"
        );

        assert_eq!(
            seconds_tz(&txt2ts("2020-12-31T23:58:59+00:00")),
            "2020-12-31 23:58:59 +00:00"
        );
    }

    #[test]
    fn test_full_tz() {
        assert_eq!(
            full_tz(&txt2ts("2010-01-02T13:14:15+16:00")),
            "2010-01-02 13:14:15 +16:00"
        );
        assert_eq!(
            full_tz(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03.456 +16:00"
        );
        assert_eq!(
            full_tz(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03.456789 +16:00"
        );
        assert_eq!(
            full_tz(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03.7 -16:00"
        );

        assert_eq!(
            full_tz(&txt2ts("2010-12-24T00:00:00+00:00")),
            "2010-12-24 00:00:00 +00:00"
        );
        assert_eq!(
            full_tz(&txt2ts("2020-12-31T23:58:59.123456789+00:00")),
            "2020-12-31 23:58:59.123456789 +00:00"
        );
    }

    #[test]
    fn test_fmt_seconds() {
        assert_eq!(
            fmt_seconds(&txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            fmt_seconds(&txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15"
        );
        assert_eq!(
            fmt_seconds(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03"
        );
        assert_eq!(
            fmt_seconds(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03"
        );
        assert_eq!(
            fmt_seconds(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59"
        );
        assert_eq!(
            fmt_seconds(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03"
        );
    }

    #[test]
    fn test_fmt_full() {
        assert_eq!(
            fmt_full(&txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            fmt_full(&txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15"
        );
        assert_eq!(
            fmt_full(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03.456"
        );
        assert_eq!(
            fmt_full(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03.456789"
        );
        assert_eq!(
            fmt_full(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59.123456789"
        );
        assert_eq!(
            fmt_full(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03.7"
        );
    }

    #[test]
    fn test_fmt_date() {
        assert_eq!(fmt_date(&txt2ts("2010-01-02T00:00:00+00:00")), "2010-01-02");
        assert_eq!(fmt_date(&txt2ts("2010-12-24T13:14:15+16:00")), "2010-12-24");
        assert_eq!(
            fmt_date(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24"
        );
        assert_eq!(
            fmt_date(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24"
        );
        assert_eq!(
            fmt_date(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24"
        );
        assert_eq!(
            fmt_date(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24"
        );
    }

    #[test]
    fn test_fmt_month() {
        assert_eq!(fmt_month(&txt2ts("2010-01-02T00:00:00+00:00")), "2010-01");
        assert_eq!(fmt_month(&txt2ts("2010-12-24T13:14:15+16:00")), "2010-12");
        assert_eq!(
            fmt_month(&txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12"
        );
        assert_eq!(
            fmt_month(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12"
        );
        assert_eq!(
            fmt_month(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12"
        );
        assert_eq!(
            fmt_month(&txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12"
        );
    }

    #[test]
    fn test_fmt_year() {
        assert_eq!(fmt_year(&txt2ts("2010-01-02T00:00:00+00:00")), "2010");
        assert_eq!(fmt_year(&txt2ts("2010-12-24T13:14:15+16:00")), "2010");
        assert_eq!(fmt_year(&txt2ts("2010-12-24T01:02:03.456+16:00")), "2010");
        assert_eq!(
            fmt_year(&txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010"
        );
        assert_eq!(
            fmt_year(&txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020"
        );
        assert_eq!(fmt_year(&txt2ts("2010-12-24T01:02:03.700-16:00")), "2010");
    }

    #[test]
    fn test_fmt_week() {
        assert_eq!(fmt_week(&txt2ts("2010-01-03T00:00:00+00:00")), "2009-W53");
        assert_eq!(fmt_week(&txt2ts("2010-01-04T00:00:00+00:00")), "2010-W01");
        assert_eq!(fmt_week(&txt2ts("2017-01-01T00:00:00+00:00")), "2016-W52");
        assert_eq!(fmt_week(&txt2ts("2017-01-02T00:00:00+00:00")), "2017-W01");
    }

    #[test]
    fn test_fmt_week_date() {
        assert_eq!(
            fmt_week_date(&txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53-7"
        );
        assert_eq!(
            fmt_week_date(&txt2ts("2010-01-04T00:00:00+00:00")),
            "2010-W01-1"
        );
        assert_eq!(
            fmt_week_date(&txt2ts("2017-01-01T00:00:00+00:00")),
            "2016-W52-7"
        );

        assert_eq!(
            fmt_week_date(&txt2ts("2017-01-02T00:00:00+00:00")),
            "2017-W01-1"
        );

        assert_eq!(
            fmt_week_date(&txt2ts("2020-12-31T00:00:00+00:00")),
            "2020-W53-4"
        );
        assert_eq!(
            fmt_week_date(&txt2ts("2021-01-01T00:00:00+00:00")),
            "2020-W53-5"
        );
    }

    #[test]
    fn test_utc_seconds() {
        assert_eq!(
            as_utc_seconds(&txt2ts("2010-01-01T00:00:00+16:00")),
            "2009-12-31 08:00:00"
        );
        assert_eq!(
            as_utc_seconds(&txt2ts("2010-01-02T14:15:16+00:00")),
            "2010-01-02 14:15:16"
        );
        assert_eq!(
            as_utc_seconds(&txt2ts("2010-01-01T01:02:03.700-16:00")),
            "2010-01-01 17:02:03"
        );
    }

    #[test]
    fn test_utc_full() {
        assert_eq!(
            as_utc_full(&txt2ts("2010-01-01T00:00:00+16:00")),
            "2009-12-31 08:00:00"
        );
        assert_eq!(
            as_utc_full(&txt2ts("2010-01-02T14:15:16.456+00:00")),
            "2010-01-02 14:15:16.456"
        );
        assert_eq!(
            as_utc_full(&txt2ts("2010-01-01T01:02:03.700-16:00")),
            "2010-01-01 17:02:03.7"
        );

        assert_eq!(
            as_utc_full(&txt2ts("2020-12-31T23:58:59.123456789+00:00")),
            "2020-12-31 23:58:59.123456789"
        );

        assert_eq!(
            as_utc_full(&txt2ts("2020-12-31T23:58:59.123456789+00:00")),
            "2020-12-31 23:58:59.123456789"
        );
    }

    #[test]
    fn test_utc_week() {
        assert_eq!(
            as_utc_iso_week(&txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53"
        );
        assert_eq!(
            as_utc_iso_week(&txt2ts("2017-01-02T00:00:00+00:00")),
            "2017-W01"
        );
        assert_eq!(
            as_utc_iso_week(&txt2ts("2017-01-02T00:00:00+02:00")),
            "2016-W52"
        );
        assert_eq!(
            as_utc_iso_week(&txt2ts("2017-01-02T00:00:00-02:00")),
            "2017-W01"
        );
    }

    #[test]
    fn test_utc_week_date() {
        assert_eq!(
            as_utc_iso_week_date(&txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53-7"
        );

        assert_eq!(
            as_utc_iso_week_date(&txt2ts("2017-01-02T00:00:00+00:00")),
            "2017-W01-1"
        );
        assert_eq!(
            as_utc_iso_week_date(&txt2ts("2017-01-02T00:00:00+02:00")),
            "2016-W52-7"
        );
        assert_eq!(
            as_utc_iso_week_date(&txt2ts("2017-01-01T22:00:00+00:00")),
            "2016-W52-7"
        );
        assert_eq!(
            as_utc_iso_week_date(&txt2ts("2017-01-01T22:00:00-02:00")),
            "2017-W01-1"
        );
    }

    #[test]
    fn test_utc_date() {
        assert_eq!(
            as_utc_date(&txt2ts("2010-01-01T15:00:00+16:00")),
            "2009-12-31"
        );
        assert_eq!(
            as_utc_date(&txt2ts("2010-01-01T08:02:03-16:00")),
            "2010-01-02"
        );
        assert_eq!(
            as_utc_date(&txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020-12-31"
        );
    }

    #[test]
    fn test_utc_month() {
        assert_eq!(
            as_utc_month(&txt2ts("2010-01-01T15:00:00+16:00")),
            "2009-12"
        );
        assert_eq!(
            as_utc_month(&txt2ts("2010-01-31T08:02:03-16:00")),
            "2010-02"
        );
        assert_eq!(
            as_utc_month(&txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020-12"
        );
    }

    #[test]
    fn test_utc_year() {
        assert_eq!(as_utc_year(&txt2ts("2010-01-01T15:00:00+16:00")), "2009");
        assert_eq!(as_utc_year(&txt2ts("2010-12-31T08:02:03-16:00")), "2011");
        assert_eq!(
            as_utc_year(&txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020"
        );
    }

    #[test]
    fn test_zoned_seconds() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_seconds(&txt2ts("2010-01-02T00:00:00+00:00"), utc_tz.clone()),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            as_tz_seconds(&txt2ts("2010-01-02T00:00:00+00:00"), helsinki_tz.clone()),
            "2010-01-02 02:00:00"
        );

        // daylight saving time
        assert_eq!(
            as_tz_seconds(&txt2ts("2022-06-24T00:00:00+00:00"), utc_tz.clone()),
            "2022-06-24 00:00:00"
        );
        assert_eq!(
            as_tz_seconds(&txt2ts("2022-06-24T00:00:00+00:00"), helsinki_tz.clone()),
            "2022-06-24 03:00:00"
        );

        assert_eq!(
            as_tz_seconds(&txt2ts("2010-01-02T00:00:00+16:00"), utc_tz),
            "2010-01-01 08:00:00"
        );
        assert_eq!(
            as_tz_seconds(&txt2ts("2010-01-02T00:00:00+16:00"), helsinki_tz),
            "2010-01-01 10:00:00"
        );
    }

    #[test]
    fn test_zoned_full() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_full(&txt2ts("2010-01-02T00:00:00+00:00"), utc_tz.clone()),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            as_tz_full(&txt2ts("2010-01-02T00:00:00+00:00"), helsinki_tz.clone()),
            "2010-01-02 02:00:00"
        );

        // daylight saving time
        assert_eq!(
            as_tz_full(&txt2ts("2022-06-24T00:00:00+00:00"), utc_tz.clone()),
            "2022-06-24 00:00:00"
        );
        assert_eq!(
            as_tz_full(&txt2ts("2022-06-24T00:00:00+00:00"), helsinki_tz.clone()),
            "2022-06-24 03:00:00"
        );

        assert_eq!(
            as_tz_full(&txt2ts("2010-01-02T00:00:00.123+16:00"), utc_tz),
            "2010-01-01 08:00:00.123"
        );
        assert_eq!(
            as_tz_full(&txt2ts("2010-01-02T00:00:00.123+16:00"), helsinki_tz),
            "2010-01-01 10:00:00.123"
        );
    }

    #[test]
    fn test_zoned_date() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_date(&txt2ts("2010-01-02T00:00:00+00:00"), utc_tz.clone()),
            "2010-01-02"
        );
        assert_eq!(
            as_tz_date(
                &txt2ts("2009-12-31T21:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2009-12-31"
        );
        assert_eq!(
            as_tz_date(&txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz.clone()),
            "2010-01-01"
        );

        // daylight saving time
        assert_eq!(
            as_tz_date(&txt2ts("2022-06-24T00:00:00+00:00"), utc_tz),
            "2022-06-24"
        );
        assert_eq!(
            as_tz_date(&txt2ts("2022-06-23T21:00:00+00:00"), helsinki_tz),
            "2022-06-24"
        );
    }

    #[test]
    fn test_zoned_month() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_month(&txt2ts("2010-01-02T00:00:00+00:00"), utc_tz.clone()),
            "2010-01"
        );
        assert_eq!(
            as_tz_month(
                &txt2ts("2009-12-31T21:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2009-12"
        );
        assert_eq!(
            as_tz_month(&txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz.clone()),
            "2010-01"
        );

        // daylight saving time
        assert_eq!(
            as_tz_month(&txt2ts("2022-06-30T00:00:00+00:00"), utc_tz),
            "2022-06"
        );
        assert_eq!(
            as_tz_month(&txt2ts("2022-06-30T21:00:00+00:00"), helsinki_tz),
            "2022-07"
        );
    }

    #[test]
    fn test_zoned_year() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_year(&txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010"
        );
        assert_eq!(
            as_tz_year(
                &txt2ts("2009-12-31T21:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2009"
        );
        assert_eq!(
            as_tz_year(&txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz),
            "2010"
        );
    }

    #[test]
    fn test_zoned_week() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_iso_week(&txt2ts("2010-01-03T00:00:00+00:00"), utc_tz.clone()),
            "2009-W53"
        );
        assert_eq!(
            as_tz_iso_week(
                &txt2ts("2010-01-03T21:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2009-W53"
        );
        assert_eq!(
            as_tz_iso_week(&txt2ts("2010-01-04T00:00:00+00:00"), utc_tz.clone()),
            "2010-W01"
        );
        assert_eq!(
            as_tz_iso_week(&txt2ts("2010-01-03T22:00:00+00:00"), helsinki_tz.clone()),
            "2010-W01"
        );

        // daylight saving time
        assert_eq!(
            as_tz_iso_week(
                &txt2ts("2022-06-19T23:59:59.999999999+00:00"),
                utc_tz.clone()
            ),
            "2022-W24"
        );
        assert_eq!(
            as_tz_iso_week(
                &txt2ts("2022-06-19T20:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2022-W24"
        );
        assert_eq!(
            as_tz_iso_week(&txt2ts("2022-06-20T00:00:00+00:00"), utc_tz.clone()),
            "2022-W25"
        );
        assert_eq!(
            as_tz_iso_week(&txt2ts("2022-06-19T21:00:00+00:00"), helsinki_tz.clone()),
            "2022-W25"
        );
    }

    #[test]
    fn test_zoned_week_date() {
        let utc_tz = TimeZone::get("UTC").unwrap(/*:test:*/);
        let helsinki_tz = TimeZone::get("Europe/Helsinki").unwrap(/*:test:*/);

        // standard time
        assert_eq!(
            as_tz_iso_week_date(&txt2ts("2010-01-03T00:00:00+00:00"), utc_tz.clone()),
            "2009-W53-7"
        );
        assert_eq!(
            as_tz_iso_week_date(
                &txt2ts("2010-01-03T21:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2009-W53-7"
        );
        assert_eq!(
            as_tz_iso_week_date(&txt2ts("2010-01-04T00:00:00+00:00"), utc_tz.clone()),
            "2010-W01-1"
        );
        assert_eq!(
            as_tz_iso_week_date(&txt2ts("2010-01-03T22:00:00+00:00"), helsinki_tz.clone()),
            "2010-W01-1"
        );

        // daylight saving time
        assert_eq!(
            as_tz_iso_week_date(
                &txt2ts("2022-06-19T23:59:59.999999999+00:00"),
                utc_tz.clone()
            ),
            "2022-W24-7"
        );
        assert_eq!(
            as_tz_iso_week_date(
                &txt2ts("2022-06-19T20:59:59.999999999+00:00"),
                helsinki_tz.clone()
            ),
            "2022-W24-7"
        );
        assert_eq!(
            as_tz_iso_week_date(&txt2ts("2022-06-20T00:00:00+00:00"), utc_tz.clone()),
            "2022-W25-1"
        );
        assert_eq!(
            as_tz_iso_week_date(&txt2ts("2022-06-19T21:00:00+00:00"), helsinki_tz.clone()),
            "2022-W25-1"
        );
    }
}
