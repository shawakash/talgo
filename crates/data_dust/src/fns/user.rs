extern crate diesel;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::Rng;
use std::env;
use uuid::Uuid;

use super::DbError;

#[derive(Debug)]
pub enum CryptoError {
    EnvironmentError(std::env::VarError),
    DecodeError(base64::DecodeError),
    EncryptionError,
    DecryptionError,
    Utf8Error(std::string::FromUtf8Error),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EnvironmentError(e) => write!(f, "Environment error: {}", e),
            CryptoError::DecodeError(e) => write!(f, "Decode error: {}", e),
            CryptoError::EncryptionError => write!(f, "Encryption error"),
            CryptoError::DecryptionError => write!(f, "Decryption error"),
            CryptoError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl std::error::Error for CryptoError {}

pub fn get_user_id_by_username_email(
    con: &mut PgConnection,
    username_or_email: &str,
) -> Result<(Uuid, String), DbError> {
    use crate::schema::users::dsl::*;

    users
        .filter(
            username
                .eq(username_or_email)
                .or(email.eq(username_or_email)),
        )
        .select((id, password_hash))
        .first(con)
        .map_err(|e| e.into())
}

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

fn get_encryption_key() -> Result<Vec<u8>, CryptoError> {
    let key = env::var("ENCRYPTION_KEY").map_err(CryptoError::EnvironmentError)?;
    general_purpose::STANDARD_NO_PAD
        .decode(key)
        .map_err(CryptoError::DecodeError)
}

pub fn encrypt_token(token: &str) -> Result<String, CryptoError> {
    let encryption_key = get_encryption_key()?;
    let cipher =
        Aes256Gcm::new_from_slice(&encryption_key).map_err(|_| CryptoError::EncryptionError)?;

    let nonce_array: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_array);

    let ciphertext = cipher
        .encrypt(nonce, token.as_bytes())
        .map_err(|_| CryptoError::EncryptionError)?;

    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD_NO_PAD.encode(combined))
}

pub fn decrypt_token(token: &str) -> Result<String, CryptoError> {
    let encryption_key = get_encryption_key()?;
    let cipher =
        Aes256Gcm::new_from_slice(&encryption_key).map_err(|_| CryptoError::DecryptionError)?;

    let decoded = general_purpose::STANDARD_NO_PAD
        .decode(token)
        .map_err(CryptoError::DecodeError)?;
    if decoded.len() < 12 {
        return Err(CryptoError::DecryptionError);
    }

    let (nonce, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionError)?;
    String::from_utf8(plaintext).map_err(CryptoError::Utf8Error)
}
