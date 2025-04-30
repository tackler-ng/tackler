/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module contains the overlap
//! configuration items to be used e.g. with CLI

use crate::config::PriceLookupType;
use std::path::PathBuf;

/// Collections of all configuration overlaps
#[derive(Debug, Default, Clone)]
pub struct OverlapConfig {
    /// Audit mode related overlaps
    pub audit: AuditOverlap,
    /// Strict mode related overlaps
    pub strict: StrictOverlap,
    /// Price DB and conversion related overlaps
    pub price: PriceOverlap,
    /// Reporting related overlaps
    pub report: ReportOverlap,
    /// Target (reports, exports) related overlaps
    pub target: TargetOverlap,
}

/// Audit mode related overlaps
#[derive(Debug, Default, Clone)]
pub struct AuditOverlap {
    /// Audit-mode
    pub mode: Option<bool>,
}

/// Strict mode related overlaps
#[derive(Debug, Default, Clone)]
pub struct StrictOverlap {
    /// Strict-mode
    pub mode: Option<bool>,
}

/// Price overlap configuration
#[derive(Debug, Default, Clone)]
pub struct PriceOverlap {
    /// Price DB path
    pub db_path: Option<PathBuf>,
    /// Price lookup type
    pub lookup_type: Option<PriceLookupType>,
    /// Price lookup "before" time(stamp)
    pub before_time: Option<String>,
}

/// Report overlap configuration
#[derive(Debug, Default, Clone)]
pub struct ReportOverlap {
    /// Report commodity
    pub commodity: Option<String>,
    /// Default reporting account
    pub account_overlap: Option<Vec<String>>,
    /// Group-By operator
    pub group_by: Option<String>,
}

/// Target (reports, exports) overlap configuration
#[derive(Debug, Default, Clone)]
pub struct TargetOverlap {
    /// reports
    pub reports: Option<Vec<String>>,
    /// exports
    pub exports: Option<Vec<String>>,
    /// Report output formats
    pub formats: Option<Vec<String>>,
}
