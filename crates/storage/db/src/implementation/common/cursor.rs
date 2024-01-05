use crate::{
    cursor::{
        DbCursorRO, DbCursorRW, DbDupCursorRO, DbDupCursorRW, DupWalker, RangeWalker,
        ReverseWalker, Walker,
    },
    implementation::{mdbx, rocksdb},
    table::{DupSort, Table},
};

#[derive(Debug)]
pub enum Cursor<T: Table> {
    MdbxCursorRW(mdbx::cursor::Cursor<reth_libmdbx::RW, T>),
    MdbxCursorRO(mdbx::cursor::Cursor<reth_libmdbx::RO, T>),
    RocksDbCursor(rocksdb::cursor::Cursor<T>),
}

impl<T: Table> DbCursorRO<T> for Cursor<T> {
    fn start(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), reth_interfaces::db::DatabaseError>>
    {
        match self {
            Cursor::MdbxCursorRO(c) => c.start(start_key),
            Cursor::MdbxCursorRW(c) => c.start(start_key),
            Cursor::RocksDbCursor(c) => c.start(start_key),
        }
    }

    fn start_back(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), reth_interfaces::db::DatabaseError>>
    {
        match self {
            Cursor::MdbxCursorRO(c) => c.start_back(start_key),
            Cursor::MdbxCursorRW(c) => c.start_back(start_key),
            Cursor::RocksDbCursor(c) => c.start_back(start_key),
        }
    }

    fn start_range(
        &mut self,
        range: impl std::ops::RangeBounds<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), reth_interfaces::db::DatabaseError>>
    {
        match self {
            Cursor::MdbxCursorRO(c) => c.start_range(range),
            Cursor::MdbxCursorRW(c) => c.start_range(range),
            Cursor::RocksDbCursor(c) => c.start_range(range),
        }
    }

    fn first(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.first(),
            Cursor::MdbxCursorRW(c) => c.first(),
            Cursor::RocksDbCursor(c) => c.first(),
        }
    }

    fn seek_exact(&mut self, key: <T as Table>::Key) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.seek_exact(key),
            Cursor::MdbxCursorRW(c) => c.seek_exact(key),
            Cursor::RocksDbCursor(c) => c.seek_exact(key),
        }
    }

    fn seek(&mut self, key: <T as Table>::Key) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.seek(key),
            Cursor::MdbxCursorRW(c) => c.seek(key),
            Cursor::RocksDbCursor(c) => c.seek(key),
        }
    }

    fn next(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.next(),
            Cursor::MdbxCursorRW(c) => c.next(),
            Cursor::RocksDbCursor(c) => c.next(),
        }
    }

    fn prev(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.prev(),
            Cursor::MdbxCursorRW(c) => c.prev(),
            Cursor::RocksDbCursor(c) => c.prev(),
        }
    }

    fn last(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.last(),
            Cursor::MdbxCursorRW(c) => c.last(),
            Cursor::RocksDbCursor(c) => c.last(),
        }
    }

    fn current(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.current(),
            Cursor::MdbxCursorRW(c) => c.current(),
            Cursor::RocksDbCursor(c) => c.current(),
        }
    }

    fn walk(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<crate::cursor::Walker<'_, T, Self>, reth_interfaces::db::DatabaseError>
    where
        Self: Sized,
    {
        let start = self.start(start_key);
        Ok(Walker::new(self, start))
    }

    fn walk_range(
        &mut self,
        range: impl std::ops::RangeBounds<<T as Table>::Key>,
    ) -> Result<crate::cursor::RangeWalker<'_, T, Self>, reth_interfaces::db::DatabaseError>
    where
        Self: Sized,
    {
        let end = range.end_bound().cloned();
        let start = self.start_range(range);
        Ok(RangeWalker::new(self, start, end))
    }

    fn walk_back(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<crate::cursor::ReverseWalker<'_, T, Self>, reth_interfaces::db::DatabaseError>
    where
        Self: Sized,
    {
        let start = self.start_back(start_key);
        Ok(ReverseWalker::new(self, start))
    }
}

impl<T: DupSort> DbDupCursorRO<T> for Cursor<T> {
    fn next_dup(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.next_dup(),
            Cursor::RocksDbCursor(c) => c.next_dup(),
            Cursor::MdbxCursorRW(_) => unreachable!(),
        }
    }

    fn next_no_dup(&mut self) -> crate::common::PairResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.next_no_dup(),
            Cursor::RocksDbCursor(c) => c.next_no_dup(),
            Cursor::MdbxCursorRW(_) => unreachable!(),
        }
    }

    fn next_dup_val(&mut self) -> crate::common::ValueOnlyResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.next_dup_val(),
            Cursor::RocksDbCursor(c) => c.next_dup_val(),
            Cursor::MdbxCursorRW(_) => unreachable!(),
        }
    }

    fn seek_by_key_subkey(
        &mut self,
        key: <T>::Key,
        subkey: <T as DupSort>::SubKey,
    ) -> crate::common::ValueOnlyResult<T> {
        match self {
            Cursor::MdbxCursorRO(c) => c.seek_by_key_subkey(key, subkey),
            Cursor::RocksDbCursor(c) => c.seek_by_key_subkey(key, subkey),
            Cursor::MdbxCursorRW(_) => unreachable!(),
        }
    }

    fn walk_dup(
        &mut self,
        key: Option<<T>::Key>,
        subkey: Option<<T as DupSort>::SubKey>,
    ) -> Result<crate::cursor::DupWalker<'_, T, Self>, reth_interfaces::db::DatabaseError>
    where
        Self: Sized,
    {
        let start = self.start_dup(key, subkey)?;
        Ok(DupWalker::<'_, T, Self> { cursor: self, start })
    }

    fn start_dup(
        &mut self,
        key: Option<<T>::Key>,
        subkey: Option<<T as DupSort>::SubKey>,
    ) -> Result<
        Option<
            Result<(<T as Table>::Key, <T as Table>::Value), reth_interfaces::db::DatabaseError>,
        >,
        reth_interfaces::db::DatabaseError,
    > {
        match self {
            Cursor::MdbxCursorRW(c) => c.start_dup(key, subkey),
            Cursor::MdbxCursorRO(c) => c.start_dup(key, subkey),
            Cursor::RocksDbCursor(c) => c.start_dup(key, subkey),
        }
    }
}

impl<T: Table> DbCursorRW<T> for Cursor<T> {
    fn upsert(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.upsert(key, value),
            Cursor::RocksDbCursor(c) => c.upsert(key, value),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }

    fn insert(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.insert(key, value),
            Cursor::RocksDbCursor(c) => c.insert(key, value),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }

    fn append(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.append(key, value),
            Cursor::RocksDbCursor(c) => c.append(key, value),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }

    fn delete_current(&mut self) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.delete_current(),
            Cursor::RocksDbCursor(c) => c.delete_current(),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }
}

impl<T: DupSort> DbDupCursorRW<T> for Cursor<T> {
    fn delete_current_duplicates(&mut self) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.delete_current_duplicates(),
            Cursor::RocksDbCursor(c) => c.delete_current_duplicates(),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }

    fn append_dup(
        &mut self,
        key: <T>::Key,
        value: <T>::Value,
    ) -> Result<(), reth_interfaces::db::DatabaseError> {
        match self {
            Cursor::MdbxCursorRW(c) => c.append_dup(key, value),
            Cursor::RocksDbCursor(c) => c.append_dup(key, value),
            Cursor::MdbxCursorRO(_) => unreachable!(),
        }
    }
}
