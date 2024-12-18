use sha2::{Digest,Sha256};

pub fn hash(data: &str)->String{
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input = "Hello, blockchain!";
        let hash_result = hash(input);

        assert_eq!(
            hash_result,
            "6b7b8bf2b5af732ff634b15f3c7431c42599714cdb51c48c891b71cfad2f8e78"
        );
    }
} */