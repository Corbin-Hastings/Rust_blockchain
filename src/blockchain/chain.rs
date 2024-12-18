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

    /// this adds a transaction to the queue and send back a block to be mined after threshhold met
    ///
    /// # Arguments
    ///
    /// * `transaction` - A `Transaction`  looking to be added.
    /// * `block_q` - A `&mut Vec<Block>` to allow for the addition of the block to the queue to be mined.
    ///
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


    /// this validates the block and returns a bool for use in another function
    ///
    /// # Arguments
    ///
    /// * `block` - A `&Block`  looking to be added to the chain.
    /// 
    ///
    pub fn validate_block(&self, block: &Block) -> bool {
        // the blocks hash is valid
        if block.hash != block.calculate_hash() {
            println!("Invalid block: Hash does not match calculated hash");
            return false;
        }

        // the blocks previous hash matches the last blocks hash
        if let Some(last_block) = self.chain.last() {
            
            if block.prev_hash != last_block.hash {
                println!("last block hash{}",last_block.hash);
                println!("new block prev hash{}",block.prev_hash);
                println!("Invalid block: Previous hash does not match last block's hash");
                return false;
            }
        } else if !block.prev_hash.is_empty() {
            //If it's the first block prev_hash should be empty
            println!("Invalid block: First block's prev_hash is not empty");
            return false;
        }

        // the block's hash meets the difficulty requirements
        let prefix = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&prefix) {
            println!("Invalid block: does not meet difficulty requirements");
            return false;
        }

        true
    }

    /// takes the validated block to the chain
    ///
    /// # Arguments
    ///
    /// * `block` - A `Block`  looking to be added to the chain.
    /// 
    /// 
    pub fn to_chain(&mut self, block: Block) {
        if self.validate_block(&block) {
            self.chain.push(block);
            println!("Block added to the chain");
        } else {
            println!("Block Validation failed");
        }
    }
}

