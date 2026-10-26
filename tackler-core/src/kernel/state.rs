/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Account;
use crate::config::spec_items::AccountSpec;
use crate::kernel::state::account_tree::{AccountTree, Commodities, Tags};
use crate::model::{AccountTreeNode, Commodity, TxnAccount};
use crate::{config, tackler};
use std::sync::Arc;
use tackler_api::txn_header::Tag;

pub(crate) mod account_tree;

#[derive(Debug, Default)]
pub(crate) struct State {
    pub(crate) accounts: AccountTree,
    pub(crate) commodities: Commodities,
    pub(crate) tags: Tags,
    pub(crate) strict_mode: bool,
    pub(crate) time_defaults: config::Timestamp,
}
impl State {
    pub(crate) fn try_from(
        acc_spec: &config::Accounts,
        comm_spec: &config::Commodities,
        tags_spec: &config::Tags,
        strict_mode: bool,
        time_def: &config::Timestamp,
    ) -> Result<State, tackler::Error> {
        let state = State {
            accounts: AccountTree::from(&acc_spec.items, strict_mode)?,
            commodities: Commodities::from(comm_spec)?,
            tags: Tags::from(tags_spec)?,
            strict_mode,
            time_defaults: time_def.clone(),
        };

        Ok(state)
    }
}

impl State {
    /// Get parent `TxnAccount` of child
    ///
    /// # Errors
    /// Error if child is root
    pub(crate) fn parent_txn_account(
        &self,
        child: &Arc<AccountTreeNode>,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let parent = self.accounts.parent_account(child)?;

        Ok(TxnAccount {
            atn: parent,
            comm: commodity.clone(),
        })
    }

    /// Get or create `TxnAccount` by name and `Commodity`
    ///
    /// Both name and commodity must be valid name and ID
    /// e.g. this function is supposed to be used by parser.
    pub(crate) fn get_or_create_txn_account(
        &mut self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let txn_acc = if let Some(atn) = self.accounts.defined_accounts.get(name) {
            TxnAccount {
                atn: atn.clone(),
                comm: commodity.clone(),
            }
        } else {
            if self.strict_mode {
                let msg = format!("Unknown account: '{name}'");
                return Err(msg.into());
            }
            let account = Arc::new(Account::try_with(
                AccountSpec::from_str(name),
                &self.time_defaults,
            )?);
            let atn = Arc::new(AccountTreeNode::unchecked_from(&account));
            self.accounts.insert(&atn)?;

            TxnAccount {
                atn,
                comm: commodity.clone(),
            }
        };
        Ok(txn_acc)
    }

    /// Get or create `Commodity` by name
    ///
    /// The name must be a valid ID
    /// e.g. this is function is supposed to be used by parser.
    pub(crate) fn get_or_create_commodity(
        &mut self,
        name: &str,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        if name.is_empty() {
            if self.commodities.mandatory_commodity {
                let msg = "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                Err(msg.into())
            } else if let Some(c) = self.commodities.names.get(name) {
                Ok(c.clone())
            } else {
                let comm = Arc::new(Commodity::default());
                self.commodities.names.insert(name.into(), comm.clone());
                Ok(comm)
            }
        } else {
            match self.commodities.names.get(name) {
                Some(comm) => Ok(comm.clone()),
                None => {
                    if self.strict_mode {
                        let msg = format!("Unknown commodity: '{name}'");
                        Err(msg.into())
                    } else {
                        let comm = Arc::new(Commodity::unchecked_from(name));
                        self.commodities.names.insert(name.into(), comm.clone());
                        Ok(comm)
                    }
                }
            }
        }
    }

    pub(crate) fn get_or_create_tag(&mut self, name: &str) -> Result<Arc<Tag>, tackler::Error> {
        if name.is_empty() {
            let msg = "Tag name is empty string".to_string();
            return Err(msg.into());
        }
        match self.tags.names.get(name) {
            Some(tag) => Ok(tag.clone()),
            None => {
                if self.strict_mode {
                    let msg = format!("Unknown tag: '{name}'");
                    Err(msg.into())
                } else {
                    let tag = Arc::new(Tag::from(name));
                    self.tags.names.insert(name.into(), tag.clone());
                    Ok(tag)
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::spec_items::AccountSpec;
    use crate::config::{Account, Timestamp};

    #[test]
    fn accounts_strict_false() {
        let mut state = State::default();
        let comm = state.get_or_create_commodity("").unwrap();

        let txntn_1 = state.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account.spec.name, "a:b:c");
        assert_eq!(txntn_1.atn.get_leaf(), "c");

        let child = Arc::new(AccountTreeNode::from("a:b:c").unwrap(/*:test*:*/));
        let txntn_2 = state.parent_txn_account(&child, &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 2);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a");
        assert_eq!(txntn_2.atn.account.spec.name, "a:b");
        assert_eq!(txntn_2.atn.get_leaf(), "b");

        let txntn_3 = state.get_or_create_txn_account("a:b:b-leaf", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 2);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_3.atn.depth, 3);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a:b");
        assert_eq!(txntn_3.atn.account.spec.name, "a:b:b-leaf");
        assert_eq!(txntn_3.atn.get_leaf(), "b-leaf");
    }

    #[test]
    fn accounts_strict_true() {
        let mut state = State {
            strict_mode: true,
            ..Default::default()
        };

        let comm = state.get_or_create_commodity("").unwrap();
        let accounts: Vec<Arc<Account>> = vec!["a:b:c"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts, true).unwrap(/*:test:*/);

        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        let txntn_1 = state.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account.spec.name, "a:b:c");
        assert_eq!(txntn_1.atn.get_leaf(), "c");

        let child_2 = Arc::new(AccountTreeNode::from("a:b:c").unwrap(/*:test*:*/));
        let txntn_2 = state.parent_txn_account(&child_2, &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 2);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a");
        assert_eq!(txntn_2.atn.account.spec.name, "a:b");
        assert_eq!(txntn_2.atn.get_leaf(), "b");

        // Check that it won't create a synthetic account as real one
        assert!(state.get_or_create_txn_account("a:b", &comm).is_err());
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        // Check synthetic account
        let child_3 = Arc::new(AccountTreeNode::from("a:b").unwrap(/*:test*:*/));
        let txntn_3 = state.parent_txn_account(&child_3, &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_3.atn.depth, 1);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "");
        assert_eq!(txntn_3.atn.account.spec.name, "a");
        assert_eq!(txntn_3.atn.get_leaf(), "a");

        // Check synthetic account
        let child_4 = Arc::new(AccountTreeNode::from("a").unwrap(/*:test*:*/));
        assert!(state.parent_txn_account(&child_4, &comm).is_err());
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);
    }

    #[test]
    fn accounts_strict_true_child_first() {
        let mut state = State {
            strict_mode: true,
            ..Default::default()
        };

        let comm = state.get_or_create_commodity("").unwrap();
        let accounts: Vec<Arc<Account>> = vec!["a:b:c", "a:b", "a"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts, true).unwrap(/*:test:*/);

        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 0);

        let txntn_1 = state.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_1.atn.account.spec.name, "a:b:c");

        let txntn_2 = state.get_or_create_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account.spec.name, "a:b");

        let txntn_2 = state.get_or_create_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account.spec.name, "a");
    }

    #[test]
    fn accounts_strict_true_gap() {
        let mut state = State {
            strict_mode: true,
            ..Default::default()
        };

        let comm = state.get_or_create_commodity("").unwrap();
        let accounts: Vec<Arc<Account>> = vec!["a:b:c:d", "a:b", "a"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts, true).unwrap(/*:test:*/);

        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);

        // Check that it won't create a synthetic account as real one
        assert!(state.get_or_create_txn_account("a:b:c", &comm).is_err());

        let child = Arc::new(AccountTreeNode::from("a:b:c").unwrap(/*:test*:*/));
        let txntn_synth = state.parent_txn_account(&child, &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_synth.atn.account.spec.name, "a:b");

        let txntn_2 = state.get_or_create_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account.spec.name, "a:b");

        let txntn_2 = state.get_or_create_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account.spec.name, "a");
    }
}
