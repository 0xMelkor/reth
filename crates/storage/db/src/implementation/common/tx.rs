use reth_libmdbx::{RO, RW};

use crate::{
    mdbx, rocksdb,
    table::{DupSort, Table, TableImporter},
    transaction::{DbTx, DbTxMut},
};

use super::cursor;

#[derive(Debug)]
pub enum Tx {
    MBDXTx(mdbx::tx::Tx<RO>),
    RocksDBTx(rocksdb::tx::Tx),
}

impl TableImporter for TxMut {}

#[derive(Debug)]
pub enum TxMut {
    MBDXTxMut(mdbx::tx::Tx<RW>),
    RocksDBTxMut(rocksdb::tx::Tx),
}

impl DbTx for Tx {
    type Cursor<T: Table> = cursor::Cursor<T>;
    type DupCursor<T: DupSort> = cursor::Cursor<T>;

    fn get<T: crate::table::Table>(
        &self,
        key: T::Key,
    ) -> Result<Option<T::Value>, reth_interfaces::db::DatabaseError> {
        match self {
            Tx::MBDXTx(tx) => tx.get::<T>(key),
            Tx::RocksDBTx(tx) => tx.get::<T>(key),
        }
    }

    fn commit(self) -> Result<bool, reth_interfaces::db::DatabaseError> {
        match self {
            Tx::MBDXTx(tx) => tx.commit(),
            Tx::RocksDBTx(tx) => tx.commit(),
        }
    }

    fn abort(self) {
        match self {
            Tx::MBDXTx(tx) => tx.abort(),
            Tx::RocksDBTx(tx) => tx.abort(),
        }
    }

    fn cursor_read<T: crate::table::Table>(
        &self,
    ) -> Result<Self::Cursor<T>, reth_interfaces::db::DatabaseError> {
        match self {
            Tx::MBDXTx(tx) => tx.cursor_read::<T>().map(cursor::Cursor::MdbxCursorRO),
            Tx::RocksDBTx(tx) => tx.cursor_read::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }

    fn cursor_dup_read<T: crate::table::DupSort>(
        &self,
    ) -> Result<Self::DupCursor<T>, reth_interfaces::db::DatabaseError> {
        match self {
            Tx::MBDXTx(tx) => tx.cursor_dup_read::<T>().map(cursor::Cursor::MdbxCursorRO),
            Tx::RocksDBTx(tx) => tx.cursor_dup_read::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }

    fn entries<T: crate::table::Table>(&self) -> Result<usize, reth_interfaces::db::DatabaseError> {
        match self {
            Tx::MBDXTx(tx) => tx.entries::<T>(),
            Tx::RocksDBTx(tx) => tx.entries::<T>(),
        }
    }
}

impl DbTx for TxMut {
    type Cursor<T: Table> = cursor::Cursor<T>;
    type DupCursor<T: DupSort> = cursor::Cursor<T>;

    fn get<T: crate::table::Table>(
        &self,
        key: T::Key,
    ) -> Result<Option<T::Value>, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.get::<T>(key),
            TxMut::RocksDBTxMut(tx) => tx.get::<T>(key),
        }
    }

    fn commit(self) -> Result<bool, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.commit(),
            TxMut::RocksDBTxMut(tx) => tx.commit(),
        }
    }

    fn abort(self) {
        match self {
            TxMut::MBDXTxMut(tx) => tx.abort(),
            TxMut::RocksDBTxMut(tx) => tx.abort(),
        }
    }

    fn cursor_read<T: crate::table::Table>(
        &self,
    ) -> Result<Self::Cursor<T>, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.cursor_read::<T>().map(cursor::Cursor::MdbxCursorRW),
            TxMut::RocksDBTxMut(tx) => tx.cursor_read::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }

    fn cursor_dup_read<T: crate::table::DupSort>(
        &self,
    ) -> Result<Self::DupCursor<T>, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.cursor_dup_read::<T>().map(cursor::Cursor::MdbxCursorRW),
            TxMut::RocksDBTxMut(tx) => tx.cursor_dup_read::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }

    fn entries<T: crate::table::Table>(&self) -> Result<usize, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.entries::<T>(),
            TxMut::RocksDBTxMut(tx) => tx.entries::<T>(),
        }
    }
}

impl DbTxMut for TxMut {
    type CursorMut<T: Table> = cursor::Cursor<T>;
    type DupCursorMut<T: DupSort> = cursor::Cursor<T>;

    fn put<T: Table>(
        &self,
        key: T::Key,
        value: T::Value,
    ) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.put::<T>(key, value),
            TxMut::RocksDBTxMut(tx) => tx.put::<T>(key, value),
        }
    }

    fn delete<T: Table>(
        &self,
        key: T::Key,
        value: Option<T::Value>,
    ) -> Result<bool, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.delete::<T>(key, value),
            TxMut::RocksDBTxMut(tx) => tx.delete::<T>(key, value),
        }
    }

    fn clear<T: Table>(&self) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.clear::<T>(),
            TxMut::RocksDBTxMut(tx) => tx.clear::<T>(),
        }
    }

    fn cursor_write<T: Table>(
        &self,
    ) -> Result<Self::CursorMut<T>, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.cursor_write::<T>().map(cursor::Cursor::MdbxCursorRW),
            TxMut::RocksDBTxMut(tx) => tx.cursor_write::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }

    fn cursor_dup_write<T: DupSort>(
        &self,
    ) -> Result<Self::DupCursorMut<T>, reth_interfaces::db::DatabaseError> {
        match self {
            TxMut::MBDXTxMut(tx) => tx.cursor_dup_write::<T>().map(cursor::Cursor::MdbxCursorRW),
            TxMut::RocksDBTxMut(tx) => tx.cursor_dup_write::<T>().map(cursor::Cursor::RocksDbCursor),
        }
    }
}
