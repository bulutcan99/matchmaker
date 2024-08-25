use serde_derive::{Deserialize, Serialize};

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
