/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config::Scale;
use crate::kernel::RegisterSettings;
use crate::kernel::price_lookup::PriceLookup;
use crate::math::format::format_with_scale;
use crate::model::{Commodity, Posting, Transaction};
use jiff::Zoned;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use std::cmp::{Ordering, max};
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RegisterPosting<'a> {
    pub post: &'a Posting,
    pub amount: Decimal,
    pub target_commodity: Arc<Commodity>,
    pub rate: Option<Decimal>,
}

impl RegisterPosting<'_> {
    fn is_commodity_conv(&self) -> bool {
        self.target_commodity != self.post.acctn.comm
    }
}

impl Eq for RegisterPosting<'_> {}

impl PartialEq<Self> for RegisterPosting<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.post.acctn.atn == other.post.acctn.atn
    }
}

impl PartialOrd<Self> for RegisterPosting<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RegisterPosting<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.post.acctn.cmp(&other.post.acctn)
    }
}

impl Display for RegisterPosting<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.post.acctn.atn.account, self.post.amount, self.amount
        )
    }
}

#[derive(Debug)]
pub(crate) struct RegisterEntry<'a> {
    pub txn: &'a Transaction,
    pub posts: Vec<RegisterPosting<'a>>,
}

impl RegisterEntry<'_> {
    pub(crate) fn fmt_with_cfg(
        &self,
        ts_fmtr: fn(&Zoned, TimeZone) -> String,
        tz: TimeZone,
        reg_cfg: &RegisterSettings,
    ) -> String {
        fn amount_to_string(amount: &Decimal, scale: &Scale, width: usize) -> String {
            let amount_txt = format_with_scale(0, amount, scale);
            if amount.is_sign_positive() && amount_txt.chars().count() >= width {
                format!(" {}", amount_txt)
            } else {
                amount_txt
            }
        }

        let indent = " ".repeat(12);
        let mut line_len = 0;
        let mut reg_entry_txt = self.txn.header.to_string_with_indent(&indent, ts_fmtr, tz);

        let filler_width = match &reg_cfg.price_lookup {
            PriceLookup::AtTheTimeOfTxn => 20,
            PriceLookup::LastPriceDbEntry => 8,
            PriceLookup::GivenTime(_) => 8,
            PriceLookup::None => 0,
        };

        for p in &self.posts {
            let (comm, base_comm, width) = if p.is_commodity_conv() {
                let (base_comm, width) = match p.rate {
                    Some(r) => (format!(" {} @ {}", p.post.acctn.comm, r), 20),
                    None => (format!(" {}", p.post.acctn.comm), 8),
                };
                (&p.target_commodity, base_comm, width)
            } else {
                (&p.post.acctn.comm, String::default(), filler_width)
            };

            let line = format!(
                "{}{:<33}{:>18}{:<w$} {:>18}{}",
                indent,
                p.post.acctn.atn.account,
                amount_to_string(&p.post.amount, &reg_cfg.scale, 18),
                base_comm,
                amount_to_string(&p.amount, &reg_cfg.scale, 18),
                match &comm.is_any() {
                    true => format!(" {}", comm.name),
                    false => String::new(),
                },
                w = width,
            );
            line_len = max(line_len, line.chars().count());
            let _ = writeln!(reg_entry_txt, "{}", line);
        }
        let _ = writeln!(reg_entry_txt, "{}", "-".repeat(line_len));
        reg_entry_txt
    }
}
