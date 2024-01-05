use super::{mdbx, rocksdb};
use crate::database::Database;
use reth_interfaces::db::DatabaseError;

mod cursor;
mod tx;

/// TODO DOCS
#[derive(Debug)]
pub enum DatabaseEnvironment {
    /// TODO DOCS
    MBDX(mdbx::DatabaseEnv),
    /// TODO DOCS
    RocksDB(rocksdb::DatabaseEnv),
}

impl Database for DatabaseEnvironment {
    type TX = tx::Tx;
    type TXMut = tx::TxMut;

    fn tx(&self) -> Result<Self::TX, DatabaseError> {
        match self {
            DatabaseEnvironment::MBDX(db) => db.tx().map(tx::Tx::MBDXTx),
            DatabaseEnvironment::RocksDB(db) => db.tx().map(tx::Tx::RocksDBTx),
        }
    }

    fn tx_mut(&self) -> Result<Self::TXMut, DatabaseError> {
        match self {
            DatabaseEnvironment::MBDX(db) => db.tx_mut().map(tx::TxMut::MBDXTxMut),
            DatabaseEnvironment::RocksDB(db) => db.tx_mut().map(tx::TxMut::RocksDBTxMut),
        }
    }
}
