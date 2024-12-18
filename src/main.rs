mod blockchain;
mod miner;
mod threads;
use std::{io, thread::Thread};

use core::num;
use std::{sync::{mpsc, Mutex}, thread, time};

use blockchain::{block::Block, chain::{self, Blockchain}, transaction::{self, Transaction}};
use miner::{mine_multi};
use std::sync::Arc;
use rand::Rng;



fn main() {

    let mut thread_input = String::new();
    println!("how many threads do you want to use:");
    io::stdin().read_line(&mut thread_input);
    let threads: i32 = thread_input.trim().parse().expect("Please enter a number");
    println!("loading up {} threads", threads );

    let mut diff_input = String::new();
    println!("how hard do you want the hash to be (stay below 5):");
    io::stdin().read_line(&mut diff_input);
    let diff: i32 = diff_input.trim().parse().expect("Please enter a number");
    println!("Diff set to: {} ", diff );

    let mut block = Block::new(Vec::new(), 
        "none".to_string(), 0);


    let num_threads = threads as usize;

    let mut handles = Vec::with_capacity(num_threads);

    let start_chain = Arc::new(Mutex::new(Blockchain::new(diff as usize))); //creates chain at the begenning of program
    let block_queue: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(vec![block.clone()]));



    


    let chain_clone = Arc::clone(&start_chain);
    let queue_clone = Arc::clone(&block_queue);
    let handle = thread::spawn(move || {
        let num_threads = threads.clone();
        for i in 0..10 {
            let mut rng = rand::thread_rng();
            let sender = rng.gen_range(0..num_threads);
            let receiver = rng.gen_range(0..num_threads);
            let amount = rng.gen_range(0..100);
            let transaction = Transaction::new(sender, receiver, amount as f64);

            chain_clone.lock().unwrap().add_transaction(transaction,&mut queue_clone.lock().unwrap());
            println!("transaction added to queue");
            thread::sleep(time::Duration::from_secs(3));
        }
    });
    handles.push(handle);

    //init first block to hash out trasaction queue
    let mut block = Block::new(Vec::new(), 
        "prev_hash".to_string(), 0);

    //push queue to the block to get calculated and recorded
    for i in start_chain.lock().unwrap().transaction_queue.clone() {
        block.transactions.push(i);
    }
/*     println!("chain before{:?}",start_chain);
    println!("block before mining: {:?}",block);
 */
    ////////////////////////////////////////////////////////////////////////////////////////

    

    let done =  Arc::new(Mutex::new(false));

    for i in 0..num_threads {
        let block_q = Arc::clone(&block_queue);
        let done_clone = Arc::clone(&done);
        let done_clone_reset = Arc::clone(&done);
        let chain_clone = Arc::clone(&start_chain);
        let mut block_clone = block.clone();

        //let (tx, rx) = mpsc::channel();
        
        let handle = thread::spawn(move || {
            println!("miner {} starting",i);
            loop {
            if block_q.lock().unwrap().is_empty() {
                thread::sleep(time::Duration::from_secs(2));
                println!("no blocks to mine, sleeping for 2 sec")
            }else{
            *done_clone_reset.lock().unwrap() = false;
            let block = &mut block_q.lock().unwrap().first().unwrap().clone();
            block_q.lock().unwrap().pop();
            let diff = chain_clone.lock().unwrap().difficulty;
          
            
            let mined_block= mine_multi(block, diff, i as i128, num_threads as i128,&mut done_clone.lock().unwrap());
            match mined_block {
                Some(block)=>{
                    chain_clone.lock().unwrap().chain.push(block);
                    println!("miner {} mined and retured the block",i);
                    thread::sleep(time::Duration::from_secs(3));},
                None=> {println!("Thread {} lost and is going to close",i);}
            } 
            }}
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("chain after{:?}",start_chain);
}
