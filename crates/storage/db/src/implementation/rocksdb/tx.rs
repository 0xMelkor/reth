use reth_interfaces::db::DatabaseError;
use reth_rocksdb::Transaction;

use crate::{
    table::{DupSort, Table, TableImporter, Encode, Decompress},
    transaction::{DbTx, DbTxMut},
};

use super::cursor::Cursor;

#[derive(Debug)]
pub struct Tx {
    pub inner: Transaction,
}

impl TableImporter for Tx {}

impl Tx {
    pub fn new(inner: Transaction) -> Self {
        Self { inner }
    }
}

impl DbTx for Tx {
    type Cursor<T: Table> = Cursor<T>;
    type DupCursor<T: DupSort> = Cursor<T>;

    fn get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        let key = key.encode().as_ref().to_vec();
        let res = self.inner.get(key)?;
        let value = res.map(|bytes| Decompress::decompress_owned(bytes)).transpose();
        value
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        Ok(true)
    }

    fn abort(self) {}

    fn cursor_read<T: Table>(&self) -> Result<Self::Cursor<T>, DatabaseError> {
        Ok(Cursor::new(0))
    }

    fn cursor_dup_read<T: DupSort>(&self) -> Result<Self::DupCursor<T>, DatabaseError> {
        Ok(Cursor::new(0))
    }

    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        todo!()
    }
}

impl DbTxMut for Tx {
    type CursorMut<T: Table> = Cursor<T>;
    type DupCursorMut<T: DupSort> = Cursor<T>;

    fn put<T: Table>(&self, _key: T::Key, _value: T::Value) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete<T: Table>(
        &self,
        _key: T::Key,
        _value: Option<T::Value>,
    ) -> Result<bool, DatabaseError> {
        Ok(true)
    }

    fn clear<T: Table>(&self) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn cursor_write<T: Table>(&self) -> Result<Self::CursorMut<T>, DatabaseError> {
        Ok(Cursor::new(0))
    }

    fn cursor_dup_write<T: DupSort>(&self) -> Result<Self::DupCursorMut<T>, DatabaseError> {
        Ok(Cursor::new(0))
    }
}
