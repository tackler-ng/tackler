/*
 * Tackler-NG 2023-2025
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
use std::collections::BTreeMap;
use std::{collections::HashSet, sync::Arc};

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
    /// Recursive get balance tree nodes for this subtree
    /// starting from and defined by "me"
    ///
    /// Input size is "small";  ~ size of Chart of Accounts
    /// Output size is "small"; ~ size of Chart of Accounts
    ///
    /// `me` is name of root account for this sub-tree
    /// `acc_sums` is list of all account sums
    /// `returns` list of balance tree nodes for this sub-tree
    fn get_balance_tree_nodes(
        me: &(TxnAccount, Decimal),
        acc_sums: &[(TxnAccount, Decimal)],
    ) -> Vec<BalanceTreeNode> {
        let (my_acctn, my_sum) = me;

        // find my children (childs)
        let childs = acc_sums
            .iter()
            .filter(|(atn, _)| my_acctn.is_parent_of(atn));

        // calculate balance tree nodes of my childs
        let childs_balance_trees = childs
            .flat_map(|c| Balance::get_balance_tree_nodes(c, acc_sums))
            .collect::<Vec<BalanceTreeNode>>();

        // calculate top sum of my children's balance trees
        let my_childs_sum = childs_balance_trees
            .iter()
            .filter(|btn| btn.acctn.atn.parent == my_acctn.atn.account)
            .map(|btn| btn.sub_acc_tree_sum)
            .sum::<Decimal>();

        let my_btn = BalanceTreeNode {
            acctn: my_acctn.clone(),
            sub_acc_tree_sum: my_childs_sum + my_sum,
            account_sum: *my_sum,
        };

        let mut x = vec![my_btn];
        x.extend(childs_balance_trees);
        x
    }

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
    /// `returns`  new set of Account Tree Sums without gaps from this branch to root (from leaf to root)
    fn bubble_up_acctn(
        my_acctn_sum: &(TxnAccount, Decimal),
        acc_sums: &[(TxnAccount, Decimal)],
        settings: &Settings,
    ) -> Result<Vec<(TxnAccount, Decimal)>, tackler::Error> {
        let my_acctn = &my_acctn_sum.0;
        if my_acctn.is_root() {
            // we are on top, so either this node (my_acctn) exist already
            // or it has been created by its child;
            // End of recursion
            Ok(vec![my_acctn_sum.clone()])
        } else {
            // Not on top => find parent for this node
            let parent = acc_sums
                .iter()
                .filter(|(atn, _)| atn.is_parent_of(my_acctn))
                .collect::<Vec<&(TxnAccount, Decimal)>>();

            assert!(parent.is_empty() || parent.len() == 1);

            if parent.is_empty() {
                if my_acctn.my_parent_is_root() {
                    // This is on depth 2, and it doesn't have parent
                    // => let's create root account
                    // End of Recursion
                    let new_parent_atn =
                        settings.get_txn_account(my_acctn.atn.parent.as_str(), &my_acctn.comm)?;
                    Ok(vec![(new_parent_atn, Decimal::ZERO), my_acctn_sum.clone()])
                } else {
                    let new_parent_atn =
                        settings.get_txn_account(my_acctn.atn.parent.as_str(), &my_acctn.comm)?;
                    let mut sub_tree = vec![my_acctn_sum.clone()];
                    let mut x = Balance::bubble_up_acctn(
                        &(new_parent_atn, Decimal::ZERO),
                        acc_sums,
                        settings,
                    )?;
                    x.append(&mut sub_tree);
                    Ok(x)
                }
            } else {
                // Parent of this exists, just bubble them up together
                let mut sub_tree = vec![my_acctn_sum.clone()];
                let mut x = Balance::bubble_up_acctn(parent[0], acc_sums, settings)?;
                x.append(&mut sub_tree);
                Ok(x)
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
    ) -> Vec<(TxnAccount, Decimal)>
    where
        I: Iterator<Item = &'a &'a Transaction>,
    {
        let inv = Decimal::from(-1);

        // Calculate sum of postings for each account.
        //
        // Input size: is "big",    ~ all transactions
        // Output size: is "small", ~ size of CoA
        txns.flat_map(|txn| price_lookup_ctx.convert_prices(txn))
            .sorted_by_key(|(acctn, _, _)| acctn.clone())
            .chunk_by(|(acctn, _, _)| acctn.clone())
            .into_iter()
            .map(|(_, postings)| {
                let mut ps = postings.peekable();
                // unwrap: ok: this is inside map, hence there must be at least one element
                let acctn = ps.peek().unwrap(/*:ok:*/).0.clone();
                let acc_sum = ps.map(|(_, amount, _)| amount).sum::<Decimal>();
                if inverted {
                    (acctn, acc_sum * inv)
                } else {
                    (acctn, acc_sum)
                }
            })
            .collect()
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
            Self::calculate_account_sums(txns, price_lookup_ctx, settings.inverted);

        // From every account bubble up and insert missing parent AccTNs.
        //
        // This will generate duplicate forks and roots, because we are arriving
        // from different branches to the same fork in the trunk. So the set must be made
        // distinct before it can be used, so we won't duplicate sub_tree_account_sums
        //
        // Why duplicates? This is using old incomplete set of AccTNSums, not the new,
        // complete set, which will be the result of this function,
        // so the same fork in trunk will be "missing" multiple times.
        //
        //
        // Input size:  "small", e.g. ~ size of CoA
        // Output size: "small", e.g. ~ size of CoA
        let complete_coa_sum_tree: &Vec<(TxnAccount, Decimal)> = &account_sums
            .iter()
            .try_fold(
                Vec::new(),
                |mut trees: Vec<Vec<(TxnAccount, Decimal)>>, acc| {
                    let bua = Balance::bubble_up_acctn(acc, &account_sums, settings)?;
                    trees.push(bua);
                    Ok::<Vec<Vec<(TxnAccount, Decimal)>>, tackler::Error>(trees)
                },
            )?
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>() // make it distinct
            .into_iter()
            .collect::<Vec<(TxnAccount, Decimal)>>();

        // Get all root accounts
        // Input size:  "small", e.g. ~ size of CoA
        // Output size: "small", e.g. ~ size of CoA
        let roots = complete_coa_sum_tree
            .iter()
            .filter(|(acctn, _)| acctn.atn.depth == 1);

        // Start from all roots and get all subtree BalanceTreeNodes
        // Input size:  "small", e.g. ~ size of CoA
        // Output size: "small", e.g. ~ size of CoA
        let mut bal = roots
            .flat_map(|root_acc_sum| {
                Balance::get_balance_tree_nodes(root_acc_sum, complete_coa_sum_tree)
            })
            .collect::<Vec<BalanceTreeNode>>();

        bal.sort_by(ord_by_btn);
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
        let account_sums: Vec<(TxnAccount, Decimal)> =
            Self::calculate_account_sums(txns, price_lookup_ctx, settings.inverted);

        let mut v: Vec<BalanceTreeNode> = account_sums
            .into_iter()
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
