//use crate::serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::chain::Blockchain;
use crate::blockchain::hashing::hash;

#[derive(Debug)]
pub struct Block{
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: f64,
    pub index: f64,
}

impl Block {
    pub fn new(transactions:Vec<Transaction>,prev_hash:String,hash:String) -> Option<Block> {
        Some(Block{
            transactions,
            prev_hash,
            hash,
            nonce: 0.0,
            index: 0.0,
        })
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

    pub fn mine_block(&mut self,difficulty:usize){
        let prefix = "0".repeat(difficulty);//cite chatgpt
        while !self.hash.starts_with(&prefix){
            self.nonce +=1.0;
            self.hash = self.calculate_hash();
        }
    }

/*     pub fn calculate_hash(&self)->String{
        let hash_in = format!({:?}{}{}{},
            self.transactions,
            self.prev_hash,
            self.nonce,
            self.index
        );
        hash(&hash_in)
    } */
}