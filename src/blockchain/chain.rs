struct Blockchain {
    transaction_queue: Mutex<Vec<Transaction>>,
    difficulty: usize,
}