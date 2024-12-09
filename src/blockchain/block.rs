use serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;

pub struct BLock{
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
}

