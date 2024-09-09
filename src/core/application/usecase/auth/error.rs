use std::fmt;

use serde_derive::{Deserialize, Serialize};
pub type Result<T> = core::result::Result<T, TokenError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TokenError {
    HmacFailNewFromSlice,
    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeIat,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RegisterError<T> {
    BadClientData(T),
    UserAlreadyRegistered,
    DbInternalError,
    HashingError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginError {
    UserNotFound,
    BadCredentials,
    UserProviderNotValid,
    DbInternalError,
    JWTEncodingError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeError {
    InvalidJwtToken,
    InvalidIdFormat,
    DbInternalError,
    UserNotFound,
}

impl fmt::Display for MeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeError::InvalidJwtToken => write!(f, "Invalid JWT token"),
            MeError::InvalidIdFormat => write!(f, "Invalid ID format"),
            MeError::DbInternalError => write!(f, "Database internal error"),
            MeError::UserNotFound => write!(f, "User not found"),
        }
    }
}
