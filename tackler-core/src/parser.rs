/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
pub use crate::parser::pricedb_parser::{pricedb_from_file, pricedb_from_str};
pub use crate::parser::tackler_txns::git_to_txns;
pub use crate::parser::tackler_txns::paths_to_txns;
pub use crate::parser::tackler_txns::string_to_txns;
use winnow::error::{ErrMode, FromExternalError};

mod error;
mod pricedb_parser;
mod tackler_parser;
mod tackler_txns;

use crate::kernel::settings::Settings;
use crate::parser::error::TacklerTxnError;
use winnow::Stateful;

pub(crate) mod parts;

pub(crate) type Stream<'is> = Stateful<&'is str, &'is mut Settings>;

pub(crate) fn make_semantic_error<
    'is,
    E: winnow::error::FromExternalError<Stream<'is>, TacklerTxnError>,
>(
    is: &mut Stream<'is>,
    msg: &str,
) -> ErrMode<E> {
    ErrMode::from_external_error(is, TacklerTxnError::semantic_error(msg)).cut()
}

pub(crate) fn from_error<
    'is,
    E: winnow::error::FromExternalError<Stream<'is>, TacklerTxnError>,
    SE: std::error::Error + ?Sized,
>(
    is: &mut Stream<'is>,
    err: &SE,
) -> ErrMode<E> {
    ErrMode::from_external_error(
        is,
        TacklerTxnError::semantic_error(err.to_string().as_str()),
    )
    .cut()
}

/*
 * TODO: This logic should be 1:1 with TxnLexer.g4
 *       (ID, SUBID and NameChar + NameStartChar)
 * Real account names are coming through parser+lexer,
 * So these are validating Chart-of-Xyz config/settings
 * entries (accounts, tags, commodities).
 * E.g. checking these is a nicety for user
 * (warn about invalid Chart-Of-Xyz).
 */
#[inline]
fn illegal_characters(c: char) -> bool {
    c == ':' || c.is_whitespace()
}

// todo: this is too relaxed
fn is_valid_id_start_char(c: char) -> bool {
    !(c.is_ascii_digit() || c == ':' || c == '-' || c == '_' || c == '·' || c.is_whitespace())
}

// this is fine, once is_valid_id_start_char is fixed
fn is_valid_sub_id_start_char(c: char) -> bool {
    c.is_numeric() || is_valid_id_start_char(c)
}

// todo: this is too relaxed
pub fn is_valid_id(token: &str) -> bool {
    !token.is_empty()
        && token.starts_with(is_valid_id_start_char)
        && !token.contains(illegal_characters)
}

// todo: this is too relaxed
pub fn is_valid_sub_id(token: &str) -> bool {
    !token.is_empty()
        && token.starts_with(is_valid_sub_id_start_char)
        && !token.contains(illegal_characters)
}

#[cfg(test)]
mod tests;
