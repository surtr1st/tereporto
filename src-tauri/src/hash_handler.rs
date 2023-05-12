#![allow(dead_code, unused)]
use hex;
use sha2::{Digest, Sha256};

pub struct HashHandler;

impl HashHandler {
    pub fn encrypt(target: &str) -> String {
        let hash = Sha256::new().chain_update(target.as_bytes()).finalize();
        hex::encode(hash)
    }

    pub fn compare(target: &str, original: &str) -> bool {
        let from_hashed = hex::decode(target).unwrap();
        let from_original = Sha256::new().chain_update(original).finalize();
        from_original.as_slice() == from_hashed
    }
}
