use std::path::Path;

use reth_interfaces::db::{DatabaseError, LogLevel};

use crate::database::Database;

use super::mdbx::DatabaseEnvKind;

pub mod cursor;
pub mod tx;

/// Wrapper for the libmdbx environment: [Environment]
#[derive(Debug)]
pub struct DatabaseEnv {
    inner: (),
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
    /// Opens the database at the specified path with the given `EnvKind`.
    ///
    /// It does not create the tables, for that call [`DatabaseEnv::create_tables`].
    pub fn open(
        path: &Path,
        kind: DatabaseEnvKind, // TODO: Understand if this type can be shared in a root module
        log_level: Option<LogLevel>,
    ) -> Result<DatabaseEnv, DatabaseError> {
        todo!()
    }

    /// Enables metrics on the database.
    pub fn with_metrics(mut self) -> Self {
        self.with_metrics = true;
        self
    }

    /// Creates all the defined tables, if necessary.
    pub fn create_tables(&self) -> Result<(), DatabaseError> {
        todo!()
    }
}
