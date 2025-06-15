/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module contains the overlap
//! configuration items to be used e.g. with CLI

use crate::config::{PriceLookupType, StorageType};
use crate::kernel::settings::GitInputSelector;
use std::path::PathBuf;

/// Collections of all configuration overlaps
#[derive(Debug, Default, Clone)]
pub struct OverlapConfig {
    /// Audit mode related overlaps
    pub audit: AuditOverlap,
    /// Strict mode related overlaps
    pub strict: StrictOverlap,
    /// input related overlaps
    pub storage: StorageOverlap,
    /// Price DB and conversion related overlaps
    pub price: PriceOverlap,
    /// Reporting related overlaps
    pub report: ReportOverlap,
    /// Target (reports, exports) related overlaps
    pub target: TargetOverlap,
}

#[derive(Debug, Default, Clone)]
pub struct StorageOverlap {
    pub storage_type: Option<StorageType>,
    pub input: Option<InputOverlap>,
}

/// Input related overlap
#[derive(Debug, Clone)]
pub enum InputOverlap {
    File(FileInputOverlap),
    Fs(FsInputOverlap),
    Git(GitInputOverlap),
}

#[derive(Debug, Clone)]
pub struct FileInputOverlap {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FsInputOverlap {
    pub path: Option<String>,
    pub dir: Option<String>,
    pub ext: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GitInputOverlap {
    pub repo: Option<String>,
    pub dir: Option<String>,
    pub git_ref: Option<GitInputSelector>,
    pub ext: Option<String>,
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
    /// Are the report values inverted?
    pub inverted: bool,
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
