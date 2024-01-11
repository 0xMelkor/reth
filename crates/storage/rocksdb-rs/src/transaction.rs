use core::fmt;
use std::sync::{Mutex, Arc};

use rocksdb::{SnapshotWithThreadMode, TransactionDB};

#[derive(Debug)]
pub struct Transaction {
    inner: TransactionInner,
}

impl Transaction {
    pub fn rw(tx: rocksdb::Transaction<TransactionDB>) -> Self {
        let tx = Mutex::new(tx);
        Self { inner: TransactionInner::RW(tx) }
    }

    pub fn ro(snapshot: SnapshotWithThreadMode<TransactionDB>) -> Self {
        let snapshot = Mutex::new(snapshot);
        Self { inner: TransactionInner::RO(snapshot) }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        match &self.inner {
            TransactionInner::RW(tx) => {
                // TODO: Get rid of the unwrap
                let tx = tx.lock().unwrap();
                tx.get(key)
            }
            TransactionInner::RO(snap) => {
                // TODO: Get rid of the unwrap
                let snap = snap.lock().unwrap();
                snap.get(key)
            }
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
        match &self.inner {
            TransactionInner::RW(tx) => {
                // TODO: Get rid of the unwrap
                let tx = tx.lock().unwrap();
                tx.put(key, value)
            }
            TransactionInner::RO(snap) => {
                unreachable!("RW transactions only");
            }
        }
    }

    pub fn commit(self) -> Result<bool, rocksdb::Error> {
        match self.inner {
            TransactionInner::RW(tx) => {
                // TODO: Get rid of the unwrap
                let tx = tx.into_inner().unwrap();
                // TODO: Clean up this interface lol
                Arc::new(tx).commit()?;
                Ok(true)
            }
            TransactionInner::RO(_snap) => {
                Ok(true)
            }
        }
    }
}

enum TransactionInner {
    RW(Mutex<rocksdb::Transaction<TransactionDB>>),
    RO(Mutex<rocksdb::SnapshotWithThreadMode<TransactionDB>>),
}

impl fmt::Debug for TransactionInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RW(_) => f.debug_tuple("RW").finish(),
            Self::RO(_) => f.debug_tuple("RO").finish(),
        }
    }
}
