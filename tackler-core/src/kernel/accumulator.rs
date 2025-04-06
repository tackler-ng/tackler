/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::balance::Balance;
use crate::kernel::price_lookup::PriceLookupCtx;
use crate::kernel::report_item_selector::{BalanceSelector, RegisterSelector};
use crate::kernel::{RegisterSettings, Settings};
use crate::model::{RegisterEntry, RegisterPosting, Transaction, TxnAccount, TxnRefs};
use crate::tackler;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::io;

pub(crate) type RegisterReporterFn<W> =
    fn(writer: &mut W, &RegisterEntry<'_>, &RegisterSettings) -> Result<(), tackler::Error>;

pub(crate) type TxnGroupByOp<'a> = Box<dyn Fn(&Transaction) -> String + 'a>;

pub(crate) fn balance_groups<T>(
    txns: &TxnRefs<'_>,
    group_by_op: TxnGroupByOp<'_>,
    price_lookup_ctx: &PriceLookupCtx<'_>,
    ras: &T,
    settings: &Settings,
) -> Vec<Balance>
where
    T: BalanceSelector + ?Sized,
{
    txns.iter()
        .chunk_by(|txn| group_by_op(txn))
        .into_iter()
        // .par // todo: par-map
        .map(|(group_by_key, bal_grp_txns)| {
            Balance::from_iter(
                &group_by_key,
                bal_grp_txns,
                price_lookup_ctx,
                ras,
                settings,
                settings.report.balance_group.bal_type.clone(),
            )
            .expect("Logic error with Balance Group: inner balance failed")
        })
        .filter(|bal| !bal.is_empty())
        .sorted_by_key(|bal| bal.title.clone())
        .collect()
}

pub(crate) fn register_engine<'a, W, T>(
    txns: &'a TxnRefs<'_>,
    price_lookup_ctx: &PriceLookupCtx<'_>,
    ras: &T,
    w: &mut W,
    reporter: RegisterReporterFn<W>,
    register_settings: &RegisterSettings,
) -> Result<(), tackler::Error>
where
    W: io::Write + ?Sized,
    T: RegisterSelector<'a> + ?Sized,
{
    let mut register_engine: HashMap<TxnAccount, Decimal> = HashMap::new();

    // NOTE-1
    // This must be sorted, as we are collapsing all different commodities
    // to single target commodity for running total calculation. Without sorting,
    // register report running total could be in wrong order (biggest first)
    // within single transaction.
    //
    // See suite/price/ok/multi-vp-03.txn
    //
    // "aaa" is calculated after "ccc" into running total, but postings are printed in sorted order
    // (`filt_postings.sort()` in this function) - this will cause that aaa has bigger
    // running total value than ccc, if postings are not sorted before the running total calculation
    for txn in txns {
        let register_postings: Vec<_> = price_lookup_ctx
            .convert_prices(txn)
            .zip(&txn.posts)
            // note-1
            .sorted_by(|a, b| Ord::cmp(&a.1.acctn, &b.1.acctn))
            .map(|((conv_acctn, conv_amount, rate), orig_p)| {
                let running_total = *register_engine
                    .entry(conv_acctn.clone())
                    .and_modify(|v| {
                        *v += conv_amount;
                    })
                    .or_insert(conv_amount);

                RegisterPosting {
                    post: orig_p,
                    amount: running_total,
                    target_commodity: conv_acctn.comm,
                    rate,
                }
            })
            .collect();

        let mut filt_postings: Vec<_> = register_postings
            .into_iter()
            .filter(|p| ras.eval(p))
            .collect();

        filt_postings.sort();

        let register_entry = RegisterEntry {
            txn,
            posts: filt_postings,
        };
        reporter(w, &register_entry, register_settings)?;
    }
    Ok(())
}
