use std::ops::{Bound, RangeBounds};

use reth_interfaces::db::DatabaseError;

use crate::{
    common::{IterPairResult, PairResult, ValueOnlyResult},
    cursor::{
        DbCursorRO, DbCursorRW, DbDupCursorRO, DbDupCursorRW, DupWalker, RangeWalker,
        ReverseWalker, Walker,
    },
    table::{DupSort, Table},
};

/// Cursor that iterates over table
#[derive(Debug)]
pub struct Cursor {
    pub _cursor: u32,
}

impl<T: Table> DbCursorRO<T> for Cursor {
    fn first(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn seek_exact(&mut self, _key: T::Key) -> PairResult<T> {
        Ok(None)
    }

    fn seek(&mut self, _key: T::Key) -> PairResult<T> {
        Ok(None)
    }

    fn next(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn prev(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn last(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn current(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn walk(&mut self, start_key: Option<T::Key>) -> Result<Walker<'_, T, Self>, DatabaseError> {
        let start: IterPairResult<T> = match start_key {
            Some(key) => <Cursor as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Cursor as DbCursorRO<T>>::first(self).transpose(),
        };

        Ok(Walker::new(self, start))
    }

    fn walk_range(
        &mut self,
        range: impl RangeBounds<T::Key>,
    ) -> Result<RangeWalker<'_, T, Self>, DatabaseError> {
        let start_key = match range.start_bound() {
            Bound::Included(key) | Bound::Excluded(key) => Some((*key).clone()),
            Bound::Unbounded => None,
        };

        let end_key = match range.end_bound() {
            Bound::Included(key) | Bound::Excluded(key) => Bound::Included((*key).clone()),
            Bound::Unbounded => Bound::Unbounded,
        };

        let start: IterPairResult<T> = match start_key {
            Some(key) => <Cursor as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Cursor as DbCursorRO<T>>::first(self).transpose(),
        };

        Ok(RangeWalker::new(self, start, end_key))
    }

    fn walk_back(
        &mut self,
        start_key: Option<T::Key>,
    ) -> Result<ReverseWalker<'_, T, Self>, DatabaseError> {
        let start: IterPairResult<T> = match start_key {
            Some(key) => <Cursor as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Cursor as DbCursorRO<T>>::last(self).transpose(),
        };
        Ok(ReverseWalker::new(self, start))
    }
}

impl<T: DupSort> DbDupCursorRO<T> for Cursor {
    fn next_dup(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn next_no_dup(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn next_dup_val(&mut self) -> ValueOnlyResult<T> {
        Ok(None)
    }

    fn seek_by_key_subkey(
        &mut self,
        _key: <T as Table>::Key,
        _subkey: <T as DupSort>::SubKey,
    ) -> ValueOnlyResult<T> {
        Ok(None)
    }

    fn walk_dup(
        &mut self,
        _key: Option<<T>::Key>,
        _subkey: Option<<T as DupSort>::SubKey>,
    ) -> Result<DupWalker<'_, T, Self>, DatabaseError> {
        Ok(DupWalker { cursor: self, start: None })
    }
}

impl<T: Table> DbCursorRW<T> for Cursor {
    fn upsert(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn insert(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn append(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete_current(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }
}

impl<T: DupSort> DbDupCursorRW<T> for Cursor {
    fn delete_current_duplicates(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn append_dup(&mut self, _key: <T>::Key, _value: <T>::Value) -> Result<(), DatabaseError> {
        Ok(())
    }
}
