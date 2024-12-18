mod blockchain;
mod miner;
use std::{io, thread};
use std::sync::Mutex;
use blockchain::{block::Block, chain::Blockchain, transaction::Transaction};
use miner::mine_multi;
use std::sync::Arc;

/// Main function docs
/// 
/// This main function is the starting point of the program
/// it sets up all of the logic and takes in all of the user input
/// in further itterations the idea would be to move some of these opperations
/// there own files
/// 
/// Main opperations:
/// 
/// take in user input for the #of threads
/// take in user input for the difficulty
/// 
/// initilization of the blockchain and blocks and example transations
/// 
/// sets up logic for spawning threads to mine the blocks.
/// 
/// this is my first time writing documentation, I am trying my best here



fn main() {

     

       // Ask the user for the number of threads
       println!("Enter the number of threads:");
       let mut thread_input = String::new();
       io::stdin().read_line(&mut thread_input).expect("Failed to read input");
       let num_threads: usize = thread_input.trim().parse().expect("Please enter a valid number");
   
       // Ask the user for the difficulty setting
       println!("Enter the difficulty setting:");
       let mut diff_input = String::new();
       io::stdin().read_line(&mut diff_input).expect("Failed to read input");
       let difficulty: u32 = diff_input.trim().parse().expect("Please enter a valid number");
   
       let start_chain = Arc::new(Mutex::new(Blockchain::new(difficulty as usize)));

    //setting up test transaction
    {    let transaction1: Transaction = Transaction::new("Corsdafasbin".to_string(),
    "My_graveteehheasfdee".to_string(), 14543200.00);
    start_chain.lock().unwrap().transaction_queue.push(transaction1.clone());
}



    //init first block to hash out trasaction queue
    let mut block = Block::new(Vec::new(), 
        "".to_string(), 0);
    //push queue to the block to get calculated and recorded
    for i in start_chain.lock().unwrap().transaction_queue.clone() {
        block.transactions.push(i);
    }
    //init transaction 2 and block 2
    {    let transaction2: Transaction = Transaction::new("testing123".to_string(),
    "paying".to_string(), 1400.00);
    start_chain.lock().unwrap().transaction_queue.push(transaction2.clone());
}
    let mut block1 = Block::new(Vec::new(), 
    "".to_string(), 0);

    for i in start_chain.lock().unwrap().transaction_queue.clone() {
        block1.transactions.push(i);
    }
    //init block_queue with our 2 premade blocks
    let block_queue:Vec<&mut Block> = vec![&mut block,&mut block1];

    //loop to get through the block queue
    for b in block_queue{
    let mut handles = Vec::with_capacity(num_threads);

    let done =  Arc::new(Mutex::new(false));
        //setting last hash if the block is not first
    if start_chain.lock().unwrap().chain.len()>0 {
        b.prev_hash = start_chain.lock().unwrap().chain.last().unwrap().hash.clone();
    }
    //loop for spawning threads
    for i in 0..num_threads {
       // let block_q = Arc::clone(&block_queue);
        let done_clone = Arc::clone(&done);
        let chain_clone = Arc::clone(&start_chain);
        let mut block_clone = b.clone();

        
        //spawn threads
        let handle = thread::spawn(move || {
            
            let block = &mut block_clone;
            let diff = chain_clone.lock().unwrap().difficulty;
          
            println!("miner {} starting",i);
            let mined_block= mine_multi(block, diff, i as i128, num_threads as i128,done_clone);
            match mined_block {
                Some(block)=>{println!("miner {} mined and retured the block with nonce {} and hash {}",i,block.nonce ,block.hash );
                chain_clone.lock().unwrap().to_chain(block);},
                None=> {println!("Thread {} lost and is going to close",i);}
            }
            
            
        });

        handles.push(handle);
    }
//wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }}
}
