pub fn handle_threads() {

}

pub fn mining(){
    let num_threads = threads as usize;

    let mut handles = Vec::with_capacity(num_threads);

    let done =  Arc::new(Mutex::new(false));

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

        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}