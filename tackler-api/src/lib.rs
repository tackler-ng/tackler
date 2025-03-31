/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Tackler API components

pub mod metadata;

pub mod filters;
pub mod location;
pub mod reports;
pub mod txn_header;
pub mod txn_ts;

/// Generic Tackler namespace
pub mod tackler {
    /// Generic error type
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
}
