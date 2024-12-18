use std::sync::{Arc, Mutex};

use crate::blockchain::{block::{self, Block}, transaction::Transaction};
//not fully sure i need this yet but here we are
pub struct miner{
    pub id:i128,
    pub balence:f64,
    pub history:Vec<Transaction>,
    pub local_chain:Vec<Block>,
}

pub fn mine_multi(block:&mut Block,diff:usize,id:i128,total:i128,done:&mut bool)->Option<Block> {
    block.mine_block(diff,id,total,done)
}

impl miner {
    pub fn new(id:i128)->miner{
        miner{
            id,
            balence:10.0,
            history:Vec::new(),
            local_chain:Vec::new(),
        }
    }
}