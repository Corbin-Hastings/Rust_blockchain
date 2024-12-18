use std::sync::Mutex;
use crate::blockchain::transaction::Transaction;

use super::block::Block;
#[derive(Debug)]
pub struct Blockchain {
    //pub transaction_queue: Mutex<Vec<&'a Transaction>>,
    pub transaction_queue: Vec<Transaction>,
    pub difficulty: usize,
    pub chain: Vec<Block>,
}

impl Blockchain{
    pub fn new(difficulty:usize)->Blockchain{
        Blockchain{
            //transaction_queue: Mutex::new(Vec::new()),
            transaction_queue: Vec::new(),
            difficulty,
            chain: Vec::new(),
        }
    }
}

