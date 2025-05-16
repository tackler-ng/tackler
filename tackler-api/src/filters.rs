/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

//! Transaction filters
//!
//! The filtering logic is implemented in [`tackler-core`].
//!
//! [`tackler-core`]: ../../tackler_core/index.html
mod filter_definition;
pub mod logic;
pub mod posting;
pub mod txn;

pub use crate::filters::filter_definition::FilterDefZoned;
pub use crate::filters::filter_definition::FilterDefinition;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Error, Formatter};

use logic::TxnFilterAND;
use logic::TxnFilterNOT;
use logic::TxnFilterOR;

use txn::TxnFilterBBoxLatLon;
use txn::TxnFilterBBoxLatLonAlt;
use txn::TxnFilterTxnCode;
use txn::TxnFilterTxnComments;
use txn::TxnFilterTxnDescription;
use txn::TxnFilterTxnTSBegin;
use txn::TxnFilterTxnTSEnd;
use txn::TxnFilterTxnTags;
use txn::TxnFilterTxnUUID;

use posting::TxnFilterPostingAccount;
use posting::TxnFilterPostingAmountEqual;
use posting::TxnFilterPostingAmountGreater;
use posting::TxnFilterPostingAmountLess;
use posting::TxnFilterPostingComment;
use posting::TxnFilterPostingCommodity;

/// fmt with prefix indent
///
pub trait IndentDisplay {
    /// format with indent
    ///
    /// # Errors
    ///
    /// Returns `Err` if formatter fails
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result;
}

/// Enum of all Transaction filters.
///
/// See [logic], [txn] and [posting] modules
/// for the documentation of transaction filters.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TxnFilter {
    // Nullary test filters
    #[doc(hidden)]
    NullaryTRUE(NullaryTRUE),
    #[doc(hidden)]
    NullaryFALSE(NullaryFALSE),

    // Logic filters
    #[doc(hidden)]
    TxnFilterAND(TxnFilterAND),
    #[doc(hidden)]
    TxnFilterOR(TxnFilterOR),
    #[doc(hidden)]
    TxnFilterNOT(TxnFilterNOT),

    // TXN Header filters
    #[doc(hidden)]
    TxnFilterTxnTSBegin(TxnFilterTxnTSBegin),
    #[doc(hidden)]
    TxnFilterTxnTSEnd(TxnFilterTxnTSEnd),
    #[doc(hidden)]
    TxnFilterTxnCode(TxnFilterTxnCode),
    #[doc(hidden)]
    TxnFilterTxnDescription(TxnFilterTxnDescription),
    #[doc(hidden)]
    TxnFilterTxnUUID(TxnFilterTxnUUID),
    #[doc(hidden)]
    TxnFilterBBoxLatLon(TxnFilterBBoxLatLon),
    #[doc(hidden)]
    TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt),
    #[doc(hidden)]
    TxnFilterTxnTags(TxnFilterTxnTags),
    #[doc(hidden)]
    TxnFilterTxnComments(TxnFilterTxnComments),

    // TXN Postings
    #[doc(hidden)]
    TxnFilterPostingAccount(TxnFilterPostingAccount),
    #[doc(hidden)]
    TxnFilterPostingComment(TxnFilterPostingComment),
    #[doc(hidden)]
    TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual),
    #[doc(hidden)]
    TxnFilterPostingAmountLess(TxnFilterPostingAmountLess),
    #[doc(hidden)]
    TxnFilterPostingAmountGreater(TxnFilterPostingAmountGreater),
    #[doc(hidden)]
    TxnFilterPostingCommodity(TxnFilterPostingCommodity),
}

/*
impl Display for TxnFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.i_fmt("", f)
    }
}
*/
impl IndentDisplay for TxnFilter {
    fn i_fmt(&self, indent: &str, tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // specials
            TxnFilter::NullaryTRUE(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::NullaryFALSE(tf) => tf.i_fmt(indent, tz, f),

            // logic filters
            TxnFilter::TxnFilterAND(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterOR(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterNOT(tf) => tf.i_fmt(indent, tz, f),

            // txn header filters
            TxnFilter::TxnFilterTxnTSBegin(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnTSEnd(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnCode(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnDescription(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnUUID(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterBBoxLatLon(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterBBoxLatLonAlt(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnTags(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterTxnComments(tf) => tf.i_fmt(indent, tz, f),

            // posting filters
            TxnFilter::TxnFilterPostingAccount(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterPostingComment(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterPostingAmountEqual(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterPostingAmountLess(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterPostingAmountGreater(tf) => tf.i_fmt(indent, tz, f),
            TxnFilter::TxnFilterPostingCommodity(tf) => tf.i_fmt(indent, tz, f),
        }
    }
}

/// Special always true filter (e.g. selects always)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NullaryTRUE {}

impl IndentDisplay for NullaryTRUE {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}All pass")
    }
}

/// Special always false filter (e.g. selects nothing)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NullaryFALSE {}

impl IndentDisplay for NullaryFALSE {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}None pass")
    }
}

#[allow(clippy::needless_pass_by_value)]
fn logic_filter_indent_fmt(
    op: &str,
    indent: &str,
    tz: TimeZone,
    filters: &[TxnFilter],
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let new_ident = format!("{indent}  ");

    writeln!(f, "{indent}{op}")?;
    let result: Result<Vec<()>, Error> = filters
        .iter()
        .map(|tf| tf.i_fmt(&new_ident, tz.clone(), f))
        .collect();

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn posting_filter_indent_fmt(
    indent: &str,
    target: &str,
    regex: &str,
    op: &str,
    amount: &Decimal,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let my_indent = format!("{indent}  ");
    writeln!(f, "{indent}{target}")?;
    writeln!(f, "{my_indent}account: \"{regex}\"")?;
    writeln!(f, "{my_indent}amount {op} {amount}")
}
