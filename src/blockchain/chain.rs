use std::sync::{Arc, Mutex};
use rand::seq::index;

use crate::blockchain::transaction::Transaction;

use super::{block::Block, transaction};
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
    pub fn add_transaction(&mut self,transaction:Transaction,block_q:&mut Vec<Block>){
        self.transaction_queue.push(transaction);
        if self.transaction_queue.len()>3 {
            let mut prev = String::from("");
            let mut index = 0;
            match self.chain.last(){
                Some(_)=>prev = self.chain.last().unwrap().hash.clone(),
                None=>prev = String::from(""),
            }
            match self.chain.last(){
                Some(_)=>index = self.chain.last().unwrap().index+1,
                None=>index = 0,
            }
             let block = Block::new(self.transaction_queue.clone(), prev, index);
             block_q.push(block);
             println!("block added to queue");
             self.transaction_queue = Vec::new();
        }
    }
}

