use std::sync::{Arc, Mutex};
use std::{thread,time};

//use crate::serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::chain::Blockchain;
use crate::blockchain::hashing::hash;

use super::transaction;

#[derive(Debug,Clone)]
pub struct Block{
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub nonce: f64,
    pub index: i64,
    pub hash: String,
}

impl Block {
    pub fn new(transactions:Vec<Transaction>,prev_hash:String,index:i64) -> Block {
        let mut block = Block{
            transactions,
            prev_hash,
            nonce: 0.0,
            index,
            hash: "".to_string(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self)->String{
        let nonce = self.nonce;
        let transactions = &self.transactions;
        let prev = &self.prev_hash;
        let index = self.index;

        let hash_in = format!("{}{:?}{}{}",
            nonce,
            transactions,
            prev,
            index
        );
        hash(&hash_in)
    }

    pub fn mine_block(&mut self,difficulty:usize,miner_id:i128,total_miners:i128,done:&mut bool)->Option<Block>{
        self.nonce = (0+miner_id)as f64;
        let mut itter:i128 = 1;
        let prefix = "0".repeat(difficulty);//cite chatgpt
    
        while !self.hash.starts_with(&prefix){

            if *done==true {
                return Option::None;
            }
            self.nonce = ((total_miners*itter)+miner_id) as f64;
            self.hash = self.calculate_hash();
            itter+=1;
        }
        println!("Miner {} has mined the block",miner_id);
        *done = true;
        Option::Some(self.clone())
    }

}