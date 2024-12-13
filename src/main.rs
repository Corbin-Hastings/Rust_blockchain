mod blockchain;

use blockchain::{block::Block, chain::Blockchain, transaction::{self, Transaction}};



fn main() {
    let transaction1 = Transaction::new("corbin".to_string(),
     "corbin".to_string(), 12.3);

    let mut block = Block::new(vec![transaction1], 
        "prev_hash".to_string(), 0);

    println!("block before mining: {:?}",block);

    block.mine_block(6);//5 is too much

    println!("block after mining: {:?}",block);

}
