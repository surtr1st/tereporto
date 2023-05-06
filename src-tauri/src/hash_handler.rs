use bcrypt::{hash, verify};

pub struct HashHandler;

impl HashHandler {
    pub fn encrypt(target: &str) -> String {
        hash(target, 7).unwrap()
    }
    pub fn is_match(original: &str, target: &str) -> bool {
        verify(original, target).unwrap()
    }
}
