use argon2::{
    Algorithm, Argon2, Params, Version,
};
use crate::error::Result;

const MEMORY_COST: u32 = 65536;
const TIME_COST: u32 = 3;
const PARALLELISM: u32 = 4;
const KEY_LENGTH: usize = 32;
const SALT_LENGTH: usize = 32;

pub fn generate_salt() -> Vec<u8> {
    use rand::{rngs::OsRng, RngCore};
    let mut salt = vec![0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, Some(KEY_LENGTH))
        .map_err(|e| crate::error::Error::EncryptionError(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; KEY_LENGTH];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| crate::error::Error::EncryptionError(e.to_string()))?;

    Ok(key)
}
