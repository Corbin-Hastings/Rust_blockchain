
#[derive(Debug,Clone)]
pub struct Transaction {
    pub sender: i32,
    pub receiver: i32,
    pub amount: f64,
}

impl Transaction{
    pub fn new(sender:i32, receiver:i32, amount:f64)->Transaction{
        Transaction{
            sender,
            receiver,
            amount,
        }
    }
}