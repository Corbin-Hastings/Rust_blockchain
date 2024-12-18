mod blockchain;
mod miner;

use core::num;
use std::{sync::{mpsc, Mutex}, thread, time};

use blockchain::{block::Block, chain::{self, Blockchain}, transaction::{self, Transaction}};
use miner::{mine_multi};
use std::sync::Arc;



fn main() {

    let mut start_chain = Arc::new(Mutex::new(Blockchain::new(8))); //creates chain at the begenning of program

   
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
    let mut block = Block::new(Vec::new(), 
        "prev_hash".to_string(), 0);
    //push queue to the block to get calculated and recorded
    for i in start_chain.lock().unwrap().transaction_queue.clone() {
        block.transactions.push(i);
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
    let mut  num_threads = 8;

    let mut handles = Vec::with_capacity(num_threads);

    let mut done =  Arc::new(Mutex::new(false));

    for i in 0..num_threads {
        let done_clone = Arc::clone(&done);
        let chain_clone = Arc::clone(&start_chain);
        let mut block_clone = block.clone();

        //let (tx, rx) = mpsc::channel();
        
        let handle = thread::spawn(move || {
           let block = &mut block_clone;
            let diff = chain_clone.lock().unwrap().difficulty;
          
            println!("miner {} starting",i);
            let mined_block= mine_multi(block, diff, i as i128, num_threads as i128,done_clone);
            match mined_block {
                Some(block)=>{
                    chain_clone.lock().unwrap().chain.push(block);
                    println!("miner {} mined and retured the block",i);},
                None=> {println!("Thread {} lost and is going to close",i);}
            } 

 /*            block_clone.lock().unwrap().nonce = (0+i)as f64;
            let mut itter:i128 = 1;
            let prefix = "0".repeat(chain_clone.lock().unwrap().difficulty);//cite chatgpt
            let mut done_val = done_clone.lock().unwrap();
            println!("broken ?");
            while !block_clone.lock().unwrap().hash.starts_with(&prefix){
                //let done_val = done.lock().unwrap();
                if *done_val==true {
                    return Option::None;
                }
                block_clone.lock().unwrap().nonce = (((num_threads as i128)*itter)+i)as f64;
                block_clone.lock().unwrap().hash = block_clone.lock().unwrap().calculate_hash();
                itter+=1;
                println!("Miner {}  is on itter {}",i,itter);
                thread::sleep(time::Duration::from_secs(2));
            }
            println!("Miner {} has mined the block",i);
            *done_val = true;
            Option::Some(block_clone.lock().unwrap().clone())
            
             */
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }
    println!("chain after{:?}",start_chain);
}
