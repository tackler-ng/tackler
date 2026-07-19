/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::hash::Hash;
use crate::kernel::{Predicate, Settings};
use crate::model::{TxnRefs, Txns, transaction};
use crate::tackler;
use itertools::Itertools;
use tackler_api::filters::FilterDefinition;
use tackler_api::metadata::items::{MetadataItem, TxnFilterDescription, TxnSetChecksum};
use tackler_api::metadata::{Checksum, Metadata};
use uuid::Uuid;

#[derive(Debug)]
pub struct TxnData {
    metadata: Option<Metadata>,
    txns: Txns,
    hash: Option<Hash>,
    unique_extid: bool,
}

pub struct TxnSet<'a> {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) txns: TxnRefs<'a>,
}

impl TxnSet<'_> {
    #[must_use]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.txns.is_empty()
    }
}

impl TxnData {
    #[must_use]
    pub fn len(&self) -> usize {
        self.txns.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.txns.is_empty()
    }

    /// Try to create `TxnData` from set of transactions
    ///
    /// # Errors
    /// If resulting txn set is logically invalid, the method will return error
    pub fn try_from(
        mdi_opt: Option<MetadataItem>,
        txns: Txns,
        settings: &Settings,
    ) -> Result<TxnData, tackler::Error> {
        let metadata = mdi_opt.map(Metadata::from_mdi);

        if settings.audit_mode {
            check_uuids(&txns)?;
        }
        if settings.is_extid_unique() {
            check_extid(&txns)?;
        }

        let mut t = txns;
        t.sort_by(transaction::ord_by_txn);

        Ok(TxnData {
            metadata,
            txns: t,
            hash: settings.get_hash().clone(),
            unique_extid: settings.is_extid_unique(),
        })
    }

    /// Append `TxnData` to existing `TxnData`
    ///
    /// This will reset the Metadata of target `TxnData`
    ///
    /// # Errors
    /// Returns `Err` in case resulting txn set is invalid (e.g. there are missing UUIDs)
    /// This could happen especially when Txns with Audit information are appended with
    /// plain Txns without UUIDs.
    pub fn append(&mut self, txn_data: &mut TxnData) -> Result<&mut Self, tackler::Error> {
        self.txns.append(&mut txn_data.txns);

        if self.hash.is_some() {
            check_uuids(&self.txns)?;
        }
        if self.unique_extid {
            check_extid(&self.txns)?;
        }

        let metadata =
            TxnData::make_metadata(self.hash.as_ref(), None, &self.txns.iter().collect())?;
        self.metadata = Some(metadata);
        Ok(self)
    }

    fn make_metadata(
        hash_opt: Option<&Hash>,
        metadata_opt: Option<&Metadata>,
        txns: &TxnRefs<'_>,
    ) -> Result<Metadata, tackler::Error> {
        let mut metadata = match metadata_opt {
            Some(md) => Metadata::from_metadata(md),
            None => Metadata::new(),
        };

        if let Some(hash) = hash_opt {
            let new_tsc_mdi = MetadataItem::TxnSetChecksum(TxnSetChecksum {
                size: txns.len(),
                hash: calc_txn_checksum(txns, hash)?,
            });

            metadata.push(new_tsc_mdi);
        }

        Ok(metadata)
    }

    /// # Errors
    /// Returns `Err` in case resulting Txn Set is not valid (e.g. there are missing UUIDs)
    pub fn filter(&self, tf: &FilterDefinition) -> Result<TxnSet<'_>, tackler::Error> {
        let refvec: TxnRefs<'_> = self.txns.iter().filter(|txn| tf.eval(txn)).collect();

        let mut metadata =
            TxnData::make_metadata(self.hash.as_ref(), self.metadata.as_ref(), &refvec)?;
        let filter_mdi = MetadataItem::TxnFilterDescription(TxnFilterDescription::from(tf.clone()));
        metadata.push(filter_mdi);

        Ok(TxnSet {
            metadata: Some(metadata),
            txns: refvec,
        })
    }

    /// # Errors
    /// Returns `Err` in case resulting Txn Set is not valid (e.g. there are missing UUIDs)
    pub fn get_all(&self) -> Result<TxnSet<'_>, tackler::Error> {
        let txns: TxnRefs<'_> = self.txns.iter().collect();

        let metadata = if self.hash.is_some() || self.metadata.is_some() {
            Some(TxnData::make_metadata(
                self.hash.as_ref(),
                self.metadata.as_ref(),
                &txns,
            )?)
        } else {
            None
        };

        Ok(TxnSet { metadata, txns })
    }
}

fn check_extid(txns: &Txns) -> Result<(), tackler::Error> {
    let dups: Vec<&String> = txns
        .iter()
        .filter_map(|txn| txn.header.extid.as_ref())
        .duplicates()
        .collect();

    if dups.is_empty() {
        Ok(())
    } else {
        let dups_count = dups.len();
        let msg = if dups_count < 10 {
            format!(
                "Found {} duplicate external ids.\nDuplicate ext-ids are:\n{}",
                dups.len(),
                dups.iter().join(",\n")
            )
        } else {
            format!(
                "Found {} duplicate external is.\nFirst ten duplicate ext-ids are:\n{}",
                dups.len(),
                dups[0..10].iter().join(",\n")
            )
        };
        Err(msg.into())
    }
}

fn check_uuids(txns: &Txns) -> Result<(), tackler::Error> {
    if txns.iter().any(|txn| txn.header.uuid.is_none()) {
        let msg =
            "Txn without UUID. Txn UUID is mandatory with transaction set checksum calculation.";
        return Err(msg.into());
    }

    let dups: Vec<&Uuid> = txns
        .iter()
        .filter_map(|txn| txn.header.uuid.as_ref())
        .duplicates()
        .collect();

    if dups.is_empty() {
        Ok(())
    } else {
        let dups_count = dups.len();
        let msg = if dups_count < 10 {
            format!(
                "Found {} duplicate txn uuids with txn set checksum.\nDuplicate ids are:\n{}",
                dups.len(),
                dups.iter().map(|u| { u.to_string() }).join(",\n")
            )
        } else {
            format!(
                "Found {} duplicate txn uuids with txn set checksum.\nFirst ten duplicate ids are:\n{}",
                dups.len(),
                dups[0..10].iter().map(|u| { u.to_string() }).join(",\n")
            )
        };
        Err(msg.into())
    }
}

fn calc_txn_checksum(txns: &TxnRefs<'_>, hasher: &Hash) -> Result<Checksum, tackler::Error> {
    let u: Result<Vec<String>, tackler::Error> = txns
        .iter()
        .map(|txn| {
            if let Some(u) = txn.header.uuid {
                Ok(u.to_string())
            } else {
                let msg = "Internal error: calc_txn_checksum with txns missing UUID";
                Err(msg.into())
            }
        })
        .collect();
    let mut uuids = u?;

    uuids.sort();

    let cs = hasher.checksum(&uuids, "\n".as_bytes());
    Ok(cs)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    // desc: check that uuid::to_string returns normalized lower-case UUID
    fn uuid_as_lower_case() {
        let uuid_ref = "e274c99e-1ebb-45e8-832d-58caf54ed95f";
        let uuid_mixed = "E274C99E-1ebb-45e8-832d-58Caf54Ed95f";
        let uuid_upper = "E274C99E-1EBB-45E8-832D-58CAF54ED95F";

        assert_eq!(
            Uuid::parse_str(uuid_ref).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
        assert_eq!(
            Uuid::parse_str(uuid_mixed).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
        assert_eq!(
            Uuid::parse_str(uuid_upper).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
    }
}
