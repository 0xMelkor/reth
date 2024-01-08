use core::fmt;
use std::sync::Mutex;

use rocksdb::{TransactionDB, SnapshotWithThreadMode};

#[derive(Debug)]
pub struct Transaction<'db> {
    inner: TransactionInner<'db>,
}

impl<'db> Transaction<'db> {
    pub fn rw(tx: rocksdb::Transaction<'db, TransactionDB>) -> Self {
        let tx = Mutex::new(tx);
        Self { inner: TransactionInner::RW(tx) }
    }

    pub fn ro(snapshot: SnapshotWithThreadMode<'db, TransactionDB>) -> Self {
        let snapshot = Mutex::new(snapshot);
        Self { inner: TransactionInner::RO(snapshot) }
    }
}

enum TransactionInner<'db> {
    RW(Mutex<rocksdb::Transaction<'db, TransactionDB>>),
    RO(Mutex<rocksdb::SnapshotWithThreadMode<'db, TransactionDB>>)
}

impl<'db> fmt::Debug for TransactionInner<'db> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RW(_) => f.debug_tuple("RW").finish(),
            Self::RO(_) => f.debug_tuple("RO").finish(),
        }
    }
}