use crate::database::Database;

pub mod cursor;
pub mod tx;


/// Wrapper for the libmdbx environment: [Environment]
#[derive(Debug)]
pub struct DatabaseEnv {
    inner: ()
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
