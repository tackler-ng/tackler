/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::spec_items::AccountSpec;
use crate::config::{Account, Timestamp};
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
    pub(crate) time_defaults: Timestamp,
}
impl State {
    pub(crate) fn try_from(
        acc_spec: &config::Accounts,
        comm_spec: &config::Commodities,
        tags_spec: &config::Tags,
        strict_mode: bool,
        time_def: &config::Timestamp,
    ) -> Result<State, tackler::Error> {
        Ok(State {
            accounts: AccountTree::from(&acc_spec.items)?,
            commodities: Commodities::from(comm_spec)?,
            tags: Tags::from(tags_spec)?,
            strict_mode,
            time_defaults: time_def.clone(),
        })
    }
}

impl State {
    pub(crate) fn get_txn_account(
        &self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let comm = self.get_commodity(commodity.name.as_str())?;

        match self.accounts.defined_accounts.get(name) {
            Some(account_tree) => Ok(TxnAccount {
                atn: account_tree.clone(),
                comm,
            }),
            None => {
                if let Some(acc_parent) = self.accounts.synthetic_parents.get(name) {
                    Ok(TxnAccount {
                        atn: acc_parent.clone(),
                        comm,
                    })
                } else {
                    let msg = format!("Unknown account: '{name}'");
                    Err(msg.into())
                }
            }
        }
    }

    /// Get or create `TxnAccount` by name and commodity
    ///
    /// Both name and commodity must be valid name and ID
    /// e.g. this function is supposed to be used by parser.
    pub(crate) fn get_or_create_txn_account(
        &mut self,
        name: &str,
        commodity: &Arc<Commodity>,
    ) -> Result<TxnAccount, tackler::Error> {
        let comm = self.get_or_create_commodity(Some(commodity.name.as_str()))?;

        let txn_acc = if let Some(atn) = self.accounts.defined_accounts.get(name) {
            TxnAccount {
                atn: atn.clone(),
                comm,
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

            TxnAccount { atn, comm }
        };
        Ok(txn_acc)
    }

    /// # Errors
    /// Returns reference for commodity, error if it doesn't exist
    pub(crate) fn get_commodity(&self, name: &str) -> Result<Arc<Commodity>, tackler::Error> {
        if let Some(comm) = self.commodities.names.get(name) {
            Ok(comm.clone())
        } else {
            let msg = format!("Unknown commodity: '{name}'");
            Err(msg.into())
        }
    }

    /// Get or create `Commodity` by name
    ///
    /// The name must be a valid ID
    /// e.g. this is function is supposed to be used by parser.
    pub(crate) fn get_or_create_commodity(
        &mut self,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        Self::inner_get_or_create_commodity(&mut self.commodities, self.strict_mode, name)
    }

    fn inner_get_or_create_commodity(
        commodities: &mut Commodities,
        strict_mode: bool,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, tackler::Error> {
        if let Some(n) = name {
            if n.is_empty() {
                let res = if commodities.mandatory_commodity {
                    let msg = "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                    Err(msg.into())
                } else if let Some(c) = commodities.names.get(n) {
                    Ok(c.clone())
                } else {
                    let comm = Arc::new(Commodity::default());
                    commodities.names.insert(n.into(), comm.clone());
                    Ok(comm)
                };
                return res;
            }
            match commodities.names.get(n) {
                Some(comm) => Ok(comm.clone()),
                None => {
                    if strict_mode {
                        let msg = format!("Unknown commodity: '{n}'");
                        Err(msg.into())
                    } else {
                        let comm = Arc::new(Commodity::unchecked_from(n));
                        commodities.names.insert(n.into(), comm.clone());
                        Ok(comm)
                    }
                }
            }
        } else {
            let comm = Arc::new(Commodity::default());
            Ok(comm)
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
        let comm = Arc::new(Commodity::default());

        let txntn_1 = state.get_or_create_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account.spec.name, "a:b:c");
        assert_eq!(txntn_1.atn.get_leaf(), "c");

        let txntn_2 = state.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account.spec.name, "a:b:c");
        assert_eq!(txntn_2.atn.get_leaf(), "c");

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

        let comm = Arc::new(Commodity::default());
        let accounts: Vec<Arc<Account>> = vec!["a:b:c"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts).unwrap(/*:test:*/);

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

        let txntn_2 = state.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account.spec.name, "a:b:c");
        assert_eq!(txntn_2.atn.get_leaf(), "c");

        // Check that it won't create a synthetic account as real one
        assert!(state.get_or_create_txn_account("a:b", &comm).is_err());
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        // Check synthetic account
        let txntn_3 = state.get_txn_account("a:b", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_3.atn.depth, 2);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a");
        assert_eq!(txntn_3.atn.account.spec.name, "a:b");
        assert_eq!(txntn_3.atn.get_leaf(), "b");

        // Check synthetic account
        let txntn_4 = state.get_txn_account("a", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 1);
        assert_eq!(state.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_4.atn.depth, 1);
        assert_eq!(txntn_4.atn.get_root(), "a");
        assert_eq!(txntn_4.atn.parent, "");
        assert_eq!(txntn_4.atn.account.spec.name, "a");
        assert_eq!(txntn_4.atn.get_leaf(), "a");
    }

    #[test]
    fn accounts_strict_true_child_first() {
        let mut state = State {
            strict_mode: true,
            ..Default::default()
        };

        let comm = Arc::new(Commodity::default());
        let accounts: Vec<Arc<Account>> = vec!["a:b:c", "a:b", "a"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts).unwrap(/*:test:*/);

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

        let comm = Arc::new(Commodity::default());
        let accounts: Vec<Arc<Account>> = vec!["a:b:c:d", "a:b", "a"]
            .into_iter()
            .map(|s| {
                Arc::new(Account::try_with(AccountSpec::from_str(s), &Timestamp::default()).unwrap(/*:test:*/))
            })
            .collect();

        state.accounts = AccountTree::from(&accounts).unwrap(/*:test:*/);

        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);

        // Check that it won't create a synthetic account as real one
        assert!(state.get_or_create_txn_account("a:b:c", &comm).is_err());

        let txntn_synth = state.get_txn_account("a:b:c", &comm).unwrap(/*:test:*/);
        assert_eq!(state.accounts.defined_accounts.len(), 3);
        assert_eq!(state.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_synth.atn.account.spec.name, "a:b:c");

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
