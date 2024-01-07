use super::{mdbx, rocksdb};
use crate::{database::Database, database_metrics::{DatabaseMetrics, DatabaseMetadata}};
use reth_interfaces::db::DatabaseError;

/// TODO: DOCS
pub mod cursor;
/// TODO: DOCS
pub mod tx;

/// Environment used when opening the database RO/RW.
#[derive(Debug)]
pub enum DbAccessMode {
    /// Read-only database access.
    RO,
    /// Read-write database access.
    RW,
}

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

impl DatabaseMetrics for DatabaseEnvironment {
    fn report_metrics(&self) {
        match self {
            DatabaseEnvironment::MBDX(db) => db.report_metrics(),
            DatabaseEnvironment::RocksDB(_) => todo!(),
        }
    }

    fn gauge_metrics(&self) -> Vec<(&'static str, f64, Vec<metrics::Label>)> {
        match self {
            DatabaseEnvironment::MBDX(db) => db.gauge_metrics(),
            DatabaseEnvironment::RocksDB(_) => todo!(),
        }
    }

    fn counter_metrics(&self) -> Vec<(&'static str, u64, Vec<metrics::Label>)> {
        match self {
            DatabaseEnvironment::MBDX(db) => db.counter_metrics(),
            DatabaseEnvironment::RocksDB(_) => todo!(),
        }
    }

    fn histogram_metrics(&self) -> Vec<(&'static str, f64, Vec<metrics::Label>)> {
        match self {
            DatabaseEnvironment::MBDX(db) => db.histogram_metrics(),
            DatabaseEnvironment::RocksDB(_) => todo!(),
        }
    }
}

impl DatabaseMetadata for DatabaseEnvironment {
    fn metadata(&self) -> crate::database_metrics::DatabaseMetadataValue {
        match self {
            DatabaseEnvironment::MBDX(db) => db.metadata(),
            DatabaseEnvironment::RocksDB(_) => todo!(),
        }
    }
}