mod blockchain;
mod miner;

use std::{sync::Mutex, thread};

use blockchain::{block::Block, chain::Blockchain, transaction::{self, Transaction}};
use miner::{mine_multi};
use std::sync::Arc;



fn main() {

    let mut start_chain = Arc::new(Mutex::new(Blockchain::new(5))); //creates chain at the begenning of program

   
    //setting up test transactions
    {    let transaction1: Transaction = Transaction::new("sadfasdfasdf".to_string(),
    "gfgdfgsdfvdfge".to_string(), 34.343);
   let transaction2 = Transaction::new("dsfsadf".to_string(),
    "dsfg".to_string(), 12.3);
   let transaction3 = Transaction::new("dgsfdsfg".to_string(),
    "corbin".to_string(), 12.3);
   let transaction4 = Transaction::new("dsfg".to_string(),
    "vbdsfgd".to_string(), 12.3);
    start_chain.lock().unwrap().transaction_queue.push(transaction1.clone());
}

     //queuing transactions

   /*  chain.transaction_queue.get_mut().unwrap().push(&transaction2.clone());
    chain.transaction_queue.get_mut().unwrap().push(&transaction3.clone());
    chain.transaction_queue.get_mut().unwrap().push(&transaction4.clone()); */

    //init first block to hash out trasaction queue
    let mut block = Arc::new(Mutex::new(Block::new(Vec::new(), 
        "prev_hash".to_string(), 0)));
    //push queue to the block to get calculated and recorded
    for i in start_chain.lock().unwrap().transaction_queue.clone() {
        block.lock().unwrap().transactions.push(i);
    }
    println!("chain before{:?}",start_chain);
    println!("block before mining: {:?}",block);
/* 
    //calling mine opperation to mine block. this is still all mostly testing
    block.mine_block(chain.difficulty);//5 takes 5 mins 6 takes 10ish on one thread 

    println!("block after mining: {:?}",block);

    //adding that calculated block to the chain to record the transactions inside.
    chain.chain.get_mut().unwrap().push(block);

    println!("{:?}",chain); */

    ////////////////////////////////////////////////////////////////////////////////////////
    //mine_single(&mut block, diff);
    let mut  num_threads = 4;

    let mut handles = Vec::with_capacity(num_threads);

    let mut done =  Arc::new(Mutex::new(false));

    for i in 0..num_threads {
        let done_clone = Arc::clone(&done);
        let chain_clone = Arc::clone(&start_chain);
        let block_clone = Arc::clone(&block);
        let handle = thread::spawn(move || {
            let bloc = &mut block_clone.lock().unwrap().clone();
          
            println!("miner {} starting",i);
            let mined_block= mine_multi(bloc, chain_clone.lock().unwrap().difficulty, i as i128, num_threads as i128,done_clone);
            match mined_block {
                Some(block)=>{chain_clone.lock().unwrap().chain.push(block);
                    println!("miner {} mined and retured the block",i);},
                None=> {println!("Thread {} lost and is going to close",i);}
            }
            
            
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }
    println!("chain after{:?}",start_chain);
}
