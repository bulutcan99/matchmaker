// region:    --- Modules

use std::fmt::Display;
use std::str::FromStr;

use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use uuid::Uuid;

use crate::config::Settings;
use crate::core::application::usecase::auth::error::TokenError;
use crate::core::domain::valueobject::date::{parse_utc, Timestamp};
use crate::core::port::auth::TokenMaker;
use crate::shared::base64::{b64u_decode_to_string, b64u_encode};

// endregion: --- Modules

// region:    --- Token Type

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    pub ident: String,     // Identifier (email for example).
    pub iat: String,       // Issued at date in Rfc3339.
    pub exp: String,       // Expiration date in Rfc3339.
    pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(TokenError::InvalidFormat);
        }
        let (ident_b64u, iat_b64u, exp_b64u, sign_b64u) =
            (splits[0], splits[1], splits[2], splits[3]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| TokenError::CannotDecodeIdent)?,
            iat: b64u_decode_to_string(iat_b64u).map_err(|_| TokenError::CannotDecodeIat)?,
            exp: b64u_decode_to_string(exp_b64u).map_err(|_| TokenError::CannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

// endregion: --- Token Type
// region:    --- Web Token Gen and Validation
impl TokenMaker for Token {
    fn generate_token(user: &str, salt: &Uuid) -> Result<Token, TokenError> {
        let config = Settings::get();
        let key = config.password.secret_jwt.as_deref().unwrap_or_default();
        _generate_token(user, salt, key)
    }

    fn validate_token(token: &Token, salt: &Uuid) -> Result<(), TokenError> {
        let config = Settings::get();
        let key = config.password.secret_jwt.as_deref().unwrap_or_default();
        _validate_token_sign_and_exp(token, salt, key)?;
        Ok(())
    }
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation

fn _get_expire_time() -> String {
    let expire = Duration::hours(5);
    let current_time = Utc::now();
    let expire_time = current_time + expire;

    expire_time.to_rfc3339()
}

fn _get_iat() -> String {
    let current_time = Utc::now();
    current_time.to_rfc3339()
}

fn _generate_token(ident: &str, salt: &Uuid, key: &str) -> Result<Token, TokenError> {
    let ident = ident.to_string();
    let exp = _get_expire_time();
    let iat = _get_iat();
    // -- Sign the three first components.
    let sign_b64u = _token_sign_into_b64u(&ident, &iat, &exp, salt, key)?;

    Ok(Token {
        ident,
        iat,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: &Uuid,
    key: &str,
) -> Result<(), TokenError> {
    // -- Validate signature.
    let new_sign_b64u = _token_sign_into_b64u(
        &origin_token.ident,
        &origin_token.iat,
        &origin_token.exp,
        salt,
        key,
    )?;

    if new_sign_b64u != origin_token.sign_b64u {
        return Err(TokenError::SignatureNotMatching);
    }

    // -- Validate expiration.
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| TokenError::ExpNotIso)?;
    let now = Timestamp::now_utc().convert_to_offset();

    if origin_exp < now {
        return Err(TokenError::Expired);
    }

    Ok(())
}

/// Create token signature from token parts
/// and salt.
fn _token_sign_into_b64u(
    ident: &str,
    iat: &str,
    exp: &str,
    salt: &Uuid,
    key: &str,
) -> Result<String, TokenError> {
    let content = format!(
        "{}.{}.{}",
        b64u_encode(ident),
        b64u_encode(iat),
        b64u_encode(exp)
    );

    let key = key.as_bytes();
    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| TokenError::HmacFailNewFromSlice)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}
