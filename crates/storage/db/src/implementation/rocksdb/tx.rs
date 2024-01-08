use reth_interfaces::db::DatabaseError;
use reth_rocksdb::Transaction;

use crate::{
    table::{DupSort, Table, TableImporter},
    transaction::{DbTx, DbTxMut},
};

use super::cursor::Cursor;

#[derive(Debug)]
pub struct Tx<'a> {
    pub inner: Transaction<'a>,
}

impl<'a> TableImporter for Tx<'a> {}

impl<'a> Tx<'a> {
    pub fn new(inner: Transaction<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> DbTx for Tx<'a> {
    type Cursor<T: Table> = Cursor<T>;
    type DupCursor<T: DupSort> = Cursor<T>;

    fn get<T: Table>(&self, _key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        Ok(None)
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

impl<'a> DbTxMut for Tx<'a> {
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
