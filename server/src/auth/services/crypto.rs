use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn hash_token(token: &str) -> String {
    blake3::hash(token.as_bytes()).to_hex().to_string()
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| err.to_string())
        .map(|hash| hash.to_string())
}

pub fn verify_password(proposed_password: &str, confirmed_password: &str) -> Result<bool, String> {
    let password_hash = PasswordHash::new(confirmed_password).map_err(|err| err.to_string())?;

    Ok(Argon2::default()
        .verify_password(proposed_password.as_bytes(), &password_hash)
        .is_ok())
}
