use std::sync::Mutex;
use crate::blockchain::transaction::Transaction;

use super::block::Block;
#[derive(Debug)]
pub struct Blockchain<'a> {
    //pub transaction_queue: Mutex<Vec<&'a Transaction>>,
    pub transaction_queue: Vec<Transaction>,
    pub difficulty: usize,
    pub chain: Mutex<Vec<Block<'a>>>,
}

impl<'a> Blockchain<'a>{
    pub fn new(difficulty:usize)->Blockchain<'a>{
        Blockchain{
            //transaction_queue: Mutex::new(Vec::new()),
            transaction_queue: Vec::new(),
            difficulty,
            chain: Mutex::new(Vec::new()),
        }
    }
}

