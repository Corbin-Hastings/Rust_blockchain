mod blockchain;
mod miner;

use std::thread;

use blockchain::{block::Block, chain::Blockchain, transaction::{self, Transaction}};
use miner::{mine_single,mine_multi};



fn main() {

    let mut chain = Blockchain::new(3); //creates chain at the begenning of program

    
    //setting up test transactions
    let transaction1 = Transaction::new("slkdfs".to_string(),
     "dsfg".to_string(), 12.3);
    let transaction2 = Transaction::new("dsfsadf".to_string(),
     "dsfg".to_string(), 12.3);
    let transaction3 = Transaction::new("dgsfdsfg".to_string(),
     "corbin".to_string(), 12.3);
    let transaction4 = Transaction::new("dsfg".to_string(),
     "vbdsfgd".to_string(), 12.3);

     //queuing transactions
    chain.transaction_queue.get_mut().unwrap().push(&transaction1);
    chain.transaction_queue.get_mut().unwrap().push(&transaction2);
    chain.transaction_queue.get_mut().unwrap().push(&transaction3);
    chain.transaction_queue.get_mut().unwrap().push(&transaction4);

    //init first block to hash out trasaction queue
    let mut block = Block::new(Vec::new(), 
        "prev_hash".to_string(), 0);
    //push queue to the block to get calculated and recorded
    for i in chain.transaction_queue.get_mut().unwrap() {
        block.transactions.push(i);
    }

    println!("block before mining: {:?}",block);

    //calling mine opperation to mine block. this is still all mostly testing
    block.mine_block(chain.difficulty);//5 takes 5 mins 6 takes 10ish on one thread 

    println!("block after mining: {:?}",block);

    //adding that calculated block to the chain to record the transactions inside.
    chain.chain.get_mut().unwrap().push(block);

    println!("{:?}",chain);

    ////////////////////////////////////////////////////////////////////////////////////////
    //mine_single(&mut block, diff);
    let mut  num_threads = 4;

    let mut handles = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            println!("miner {} starting",i);
            chain.chain.get_mut().unwrap().push(mine_multi(&mut block.clone(), chain.difficulty, i as i128, num_threads as i128));
            println!("miner {} done!",1);
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

}
