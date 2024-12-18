use sha2::{Digest,Sha256};
/// hash is the fucntion that takes in the data used to make the hash and returns the hash as a string
/// using the SHA-256 hashing algorithm.
///
/// # Arguments
///
/// * `data` - A string slice containing the input data to be hashed.
///
/// # Returns
///
/// A `String` representing the hexadecimal representation of the hash.
pub fn hash(data: &str)->String{
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]

//hasing is working
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input = "Hello, blockchain!";
        let hash_result = hash(input);
        

        assert_eq!(
            hash_result,
            "e485541186cd67682999c1ad80eac78fb803ec57885c26d5489efb01f15ae913"
        );
    }
}