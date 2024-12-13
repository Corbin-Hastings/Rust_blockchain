mod blockchain;

use blockchain::{block::Block, chain::Blockchain, transaction::{self, Transaction}};



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
    block.mine_block(2);//5 takes 5 mins 6 takes 10ish

    println!("block after mining: {:?}",block);

    //adding that calculated block to the chain to record the transactions inside.
    chain.chain.get_mut().unwrap().push(block);

    println!("{:?}",chain);

}
