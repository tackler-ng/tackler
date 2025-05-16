/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::{
    Commodity, Transaction, TxnAccount, TxnRefs,
    price_entry::{PriceDb, PriceEntry},
};
use itertools::Itertools;
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};
use rust_decimal::Decimal;
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};
use tackler_api::metadata::items::{PriceRecord, PriceRecords};

#[derive(Debug)]
enum Cache<'p> {
    Fixed(HashMap<Arc<Commodity>, (Zoned, Decimal)>),
    Timed(HashMap<Arc<Commodity>, Vec<&'p PriceEntry>>),
}
impl Cache<'_> {
    fn is_empty(&self) -> bool {
        match &self {
            Cache::Fixed(map) => map.is_empty(),
            Cache::Timed(map) => map.is_empty(),
        }
    }
}

#[derive(Debug)]
pub struct PriceLookupCtx<'p> {
    cache: Cache<'p>,
    in_commodity: Option<Arc<Commodity>>,
}

impl Default for PriceLookupCtx<'_> {
    fn default() -> Self {
        PriceLookupCtx {
            cache: Cache::Fixed(HashMap::new()),
            in_commodity: None,
        }
    }
}

impl PriceLookupCtx<'_> {
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl PriceLookupCtx<'_> {
    pub fn metadata(&self) -> PriceRecords {
        let rates = if let Some(target) = self.in_commodity.clone() {
            match &self.cache {
                Cache::Fixed(map) => map
                    .iter()
                    .sorted_by_key(|k| *k)
                    .map(|(k, v)| PriceRecord {
                        ts: Some(v.0.clone()),
                        source: k.name.clone(),
                        rate: Some(format!("{}", v.1)),
                        target: target.name.clone(),
                    })
                    .collect(),
                Cache::Timed(map) => map
                    .iter()
                    .sorted_by_key(|k| *k)
                    .map(|(k, _)| PriceRecord {
                        ts: None,
                        source: k.name.clone(),
                        rate: None,
                        target: target.name.clone(),
                    })
                    .collect(),
            }
        } else {
            Vec::new()
        };

        PriceRecords { rates }
    }

    #[inline]
    pub(crate) fn convert_prices<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
    ) -> Box<dyn Iterator<Item = (TxnAccount, Decimal, Option<Decimal>)> + 'r>
    where
        's: 'r,
        't: 'r,
    {
        match &self.in_commodity {
            Some(comm) => Box::new(self.convert_prices_inner(txn, comm.clone())),
            None => Box::new(txn.posts.iter().map(|p| (p.acctn.clone(), p.amount, None))),
        }
    }

    fn convert_prices_inner<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
        in_commodity: Arc<Commodity>,
    ) -> Box<dyn Iterator<Item = (TxnAccount, Decimal, Option<Decimal>)> + 'r>
    where
        's: 'r,
        't: 'r,
    {
        Box::new(txn.posts.iter().map(move |p| {
            if p.acctn.comm.is_any() {
                let mut acctn = p.acctn.clone();
                let mut amount = p.amount;
                match &self.cache {
                    Cache::Fixed(cache) => {
                        if let Some(c) = cache.get(&p.acctn.comm) {
                            acctn.comm = in_commodity.clone();
                            amount *= c.1;
                        }
                        (acctn, amount, None)
                    }
                    Cache::Timed(comm_cache) => {
                        if let Some(cache) = comm_cache.get(&p.acctn.comm) {
                            let i = match cache.binary_search_by_key(
                                &(&txn.header.timestamp, &p.acctn.comm),
                                |e| (&e.timestamp, &e.base_commodity),
                            ) {
                                Ok(i) => Some(i),
                                Err(i) => i.checked_sub(1),
                            };
                            let rate = if let Some(i) = i {
                                acctn.comm = in_commodity.clone();
                                amount *= cache[i].eq_amount;
                                Some(cache[i].eq_amount)
                            } else {
                                None
                            };
                            (acctn, amount, rate)
                        } else {
                            // Cache miss
                            (p.acctn.clone(), p.amount, None)
                        }
                    }
                }
            } else {
                (p.acctn.clone(), p.amount, None)
            }
        }))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PriceLookup {
    #[default]
    None,
    AtTheTimeOfTxn,
    LastPriceDbEntry,
    GivenTime(Zoned),
}

impl PriceLookup {
    pub(crate) fn make_ctx<'p>(
        &self,
        txns: &TxnRefs<'_>,
        in_commodity: Option<Arc<Commodity>>,
        price_db: &'p PriceDb,
    ) -> PriceLookupCtx<'p> {
        let Some(in_commodity) = in_commodity else {
            // No commodity conversion, short-circuit out
            return PriceLookupCtx::default();
        };
        let lookup_timestamp = match self {
            PriceLookup::AtTheTimeOfTxn => None,
            PriceLookup::LastPriceDbEntry => Some(Timestamp::MAX.to_zoned(TimeZone::UTC)),
            PriceLookup::GivenTime(t) => Some(t.clone()),

            PriceLookup::None => return PriceLookupCtx::default(),
        };
        //
        // Ok, we have real commodity conversion case
        //
        let used_commodities = txns
            .iter()
            .flat_map(|t| &t.posts)
            // This must be acctn.comm as txn_commodity is commodity for whole txn
            .map(|p| p.acctn.comm.clone())
            .collect::<BTreeSet<_>>();

        let cache = match lookup_timestamp {
            Some(lookup_timestamp) => Cache::Fixed(
                price_db
                    .iter()
                    .filter(|e| {
                        used_commodities.contains(&e.base_commodity)
                            && e.eq_commodity == in_commodity
                            && e.timestamp < lookup_timestamp
                    })
                    .map(|e| (e.base_commodity.clone(), (e.timestamp.clone(), e.eq_amount)))
                    .collect(),
            ),
            None => {
                let mut cache = HashMap::new();
                for comm in used_commodities {
                    let comm_cache: Vec<_> = price_db
                        .iter()
                        .filter(|e| comm == e.base_commodity && e.eq_commodity == in_commodity)
                        .sorted_by_key(|e| &e.timestamp) // make sure it's sorted
                        .collect();

                    if !comm_cache.is_empty() {
                        cache.insert(comm, comm_cache);
                    }
                }
                Cache::Timed(cache)
            }
        };

        PriceLookupCtx {
            cache,
            in_commodity: Some(in_commodity),
        }
    }
}
