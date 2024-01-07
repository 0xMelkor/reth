use std::result;

/// A RocksDB result.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("RocksDB error: {0}")]
    RocksDB(#[from] rocksdb::Error),
    #[error("Readonly database")]
    ReadOnly
}