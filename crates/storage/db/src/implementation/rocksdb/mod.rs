//! Module that interacts with RocksDB.

use self::tx::Tx;

use super::common::DbAccessMode;
use crate::{database::Database, Tables};
use reth_interfaces::db::{DatabaseError, LogLevel};
use reth_rocksdb::{Environment, Mode};
use reth_tracing::tracing::error;
use std::path::Path;

/// TODO: DOCS
pub mod cursor;
/// TODO: DOCS
pub mod tx;

/// Wrapper for the libmdbx environment: [Environment]
#[derive(Debug)]
pub struct DatabaseEnv {
    inner: Environment,
    /// Whether to record metrics or not.
    with_metrics: bool,
}

impl Database for DatabaseEnv {
    type TX = tx::Tx<'static>;
    type TXMut = tx::Tx<'static>;

    fn tx(&self) -> Result<Self::TX, reth_interfaces::db::DatabaseError> {
        let inner = self.inner.begin_ro_txn().map_err(|e| DatabaseError::InitTx(-12))?;
        Ok(Tx::new(inner))
    }

    fn tx_mut(&self) -> Result<Self::TXMut, reth_interfaces::db::DatabaseError> {
        let inner = self.inner.begin_rw_txn().map_err(|e| DatabaseError::InitTx(-12))?;
        Ok(Tx::new(inner))
    }
}

impl DatabaseEnv {
    /// Opens the database at the specified path with the given [`DbAccessMode`].
    ///
    /// It does not create the tables, for that call [`DatabaseEnv::create_tables`].
    pub fn open(
        path: &Path,
        kind: DbAccessMode,
        log_level: Option<LogLevel>,
    ) -> Result<DatabaseEnv, DatabaseError> {
        let mut inner_env = Environment::builder();
        if let Some(level) = log_level {
            inner_env.set_log_level(level);
        }

        inner_env.set_mode(match kind {
            DbAccessMode::RO => Mode::ReadOnly,
            DbAccessMode::RW => Mode::ReadWrite,
        });

        let inner = inner_env.open(path).map_err(|e| {
            error!(?e, "Failed to open {kind:?} database");
            DatabaseError::Open(-1) //TODO: Can we provide a better error code?
        })?;

        let env = DatabaseEnv { inner, with_metrics: false };

        Ok(env)
    }

    /// Creates all the defined tables, if necessary.
    pub fn create_tables(&self) -> Result<(), DatabaseError> {
        // TODO: Extend Error with e.into()
        for table in Tables::ALL {
            let name = table.name();
            self.inner.create_db(table.name()).map_err(|e| {
                error!(?e, "Failed to create database table {name}");
                DatabaseError::CreateTable(-3)
            })?;
        }
        Ok(())
    }

    /// Enables metrics on the database.
    pub fn with_metrics(mut self) -> Self {
        // TODO: Understand how to integrate
        self.with_metrics = true;
        self
    }
}
