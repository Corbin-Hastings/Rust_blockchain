
#[derive(Debug,Clone)]
/// Represents a single transaction in the blockchain.
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}
/// Creates a new transaction with the given sender, receiver, and amount.
///
/// # Arguments
///
/// * `sender` - A `String` that is the sender's identifier.
/// * `receiver` - A `String` that is the receiver's identifier.
/// * `amount` - A `f64` that is  the amount being transferred.
///
/// # Returns
///
/// A `Transaction` instance with the specified details.
impl Transaction{
    pub fn new(sender:String, receiver:String, amount:f64)->Transaction{
        Transaction{
            sender,
            receiver,
            amount,
        }
    }
}