/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::kernel::Settings;
use crate::model::Commodity;
use crate::parser::parts::identifier::p_identifier;
use crate::parser::parts::number::p_number;
use crate::parser::{Stream, from_error};
use crate::tackler;
use rust_decimal::Decimal;
use std::sync::Arc;
use winnow::ascii::{space0, space1};
use winnow::combinator::{alt, opt};
use winnow::{ModalResult, Parser, seq};
/*
// The old ANTLR Grammar

posting:  indent account sp amount opt_unit? (opt_comment | opt_sp) NL;

opt_unit: sp unit opt_position?;

opt_position: opt_opening_pos
    | opt_opening_pos  closing_pos
    | closing_pos
    ;

opt_opening_pos: sp '{' opt_sp amount sp unit opt_sp '}';

closing_pos: sp ('@' | '=') sp amount sp unit;

amount: INT | NUMBER;

unit: ID;
 */

use winnow::combinator::cut_err;
use winnow::error::StrContext;
use winnow::error::StrContextValue;

struct Value<'s> {
    value: Decimal,
    commodity: &'s str,
}

fn p_opening_pos<'s>(is: &mut Stream<'s>) -> ModalResult<Value<'s>> {
    const CTX_LABEL: &str = "opening position";
    let m = seq!(
        _: space1,
        _: '{',
        _: space0,

        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("number"))),

        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space"))),

        cut_err(p_identifier)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("commodity name"))),

        _: space0,

        _: cut_err('}')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("closing '}'"))),
    )
    .parse_next(is)?;

    Ok(Value {
        value: m.0,
        commodity: m.1,
    }) // opening position is recognized but ignored at the moment
}

enum PriceType {
    TotalPrice,
    UnitPrice,
}

fn p_closing_pos<'s>(is: &mut Stream<'s>) -> ModalResult<(PriceType, Value<'s>)> {
    const CTX_LABEL: &str = "closing position";
    let m = seq!(
        _:space1,
        alt(('@', '=')),
        _:cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space"))),

        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("number"))),

        _:cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space"))),

        cut_err(p_identifier)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("commodity name"))),

    )
    .parse_next(is)?;

    let price_type = match m.0 {
        '=' => PriceType::TotalPrice,
        '@' => PriceType::UnitPrice,
        _ => unreachable!("IE: Unexpected token"),
    };

    Ok((
        price_type,
        Value {
            value: m.1,
            commodity: m.2,
        },
    ))
}

struct Positions<'s> {
    opening: Option<Value<'s>>,
    closing: Option<(PriceType, Value<'s>)>,
}
fn p_position<'s>(is: &mut Stream<'s>) -> ModalResult<Positions<'s>> {
    let m = alt((
        (p_opening_pos, p_closing_pos).map(|x| Positions {
            opening: Some(x.0),
            closing: Some(x.1),
        }),
        p_opening_pos.map(|x| Positions {
            opening: Some(x),
            closing: None,
        }),
        p_closing_pos.map(|x| Positions {
            opening: None,
            closing: Some(x),
        }),
    ))
    .parse_next(is)?;

    Ok(m)
}
fn p_unit<'s>(is: &mut Stream<'s>) -> ModalResult<(&'s str, Option<Positions<'s>>)> {
    #[rustfmt::skip]
    let m = (
        space1,
        p_identifier,
        opt(p_position)
    ).parse_next(is)?;

    Ok((m.1, m.2))
}

pub(crate) struct ValuePosition {
    pub(crate) post_amount: Decimal,
    pub(crate) txn_amount: Decimal,
    pub(crate) total_amount: bool,
    pub(crate) post_commodity: Arc<Commodity>,
    pub(crate) txn_commodity: Arc<Commodity>,
}

fn handle_posting_value(
    amount: Decimal,
    opt_unit: Option<&(&str, Option<Positions<'_>>)>,
    settings: &mut Settings,
) -> Result<ValuePosition, tackler::Error> {
    let post_commodity = match &opt_unit {
        Some(u) => settings.get_or_create_commodity(Some(u.0))?,
        None => settings.get_or_create_commodity(None)?,
    };

    let txn_commodity = match &opt_unit {
        Some(u) => {
            match &u.1 {
                Some(pos) => {
                    match &pos.closing {
                        Some(cp) => {
                            // Ok, we have position, so there must be closing position
                            // so, we have closing position, use its commodity
                            let val_pos_commodity =
                                settings.get_or_create_commodity(Some(cp.1.commodity))?;

                            if post_commodity.name == val_pos_commodity.name {
                                let em = format!(
                                    "Both commodities are same for value position [{}]",
                                    val_pos_commodity.name
                                );
                                //let msg = error_on_line(posting_ctx, &em);
                                return Err(em.into());
                            }
                            val_pos_commodity
                        }
                        None => settings.get_or_create_commodity(None)?,
                    }
                }
                None => {
                    // no position, use original unit
                    settings.get_or_create_commodity(Some(u.0))?
                }
            }
        }
        None => settings.get_or_create_commodity(None)?,
    };

    let post_amount = amount;

    let txn_amount: (Decimal, bool) = match &opt_unit {
        Some(u) => {
            match &u.1 {
                Some(pos) => {
                    if let Some(opening_pos) = &pos.opening {
                        if opening_pos.value.is_sign_negative() {
                            //let msg = error_on_line(posting_ctx, "Unit cost '{ ... }' is negative");
                            let msg = "Unit cost '{ ... }' is negative";
                            return Err(msg.into());
                        }
                    }
                    match &pos.closing {
                        Some(cp) => {
                            // ok, we have closing position
                            match cp.0 {
                                PriceType::TotalPrice => {
                                    // this is '=', e.g. total price
                                    let total_cost = cp.1.value;

                                    if (total_cost.is_sign_negative()
                                        && post_amount.is_sign_positive())
                                        || (post_amount.is_sign_negative()
                                            && total_cost.is_sign_positive())
                                    {
                                        //let msg = error_on_line(posting_ctx, "Total cost '=' has different sign than primary posting value");
                                        let msg = "Total cost '=' has different sign than primary posting value";
                                        return Err(msg.into());
                                    }
                                    (total_cost, true)
                                }
                                PriceType::UnitPrice => {
                                    // this is '@', e.g. unit price
                                    let unit_price = cp.1.value;
                                    if unit_price.is_sign_negative() {
                                        //let msg = error_on_line(
                                        //    posting_ctx,
                                        //    "Unit price '@' is negative",
                                        //);
                                        let msg = "Unit price '@' is negative";
                                        return Err(msg.into());
                                    }
                                    (post_amount * unit_price, false)
                                }
                            }
                        }
                        None => {
                            // plain value, no closing position
                            (post_amount, false)
                        }
                    }
                }
                None => {
                    // No position at all
                    (post_amount, false)
                }
            }
        }
        None => (post_amount, false),
    };

    Ok(ValuePosition {
        post_amount,
        txn_amount: txn_amount.0,
        total_amount: txn_amount.1,
        post_commodity,
        txn_commodity,
    })
}

pub(crate) fn parse_posting_value(is: &mut Stream<'_>) -> ModalResult<ValuePosition> {
    #[rustfmt::skip]
    let m: (Decimal, Option<(&str, Option<Positions<'_>>)>) =
        seq!(
            p_number,
            opt(p_unit,)
        ).parse_next(is)?;

    match handle_posting_value(m.0, m.1.as_ref(), is.state) {
        Ok(v) => Ok(v),
        Err(err) => Err(from_error(is, err.as_ref())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    fn test_parse_posting_value() {
        #[rustfmt::skip]
        let pok_values = vec![
            (indoc!(
               "|1.23
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 ACME·INC
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 ACME·INC @ 1.23 €
                |"
            ).strip_margin(),),

            (indoc!(
               "|1.23 ACME·INC = 1.23 €
                |"
            ).strip_margin(),),

            (indoc!(
               "|1.23 {4.56} ACME·INC
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 {4.56 $} ACME·INC
                |"
            ).strip_margin(),),

            (indoc!(
               "|1.23 {4.56} ACME·INC = 5.67 £
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 {4.56 $} ACME·INC = 5.67 £
                |"
            ).strip_margin(),),


            (indoc!(
               "|1.23 {4.56} ACME·INC @ 5.67 £
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 {4.56 $} ACME·INC @ 5.67 £
                |"
            ).strip_margin(),),

            (indoc!(
               "|1.23\tACME·INC
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23 \t ACME·INC\t \n\
                |"
            ).strip_margin(),),
            (indoc!(
               "|1.23\t \t{\t \t4.56\t \t$\t \t}\t \tACME·INC\t \t@\t \t5.67\t \t£\t \t
                |"
            ).strip_margin(),),
        ];

        let mut count = 0;
        for t in &pok_values {
            let mut settings = Settings::default();
            let mut is = Stream {
                input: t.0.as_str(),
                state: &mut settings,
            };

            let res = parse_posting_value(&mut is);
            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: {}\n",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, pok_values.len());
    }
}
