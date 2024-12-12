
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

impl Transaction{
    pub fn new(sender:String, receiver:String, amount:f64)->Option<Transaction>{
        Some(Transaction{
            sender,
            receiver,
            amount,
        })
    }
}