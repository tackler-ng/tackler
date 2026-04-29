/*
 * Tackler-NG 2025-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub(crate) enum TacklerTxnError {
    SemanticError { msg: String },
    TxnDataError { msg: String },
}

impl TacklerTxnError {
    pub(crate) fn semantic_error(msg: &str) -> Self {
        Self::SemanticError {
            msg: msg.to_string(),
        }
    }
    pub(crate) fn txn_data_error(msg: &str) -> Self {
        Self::TxnDataError {
            msg: msg.to_string(),
        }
    }
}

impl StdError for TacklerTxnError {
    fn description(&self) -> &'static str {
        "Semantic Tackler Txn error"
    }
}

impl Display for TacklerTxnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TacklerTxnError::SemanticError { msg } => {
                write!(f, "semantic error - {msg}")
            }
            TacklerTxnError::TxnDataError { msg } => {
                write!(f, "data error - {msg}")
            }
        }
    }
}
