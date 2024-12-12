pub struct Blockchain {
    transaction_queue: Mutex<Vec<Transaction>>,
    difficulty: usize,
}

impl Blockchain{
    pub fn new(difficulty:usize)->Blockchain{
        Blockchain{
            transaction_queue: Mutex::new(Vec::new()),
            difficulty,
        }
    }
}

// tests here at some point