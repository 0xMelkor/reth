use std::{marker::PhantomData, ops::RangeBounds};

use reth_interfaces::db::DatabaseError;

use crate::{
    common::{PairResult, ValueOnlyResult},
    cursor::{
        DbCursorRO, DbCursorRW, DbDupCursorRO, DbDupCursorRW, DupWalker, RangeWalker,
        ReverseWalker, Walker,
    },
    table::{DupSort, Table},
};

/// Cursor that iterates over table
#[derive(Debug)]
pub struct Cursor<T: Table> {
    cursor: u32,
    _dbi: PhantomData<T>,
}

impl<T: Table> Cursor<T> {
    pub fn new(cursor: u32) -> Self {
        Self { cursor, _dbi: PhantomData }
    }
}

impl<T: Table> DbCursorRO<T> for Cursor<T> {
    fn start(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), DatabaseError>> {
        todo!()
    }

    fn start_back(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), DatabaseError>> {
        todo!()
    }

    fn start_range(
        &mut self,
        range: impl RangeBounds<<T as Table>::Key>,
    ) -> Option<Result<(<T as Table>::Key, <T as Table>::Value), DatabaseError>> {
        todo!()
    }
    
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

    fn walk(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<Walker<'_, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn walk_range(
        &mut self,
        range: impl RangeBounds<<T as Table>::Key>,
    ) -> Result<RangeWalker<'_, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn walk_back(
        &mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<ReverseWalker<'_, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<T: DupSort> DbDupCursorRO<T> for Cursor<T> {
    fn start_dup(
        &mut self,
        key: Option<<T>::Key>,
        subkey: Option<<T as DupSort>::SubKey>,
    ) -> Result<Option<Result<(<T as Table>::Key, <T as Table>::Value), DatabaseError>>, DatabaseError>  {
        todo!()
    }
    
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

impl<T: Table> DbCursorRW<T> for Cursor<T> {
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

impl<T: DupSort> DbDupCursorRW<T> for Cursor<T> {
    fn delete_current_duplicates(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn append_dup(&mut self, _key: <T>::Key, _value: <T>::Value) -> Result<(), DatabaseError> {
        Ok(())
    }
}
