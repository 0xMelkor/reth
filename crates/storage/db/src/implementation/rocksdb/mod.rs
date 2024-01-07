//! Module that interacts with RocksDB.

use super::common::DbAccessMode;
use crate::database::Database;
use reth_interfaces::db::{DatabaseError, LogLevel};
use reth_rocksdb::{Environment, Mode};
use std::path::Path;
use reth_tracing::tracing::error;

pub mod cursor;
pub mod tx;

/// Wrapper for the libmdbx environment: [Environment]
#[derive(Debug)]
pub struct DatabaseEnv {
    inner: Environment,
    /// Whether to record metrics or not.
    with_metrics: bool,
}

impl Database for DatabaseEnv {
    type TX = tx::Tx;
    type TXMut = tx::Tx;

    fn tx(&self) -> Result<Self::TX, reth_interfaces::db::DatabaseError> {
        todo!()
    }

    fn tx_mut(&self) -> Result<Self::TXMut, reth_interfaces::db::DatabaseError> {
        todo!()
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

        let inner = inner_env.open(path).map_err(|e|{
            error!(?e, "Failed to open {kind:?} database");
            DatabaseError::Open(-1) //TODO: Can we provide a better error code?
        })?;

        let env = DatabaseEnv { inner, with_metrics: false };

        Ok(env)
    }

    /// Enables metrics on the database.
    pub fn with_metrics(mut self) -> Self {
        // TODO: Understand how to integrate
        self.with_metrics = true;
        self
    }

    /// Creates all the defined tables, if necessary.
    pub fn create_tables(&self) -> Result<(), DatabaseError> {
        todo!()
    }
}
