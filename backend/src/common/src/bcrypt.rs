extern crate bcrypt;
use bcrypt::{hash, verify, BcryptError, BcryptResult, DEFAULT_COST};

#[derive(Debug)]
pub struct BcryptHasher;

impl BcryptHasher {
    pub fn hash_string<S: Into<String>>(data_string: S) -> Result<String, BcryptError> {
        hash(data_string.into(), DEFAULT_COST)
    }

    pub fn verify_hash<S: Into<String>>(data_string: S, hash: S) -> BcryptResult<bool> {
        verify(data_string.into(), &hash.into())
    }
}
