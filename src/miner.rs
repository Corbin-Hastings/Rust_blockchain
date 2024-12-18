use std::sync::{Arc, Mutex};

use crate::blockchain::{block::{self, Block}, transaction::Transaction};
//not fully sure i need this yet but here we are
pub struct miner{
    pub id:String,
    pub balence:f64,
    pub history:Vec<Transaction>,
}


 //its time to cook
/*  pub fn mine_single(block:&mut Block,diff:usize) {
    block.mine_block(diff,0,1);
}
 */
pub fn mine_multi<'a>(block:&'a mut Block<'a>,diff:usize,id:i128,total:i128,done:Arc<Mutex<bool>>)->Option<Block<'a>> {
    block.mine_block(diff,id,total,done)
}
/* impl miner {
    pub fn mine_single(block:&mut Block,diff:usize) {
        block.mine_block(diff,0,1);
    }

    pub fn mine_multi(block:&mut Block,diff:usize,id:i128,total:i128) {
        block.mine_block(diff,miner,total);
    }
} */