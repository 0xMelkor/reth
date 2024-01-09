use core::fmt;
use reth_interfaces::db::DatabaseError;
use rocksdb::TransactionDB;
use std::{
    sync::{
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
    thread,
};

type GetResponse = Result<Option<Vec<u8>>, rocksdb::Error>;
enum TransactionCommands {
    Get { key: Vec<u8>, response: SyncSender<GetResponse> },
}

#[derive(Debug)]
pub struct Transaction {
    inner: TransactionInner,
}

impl Transaction {
    pub fn rw(db: Arc<TransactionDB>) -> Self {
        let (tx, rx) = sync_channel::<TransactionCommands>(10);
        thread::spawn(move || {
            let snap = db.snapshot();
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    TransactionCommands::Get { key, response } => {
                        let value = snap.get(key);
                        if response.send(value).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        Self { inner: TransactionInner::RW(tx) }
    }

    pub fn ro(db: Arc<TransactionDB>) -> Self {
        // TODO implement
        Self::rw(db)
    }

    pub fn get(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>, DatabaseError> {
        let (response_tx, response_rx) = sync_channel(0);
        let sender = self.to_manager();
        // TODO handle error
        let _ = sender.send(TransactionCommands::Get { key, response: response_tx });
        // TODO handle error
        let value = response_rx.recv().unwrap();
        value.map_err(|e| DatabaseError::Read(-13))
    }

    fn to_manager(&self) -> SyncSender<TransactionCommands> {
        match &self.inner {
            TransactionInner::RW(sender) => sender.clone(),
            TransactionInner::RO(sender) => sender.clone(),
        }
    }
}

enum TransactionInner {
    RW(SyncSender<TransactionCommands>),
    RO(SyncSender<TransactionCommands>),
}

impl fmt::Debug for TransactionInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RW(_) => f.debug_tuple("RW").finish(),
            Self::RO(_) => f.debug_tuple("RO").finish(),
        }
    }
}
