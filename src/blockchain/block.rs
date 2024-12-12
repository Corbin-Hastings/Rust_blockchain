use serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;

pub struct Block{
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(transactions:Vec<Transaction>,prev_hash:String,hash:String) -> Option<Block> {
        Some(Block{
            transaction,
            prev_hash,
            hash,
        })
    }

    pub fn mine_block(){
        
    }
}