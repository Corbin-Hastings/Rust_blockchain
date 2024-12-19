use std::sync::{Arc, Mutex};
use std::{thread,time};

use crate::blockchain::transaction::Transaction;

use crate::blockchain::hashing::hash;



#[derive(Debug,Clone)]
/// The Block struct is a single block in a blockchain.
/// Each block contains a set of transactions, a reference to the previous block's hash,
/// a nonce for mining, its index in the blockchain, and its own hash.
pub struct Block{
    /// a list of the transactions in the block
    pub transactions: Vec<Transaction>,

    /// a string that represents the hash of the block before this one in the change
    pub prev_hash: String,

    /// the number that is used to genorate the correct hash
    pub nonce: f64,

    /// the location of the block in the chain
    pub index: i64,

    /// the hash of this block
    pub hash: String,
}

impl Block {

    /// Creates a new block with the specified transactions, previous hash, and index.
    ///
    /// # Arguments
    ///
    /// * `transactions` - A vector of transactions to include in the block.
    /// * `prev_hash` - The hash of the previous block in the chain.
    /// * `index` - The index of this block in the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Block` instance with a computed hash.
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

    /// Calculates the hash for the block based on its contents.
    ///
    /// This function uses the nonce, transactions, previous hash, and index to compute the hash.
    ///
    /// # Returns
    ///
    /// A `String` representing the hash of the block.
    pub fn calculate_hash(&self)->String{
        let nonce = self.nonce;
        let transactions = &self.transactions;
        let prev = &self.prev_hash;
        let index = self.index;
        //collects all of the info used to make the hash
        let hash_in = format!("{}{:?}{}{}",
            nonce,
            transactions,
            prev,
            index
        );
        hash(&hash_in)
    }

    /// Mines the block by finding a valid hash that starts with a certain number of zeros = to the difficulty.
    ///
    /// designed to be used in more than one thread
    /// Each miner starts at a different nonce and iterates to find a valid hash.
    /// miners dont cover the same numbers becaue each miner mines: (0)+id, n+id, 2n+id, 3n+id ....
    ///
    /// # Arguments
    ///
    /// * `difficulty` - The number of leading zeros required in the hash.
    /// * `miner_id` - The unique identifier of the miner working to mine the block.
    /// * `total_miners` - The total number of miners working at the same time
    /// * `done` - A shared flag to signal when a block has been mined by any miner.
    ///
    /// # Returns
    ///
    /// An `Option<Block>` which contains the mined block if successful or `None` if another miner has already completed the task.
    pub fn mine_block(&mut self,difficulty:usize,miner_id:i128,total_miners:i128,done:Arc<Mutex<bool>>)->Option<Block>{
        self.nonce = (0+miner_id)as f64;
        let mut itter:i128 = 1;
        let prefix = "0".repeat(difficulty);//cite chatgpt
        while !self.hash.starts_with(&prefix){
            if *done.lock().unwrap()==true {
                return Option::None;
            }
            self.nonce = ((total_miners*itter)+miner_id) as f64;
            self.hash = self.calculate_hash();
            itter+=1;
            
            
        }
        println!("Miner {} has mined the block",miner_id);
        *done.lock().unwrap() = true;
        Option::Some(self.clone())
    }

}