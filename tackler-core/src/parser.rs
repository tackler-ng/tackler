/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
pub use crate::parser::pricedb_parser::{pricedb_from_file, pricedb_from_str};
pub use crate::parser::tackler_txns::git_to_txns;
pub use crate::parser::tackler_txns::paths_to_txns;
pub use crate::parser::tackler_txns::string_to_txns;
use std::fmt::Write;
use winnow::error::{ErrMode, FromExternalError};

mod error;
mod pricedb_parser;
mod tackler_parser;
mod tackler_txns;

use crate::kernel::settings::Settings;
use crate::parser::error::TacklerTxnError;
use crate::parser::parts::identifier::{parse_identifier, parse_multi_part_id};
use crate::tackler;
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

/// Check if id is a valid identifier, e.g., commodity name
///
/// # Errors
/// Returns error in case the identifier is not valid
pub fn is_valid_identifier(id: &str) -> Result<bool, tackler::Error> {
    let mut ptr = id;
    let res = parse_identifier(&mut ptr);

    if res.is_err() || !ptr.is_empty() {
        let mut msg = format!("Invalid identifier '{id}'. ");
        match res {
            Err(e) => {
                let _ = write!(msg, "Error was {e}");
            }
            Ok(_) => {
                let _ = write!(msg, "Extra characters: '{ptr}')");
            }
        }
        return Err(msg.into());
    }
    Ok(true)
}

/// Check if id is a valid name, e.g., account name
///
/// # Errors
/// Returns error in case the account name is not valid
pub fn is_valid_name(name: &str) -> Result<bool, tackler::Error> {
    let mut ptr = name;
    let res = parse_multi_part_id(&mut ptr);

    if res.is_err() || !ptr.is_empty() {
        let mut msg = format!("Invalid name '{name}'. ");
        match res {
            Err(e) => {
                let _ = write!(msg, "Error was {e}");
            }
            Ok(_) => {
                let _ = write!(msg, "Extra characters: '{ptr}')");
            }
        }
        return Err(msg.into());
    }
    Ok(true)
}

#[cfg(test)]
mod tests;
