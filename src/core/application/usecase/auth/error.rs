use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
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

// region:    --- Error Boilerplate
impl core::fmt::Display for TokenError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for TokenError {}
// endregion: --- Error Boilerplate

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
