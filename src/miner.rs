use std::sync::{Arc, Mutex};

use crate::blockchain::{block::Block, transaction::Transaction};
/// this still needs to be implmented 
/// miner would be used to represent the user
/// holding their id, balence, transaction history, and a local copy of the chain to check
pub struct Miner{
    pub id:i128,
    pub balence:f64,
    pub history:Vec<Transaction>,
    pub local_chain:Vec<Block>,
}
/// this is the setup for the multi threaded mining
///
/// acts to wrap the mine_block
/// this is to allow for more abstaraction
/// also allows for more flexiblility  when implementing miners  
///
/// # Arguments
///
/// * `block` - A mutable reference to the `Block` to be mined.
/// * `diff` - The difficulty level of mining represented as the number of leading zeros required in the hash.
/// * `id` - The unique identifier of the miner attempting to mine the block.
/// * `total` - The total number of miners working.
/// * `done` - A shared bool to signal when a block has been mined by any miner.
///
/// # Returns
///
/// An `Option<Block>` which contains the mined block if successful or `None` if another miner has already completed the task.
pub fn mine_multi(block:&mut Block,diff:usize,id:i128,total:i128,done:Arc<Mutex<bool>>)->Option<Block> {
    block.mine_block(diff,id,total,done)
}

impl Miner {
   /*  pub fn new(id:i128)->miner{
        miner{
            id,
            balence:10.0,
            history:Vec::new(),
            local_chain:Vec::new(),
        }
    } */
}