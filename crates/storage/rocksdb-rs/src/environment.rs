use crate::{
    error::{Error, Result},
    flags::{DatabaseFlags, Mode},
    Transaction,
};
use core::fmt;
use reth_interfaces::db::LogLevel;
use rocksdb::{TransactionDB, TransactionDBOptions, Options};
use std::{path::Path, sync::Arc};

pub struct Environment {
    inner: Arc<rocksdb::TransactionDB>,
    access_mode: Mode,
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Environment({:?})", self.access_mode)
    }
}

impl Environment {
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder { log_level: None, access_mode: Mode::ReadOnly }
    }

    /// TODO: DOCS
    pub fn create_db<S: AsRef<str>>(&self, name: S) -> Result<()> {
        let mut opts = Options::default();
        opts.create_missing_column_families(true);
        // TODO: Tune at convenience
        // opts.create_if_missing(true);
        // opts.set_max_open_files(10000);
        // opts.set_use_fsync(false);
        // opts.set_bytes_per_sync(8388608);
        // opts.optimize_for_point_lookup(1024);
        // opts.set_table_cache_num_shard_bits(6);
        // opts.set_max_write_buffer_number(32);
        // opts.set_write_buffer_size(536870912);
        // opts.set_target_file_size_base(1073741824);
        // opts.set_min_write_buffer_number_to_merge(4);
        // opts.set_level_zero_stop_writes_trigger(2000);
        // opts.set_level_zero_slowdown_writes_trigger(0);
        // opts.set_compaction_style(DBCompactionStyle::Universal);
        // opts.set_disable_auto_compactions(true);
        self.inner.create_cf(name, &opts)?;
        Ok(())
    }

    /// Create a read-only transaction for use with the environment.
    #[inline]
    pub fn begin_ro_txn(&self) -> Result<Transaction> {
        let db = self.inner.clone();
        let tx = Transaction::ro(db);
        Ok(tx)
    }

    /// Create a read-write transaction for use with the environment.
    pub fn begin_rw_txn(&self) -> Result<Transaction> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        } else {
            let db = self.inner.clone();
            let tx = Transaction::rw(db);
            Ok(tx)
        }
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
        self.open_with_flags(path, DatabaseFlags::Open)
    }

    pub fn open_create(self, path: &Path) -> Result<Environment> {
        self.open_with_flags(path, DatabaseFlags::Create)
    }

    fn open_with_flags(self, path: &Path, flags: DatabaseFlags) -> Result<Environment> {
        let mut opts = rocksdb::Options::default();

        if let DatabaseFlags::Create = flags {
            opts.create_if_missing(true);
            opts.create_missing_column_families(true)
        }

        if let Some(level) = self.log_level {
            opts.set_log_level(level);
        }

        let txn_db_opts = TransactionDBOptions::default();
        let db = TransactionDB::open(&opts, &txn_db_opts, path)?;

        Ok(Environment { inner: Arc::new(db), access_mode: self.access_mode })
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
