/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config::BalanceType;
use crate::kernel::Settings;
use crate::kernel::price_lookup::PriceLookupCtx;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::model::balance_tree_node::ord_by_btn;
use crate::model::{BalanceTreeNode, Commodity, Transaction, TxnAccount, TxnSet};
use crate::tackler;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

// Deltas must be sorted by Commodity on reports, use BTreeMap
pub type Deltas = BTreeMap<Option<Arc<Commodity>>, Decimal>;
pub type BTNs = Vec<BalanceTreeNode>;
#[derive(Debug)]
pub struct Balance {
    pub(crate) title: String,
    pub(crate) bal: BTNs,
    pub(crate) deltas: Deltas,
}

impl Balance {
    pub(crate) fn is_empty(&self) -> bool {
        self.bal.is_empty()
    }
}

impl Balance {
    /// Bubble up from leafs to root, and generate any missing (gap)
    /// `AccountTreeNode` (ATN) for new ATN entry with zero atn sum.
    ///
    /// The max depth of recursion is the sub-account count
    /// from leaf to root (e.g. it's small)
    ///
    /// * Input size is "small";  ~ size of Chart of Accounts
    /// * Output size is "small"; ~ size of Chart of Accounts
    ///
    /// `my_acctn_sum` starting Account Tree Sum entry
    /// `acc_sums` current incomplete (in sense of Chart of Account) account sums
    fn bubble_up_acctn(
        acc_sums: &mut HashMap<TxnAccount, Decimal>,
        my_acctn_sum: &(TxnAccount, Decimal),
        settings: &Settings,
    ) -> Result<(), tackler::Error> {
        let my_acctn = &my_acctn_sum.0;
        if my_acctn.is_root() {
            // we are on top, so this node (my_acctn) exist already
            // End of recursion
            Ok(())
        } else {
            // Not on top => find parent for this node
            let new_parent_atn =
                settings.get_txn_account(my_acctn.atn.parent.as_str(), &my_acctn.comm)?;

            let parent = acc_sums.get_key_value(&new_parent_atn);
            if parent.is_some() {
                // End of recursion
                Ok(())
            } else if new_parent_atn.is_root() {
                acc_sums.insert(new_parent_atn, Decimal::ZERO);
                // End of recursion
                Ok(())
            } else {
                acc_sums.insert(new_parent_atn.clone(), Decimal::ZERO);
                Balance::bubble_up_acctn(acc_sums, &(new_parent_atn, Decimal::ZERO), settings)
            }
        }
    }

    /// Calculate sum of postings for each account.
    ///
    /// Input size: is "big",    ~ all transactions
    /// Output size: is "small", ~ size of Chart of Accounts
    fn calculate_account_sums<'a, I>(
        txns: I,
        price_lookup_ctx: &PriceLookupCtx<'_>,
        inverted: bool,
    ) -> impl Iterator<Item = (TxnAccount, Decimal)>
    where
        I: Iterator<Item = &'a &'a Transaction>,
    {
        let mut account_sums: HashMap<TxnAccount, Decimal> = HashMap::new();

        txns.for_each(|txn| {
            price_lookup_ctx.convert_prices(txn).for_each(|p| {
                let val = if inverted { -p.1 } else { p.1 };
                account_sums
                    .entry(p.0)
                    .and_modify(|v| {
                        *v += val;
                    })
                    .or_insert(val);
            });
        });
        account_sums.into_iter()
    }

    /// Calculate balance items
    ///
    /// * Input size is "big";     ~ all transactions
    /// * Output size is "small";  ~ size of Chart of Accounts
    ///
    /// * `txns` sequence of transactions
    /// * `returns` unfiltered sequence of `BalanceTreeNode`s
    fn balance_tree<'a, I>(
        txns: I,
        price_lookup_ctx: &PriceLookupCtx<'_>,
        settings: &Settings,
    ) -> Result<Vec<BalanceTreeNode>, tackler::Error>
    where
        I: Iterator<Item = &'a &'a Transaction>,
    {
        // Calculate sum of postings for each account.
        //
        // Input size: is "big",    ~ all transactions
        // Output size: is "small", ~ size of CoA
        let account_sums: Vec<(TxnAccount, Decimal)> =
            Self::calculate_account_sums(txns, price_lookup_ctx, settings.inverted).collect();

        // From every account bubble up and insert missing parent AccTNs.
        //
        // Input size:  "small", e.g. ~ size of CoA
        // Output size: "small", e.g. ~ size of CoA
        let mut complete_acctn_sums: HashMap<TxnAccount, Decimal> =
            HashMap::from_iter(account_sums.iter().cloned());

        account_sums
            .iter()
            .try_for_each(|acc| -> Result<(), tackler::Error> {
                Balance::bubble_up_acctn(&mut complete_acctn_sums, acc, settings)
            })?;

        let mut bal_tns: HashMap<TxnAccount, BalanceTreeNode> = HashMap::new();
        complete_acctn_sums
            .iter()
            .sorted_by(|a, b| a.0.atn.depth.cmp(&b.0.atn.depth).reverse())
            .try_for_each(|atn_val| -> Result<(), tackler::Error> {
                // me: insert or update
                let me = bal_tns
                    .entry(atn_val.0.clone())
                    .and_modify(|v| {
                        v.account_sum += *atn_val.1;
                        v.sub_acc_tree_sum += *atn_val.1;
                    })
                    .or_insert({
                        BalanceTreeNode {
                            acctn: atn_val.0.clone(),
                            sub_acc_tree_sum: *atn_val.1,
                            account_sum: *atn_val.1,
                        }
                    });

                let tree_sum = me.sub_acc_tree_sum;

                // If I'm not root, then insert or update parent
                if !atn_val.0.is_root() {
                    let parent_atn =
                        settings.get_txn_account(atn_val.0.atn.parent.as_str(), &atn_val.0.comm)?;

                    bal_tns
                        .entry(parent_atn.clone())
                        .and_modify(|v| {
                            v.sub_acc_tree_sum += tree_sum;
                        })
                        .or_insert({
                            BalanceTreeNode {
                                acctn: parent_atn,
                                sub_acc_tree_sum: tree_sum,
                                account_sum: Decimal::ZERO,
                            }
                        });
                }
                Ok(())
            })?;

        let bal: Vec<BalanceTreeNode> = bal_tns
            .into_iter()
            .map(|c| c.1)
            .sorted_by(ord_by_btn)
            .collect();

        Ok(bal)
    }

    /// Balance for this `txn_set`
    ///
    /// # Errors
    /// Returns `Err` in case of error
    pub fn from<T>(
        title: &str,
        txn_set: &TxnSet<'_>,
        price_lookup_ctx: &PriceLookupCtx<'_>,
        accounts: &T,
        settings: &Settings,
    ) -> Result<Balance, tackler::Error>
    where
        T: BalanceSelector + ?Sized,
    {
        Self::from_iter(
            title,
            &txn_set.txns,
            price_lookup_ctx,
            accounts,
            settings,
            settings.report.balance.bal_type.clone(),
        )
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn from_iter<'a, I, T>(
        title: &str,
        txns: I,
        price_lookup_ctx: &PriceLookupCtx<'_>,
        accounts: &T,
        settings: &Settings,
        bal_type: BalanceType,
    ) -> Result<Balance, tackler::Error>
    where
        T: BalanceSelector + ?Sized,
        I: IntoIterator<Item = &'a &'a Transaction>,
    {
        let bal = match bal_type {
            BalanceType::Tree => {
                Balance::balance_tree(txns.into_iter(), price_lookup_ctx, settings)?
            }
            BalanceType::Flat => {
                Balance::balance_flat(txns.into_iter(), price_lookup_ctx, settings)
            }
        };

        let filt_bal: Vec<_> = bal.into_iter().filter(|b| accounts.eval(b)).collect();

        if filt_bal.is_empty() {
            Ok(Balance {
                title: title.to_string(),
                bal: Vec::default(),
                deltas: BTreeMap::default(),
            })
        } else {
            let deltas = filt_bal
                .iter()
                .chunk_by(|btn| btn.acctn.comm.clone())
                .into_iter()
                .map(|(c, bs)| {
                    let dsum = bs.map(|b| b.account_sum).sum();
                    (c.is_any().then_some(c), dsum)
                })
                .collect();

            Ok(Balance {
                title: title.to_string(),
                bal: filt_bal,
                deltas,
            })
        }
    }

    fn balance_flat<'a, I>(
        txns: I,
        price_lookup_ctx: &PriceLookupCtx<'_>,
        settings: &Settings,
    ) -> Vec<BalanceTreeNode>
    where
        I: Iterator<Item = &'a &'a Transaction>,
    {
        let account_sums = Self::calculate_account_sums(txns, price_lookup_ctx, settings.inverted);

        let mut v: Vec<BalanceTreeNode> = account_sums
            .map(|(acctn, acc_sum)| BalanceTreeNode {
                acctn: acctn.clone(),
                sub_acc_tree_sum: Decimal::ZERO,
                account_sum: acc_sum,
            })
            .collect();

        v.sort_by(ord_by_btn);
        v
    }
}
