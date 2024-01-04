use std::collections::BTreeMap;

use reth_interfaces::db::DatabaseError;

use crate::{transaction::{DbTx, DbTxMut}, table::{Table, DupSort, TableImporter}};

use super::cursor::Cursor;

#[derive(Debug)]
pub struct Tx {
    /// Table representation
    _table: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl TableImporter for Tx {}

impl DbTx for Tx {
    type Cursor<T: Table> = Cursor;
    type DupCursor<T: DupSort> = Cursor;

    fn get<T: Table>(&self, _key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        Ok(None)
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        Ok(true)
    }

    fn abort(self) {}

    fn cursor_read<T: Table>(&self) -> Result<Self::Cursor<T>, DatabaseError> {
        Ok(Cursor { _cursor: 0 })
    }

    fn cursor_dup_read<T: DupSort>(&self) -> Result<Self::DupCursor<T>, DatabaseError> {
        Ok(Cursor { _cursor: 0 })
    }

    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        Ok(self._table.len())
    }
}

impl DbTxMut for Tx {
    type CursorMut<T: Table> = Cursor;
    type DupCursorMut<T: DupSort> = Cursor;

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
        Ok(Cursor { _cursor: 0 })
    }

    fn cursor_dup_write<T: DupSort>(&self) -> Result<Self::DupCursorMut<T>, DatabaseError> {
        Ok(Cursor { _cursor: 0 })
    }
}