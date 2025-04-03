use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

pub fn encrypt_password(password: &str) -> Result<String, BcryptError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, &hash)
}
