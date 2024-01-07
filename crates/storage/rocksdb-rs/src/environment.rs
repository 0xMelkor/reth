use crate::{
    error::{Error, Result},
    flags::Mode,
};
use reth_interfaces::db::LogLevel;
use rocksdb::{TransactionDB, DB};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub struct Environment {
    inner: Arc<rocksdb::DB>,
    path: Box<PathBuf>,
    access_mode: Mode,
}

impl Environment {
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder { log_level: None, access_mode: Mode::ReadOnly }
    }

    /// Create a read-only transaction for use with the environment.
    #[inline]
    pub fn begin_ro_txn(&self) -> Result<()> {
        todo!()
    }

    /// Create a read-write transaction for use with the environment.
    pub fn begin_rw_txn(&self) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }
        todo!()
    }

    /// Returns true if the environment was opened in [Mode::ReadWrite] mode.
    #[inline]
    pub fn is_read_write(&self) -> bool {
        matches!(self.access_mode, Mode::ReadWrite)
    }

    /// Returns true if the environment was opened in [Mode::ReadOnly] mode.
    #[inline]
    pub fn is_read_only(&self) -> bool {
        matches!(self.access_mode, Mode::ReadOnly)
    }
}

pub struct EnvironmentBuilder {
    log_level: Option<rocksdb::LogLevel>,
    access_mode: Mode,
}
impl EnvironmentBuilder {
    pub fn open(self, path: &Path) -> Result<Environment> {
        // Detect mode RO/RW
        // Set geometry options
        // - size
        // - shrink
        // - grow step
        // - page size

        // Set flags
        // - disable readahead: we must favour random access vs linear scan
        // - set max readers

        // Set log level

        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(false);
        if let Some(level) = self.log_level {
            opts.set_log_level(level);
        }

        let db = match self.access_mode {
            Mode::ReadOnly => DB::open_for_read_only(&opts, path, false)?,
            Mode::ReadWrite => DB::open(&opts, path)?,
        };

        Ok(Environment {
            inner: Arc::new(db),
            path: Box::new(path.to_owned()),
            access_mode: self.access_mode,
        })
    }

    pub fn set_log_level(&mut self, level: LogLevel) -> &mut Self {
        self.log_level = Some(match level {
            LogLevel::Fatal => rocksdb::LogLevel::Fatal,
            LogLevel::Error => rocksdb::LogLevel::Error,
            LogLevel::Warn => rocksdb::LogLevel::Warn,
            LogLevel::Notice => rocksdb::LogLevel::Info,
            LogLevel::Verbose => rocksdb::LogLevel::Debug,
            LogLevel::Debug => rocksdb::LogLevel::Debug,
            LogLevel::Trace => rocksdb::LogLevel::Debug,
            LogLevel::Extra => rocksdb::LogLevel::Debug,
        });
        self
    }

    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.access_mode = mode;
        self
    }
}
