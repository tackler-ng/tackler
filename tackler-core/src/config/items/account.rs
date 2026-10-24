/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::items::Timestamp;
use crate::config::spec_items::AccountSpec;
use crate::config::toml_datetime_to_zoned;
use crate::tackler;
use jiff::Zoned;

#[derive(Debug, Eq, PartialEq, Default)]
pub(crate) struct Account {
    spans: Option<Vec<(Option<Zoned>, Option<Zoned>)>>,
    pub(crate) spec: AccountSpec,
}
impl Account {
    pub(crate) fn from(name: &str) -> Self {
        Account {
            spans: None,
            spec: AccountSpec::from_str(name),
        }
    }
    pub(crate) fn try_with(acc_spec: AccountSpec, ts: &Timestamp) -> Result<Self, tackler::Error> {
        let mut spans = if let Some(raw_spans) = &acc_spec.spans {
            assert!(acc_spec.open.is_none());
            assert!(acc_spec.close.is_none());

            let mut v = raw_spans
                .iter()
                .map(|i| -> Result<(Zoned, Zoned), tackler::Error> {
                    let open = toml_datetime_to_zoned(&i.open, ts)?;
                    let close = toml_datetime_to_zoned(&i.close, ts)?;
                    Ok((open, close))
                })
                .collect::<Result<Vec<(Zoned, Zoned)>, tackler::Error>>()?;

            v.sort_by(|a, b| a.0.cmp(&b.0));

            let overlapping = v
                .iter()
                .fold((false, None), |c: (bool, Option<&Zoned>), i| {
                    if c.0 {
                        (true, c.1)
                    } else if let Some(close) = c.1 {
                        if close.gt(&i.0) {
                            (true, Some(close))
                        } else {
                            (false, Some(&i.1))
                        }
                    } else {
                        (false, Some(&i.1))
                    }
                });

            if overlapping.0 {
                let msg = format!("Account '{}' has overlapping spans items", acc_spec.name);
                return Err(msg.into());
            }

            let ov = v.into_iter().map(|i| (Some(i.0), Some(i.1))).collect();
            Some(ov)
        } else if acc_spec.open.is_some() || acc_spec.close.is_some() {
            assert!(acc_spec.spans.is_none());

            let open = acc_spec
                .open
                .map(|open| toml_datetime_to_zoned(&open, ts))
                .transpose()?;
            let close = acc_spec
                .close
                .map(|close| toml_datetime_to_zoned(&close, ts))
                .transpose()?;

            Some(vec![(open, close)])
        } else {
            None
        };

        if let Some(spans) = &mut spans {
            let inverted = spans.iter().position(|v| {
                if let Some(open) = &v.0
                    && let Some(close) = &v.1
                {
                    close.lt(open)
                } else {
                    false
                }
            });
            if inverted.is_some() {
                let msg = format!("Account '{}' has inverted span", acc_spec.name);
                return Err(msg.into());
            }
            let no_gap = spans.iter().position(|v| {
                if let Some(open) = &v.0
                    && let Some(close) = &v.1
                {
                    open.eq(close)
                } else {
                    false
                }
            });
            if no_gap.is_some() {
                let msg = format!("Account '{}' has no-span span", acc_spec.name);
                return Err(msg.into());
            }
        }

        let a: Account = Account {
            spans,
            spec: acc_spec,
        };
        Ok(a)
    }
    pub(crate) fn is_open(&self, ts: &Zoned) -> bool {
        if let Some(spans) = &self.spans {
            spans
                .iter()
                .position(|i| {
                    let open = i.0.as_ref();
                    let close = i.1.as_ref();
                    let open_ok = if let Some(open) = open {
                        ts.gt(open) || ts.eq(open)
                    } else {
                        true // It's open from the beginning of time
                    };
                    let close_ok = if let Some(close) = close {
                        ts.lt(close)
                    } else {
                        true // It's open until the end of time
                    };
                    open_ok && close_ok
                })
                .is_some()
        } else {
            // Plain old account, no open-close information, it's open
            true
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test: 153b0817-7c48-4050-980f-7a39cf68ced3
    // desc: AccountSpec combinations
    #[allow(clippy::too_many_lines)]
    fn test_ok_conf() {
        let confs = [
            (
                r#"
                name = "a:b:c"
                "#,
                vec![("2026-10-20T01:02:03[UTC]", true)],
            ),
            (
                r#"
                name = "a:b:c"
                open = 2026-10-20
                close = 2026-10-21
                "#,
                vec![
                    ("2026-10-20T00:00:00[UTC]", true),
                    ("2026-10-19T00:00:00[UTC]", false),
                    ("2026-10-21T00:00:00[UTC]", false),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                close = 2026-10-21
                "#,
                vec![
                    ("2026-10-19T00:00:00[UTC]", true),
                    ("2026-10-21T00:00:00[UTC]", false),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                open = 2026-10-20
                "#,
                vec![
                    ("2026-10-19T00:00:00[UTC]", false),
                    ("2026-10-21T00:00:00[UTC]", true),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2025-10-20, close = 2025-10-21 },
                    { open = 2026-10-20, close = 2026-10-21 },
                ]
                "#,
                vec![
                    ("2025-10-19T00:00:00[UTC]", false),
                    ("2025-10-20T01:02:03[UTC]", true),
                    ("2025-10-21T00:00:00[UTC]", false),
                    ("2026-06-21T00:00:00[UTC]", false),
                    ("2026-10-19T00:00:00[UTC]", false),
                    ("2026-10-20T01:02:03[UTC]", true),
                    ("2026-10-21T00:00:00[UTC]", false),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2025-01-01, close = 2026-01-01 },
                    { open = 2026-01-01, close = 2027-01-01 },
                ]
                "#,
                vec![
                    ("2024-12-31T00:00:00[UTC]", false),
                    ("2026-01-01T00:00:00[UTC]", true),
                    ("2027-01-01T00:00:00[UTC]", false),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                open = 2026-10-20 12:13:14.999
                close = 2026-10-21
                "#,
                vec![
                    ("2026-10-20T12:13:14.998[UTC]", false),
                    ("2026-10-20T12:13:14.999[UTC]", true),
                    ("2026-10-21T00:00:00[UTC]", false),
                ],
            ),
            (
                r#"
                name = "a:b:c"
                open = 2026-10-20T12:13:14.999+02:00
                close = 2026-10-21
                "#,
                vec![
                    ("2026-10-20T10:13:14.998[UTC]", false),
                    ("2026-10-20T10:13:14.999[UTC]", true),
                    ("2026-10-21T00:00:00[UTC]", false),
                ],
            ),
        ];
        for (count, conf) in (1..).zip(confs) {
            let acc_spec: AccountSpec = toml::from_str(conf.0).unwrap(/*:test:*/);
            let res = Account::try_with(acc_spec, &Timestamp::default());

            let acc = res.unwrap(/*:test:*/);

            for (span_number, span) in (1..).zip(&conf.1) {
                let ts: Zoned = span.0.parse().unwrap(/*:test:*/);
                assert_eq!(
                    acc.is_open(&ts),
                    span.1,
                    "\nOffending test number: {count}, span number: {span_number}\n",
                );
            }
        }
    }
    #[test]
    // test: 444f381b-7d23-420b-a7ab-614f1b9075f0
    // desc: AccountSpecs with logic errors
    #[allow(clippy::too_many_lines)]
    fn test_err_conf() {
        let confs = [
            (
                r#"
                name = "a:b:c"
                open = 2026-10-21
                close = 12:13:14.999
                "#,
                "datetime",
            ),
            (
                r#"
                name = "a:b:c"
                open = 2026-10-21
                close = 2026-10-20
                "#,
                "inverted",
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2024-01-01, close = 2024-02-01 },
                    { open = 2025-02-01, close = 2025-01-01 },
                    { open = 2026-01-01, close = 2026-02-01 },
                ]
                "#,
                "inverted",
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2024-01-01, close = 2024-02-01 },
                    { open = 2026-01-01, close = 2026-02-01 },
                    { open = 2025-01-01, close = 2026-02-01 },
                ]
                "#,
                "overlapping",
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2024-01-01, close = 2024-02-01 },
                    { open = 2026-01-01, close = 2026-02-01 },
                    { open = 2024-01-01, close = 2025-02-01 },
                ]
                "#,
                "overlapping",
            ),
            (
                r#"
                name = "a:b:c"
                spans = [
                    { open = 2025-01-01, close = 2025-02-01 },
                    { open = 2026-01-01, close = 2026-02-01 },
                    { open = 2026-01-02, close = 2026-01-03 },
                ]
                "#,
                "overlapping",
            ),
        ];
        for (count, conf) in (1..).zip(confs) {
            let acc_spec: AccountSpec = toml::from_str(conf.0).unwrap(/*:test:*/);
            let res = Account::try_with(acc_spec, &Timestamp::default());

            if let Err(e) = res {
                assert!(
                    e.to_string().contains(conf.1),
                    "\nOffending test number: {count}\n",
                );
            } else {
                panic!("Test case should be Err");
            }
        }
    }
}
