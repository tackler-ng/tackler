/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::{BalanceType, Scale};
use crate::kernel::Settings;
use crate::kernel::price_lookup::PriceLookup;
use crate::model::Commodity;
use crate::tackler;
use jiff::tz::TimeZone;
use std::sync::Arc;
use tackler_api::txn_ts::{GroupBy, TimestampStyle};

#[derive(Debug, Clone)]
pub struct BalanceSettings {
    pub(crate) title: String,
    pub(crate) bal_type: BalanceType,
    pub(crate) ras: Vec<String>,
    pub(crate) scale: Scale,
    pub(crate) inverted: bool,
    pub(crate) report_commodity: Option<Arc<Commodity>>,
    pub(crate) price_lookup: PriceLookup,
}

impl TryFrom<&Settings> for BalanceSettings {
    type Error = tackler::Error;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(BalanceSettings {
            title: settings.report.balance.title.clone(),
            bal_type: settings.report.balance.bal_type.clone(),
            ras: settings.get_balance_ras(),
            scale: settings.report.scale.clone(),
            inverted: settings.inverted,
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BalanceGroupSettings {
    pub title: String,
    pub bal_type: BalanceType,
    pub ras: Vec<String>,
    pub group_by: GroupBy,
    pub report_tz: TimeZone,
    pub scale: Scale,
    pub inverted: bool,
    pub report_commodity: Option<Arc<Commodity>>,
    pub price_lookup: PriceLookup,
}

impl TryFrom<&Settings> for BalanceGroupSettings {
    type Error = tackler::Error;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        let bgs = BalanceGroupSettings {
            title: settings.report.balance_group.title.clone(),
            bal_type: settings.report.balance_group.bal_type.clone(),
            ras: settings.get_balance_group_ras(),
            group_by: settings.report.balance_group.group_by,
            report_tz: settings.report.tz.clone(),
            scale: settings.report.scale.clone(),
            inverted: settings.inverted,
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
        };
        Ok(bgs)
    }
}

impl From<BalanceGroupSettings> for BalanceSettings {
    fn from(bgs: BalanceGroupSettings) -> BalanceSettings {
        BalanceSettings {
            title: String::default(),
            bal_type: bgs.bal_type.clone(),
            ras: bgs.ras.clone(),
            scale: bgs.scale.clone(),
            inverted: bgs.inverted,
            report_commodity: bgs.report_commodity.clone(),
            price_lookup: bgs.price_lookup.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterSettings {
    pub title: String,
    pub ras: Vec<String>,
    pub report_tz: TimeZone,
    pub report_commodity: Option<Arc<Commodity>>,
    pub(crate) scale: Scale,
    pub inverted: bool,
    pub price_lookup: PriceLookup,
    pub timestamp_style: TimestampStyle,
}

impl TryFrom<&Settings> for RegisterSettings {
    type Error = tackler::Error;

    fn try_from(settings: &Settings) -> Result<RegisterSettings, tackler::Error> {
        let rs = RegisterSettings {
            title: settings.report.register.title.clone(),
            ras: settings.get_register_ras(),
            report_tz: settings.report.tz.clone(),
            scale: settings.report.scale.clone(),
            inverted: settings.inverted,
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
            timestamp_style: settings.report.register.timestamp_style,
        };
        Ok(rs)
    }
}
