use crate::core::application::usecase::auth::error::TokenError;

use crate::core::domain::valueobject::date::parse_utc;
use crate::shared::config::config::Config;
use crate::shared::data::base64::{b64u_decode_to_string, b64u_encode};
use crate::shared::data::date::{now_utc, now_utc_plus_sec_str};
use hmac::Hmac;
use sha2::digest::Mac;
use sha2::Sha512;
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    pub ident: String,     // Identifier (username for example).
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
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| TokenError::CannotDecodeIdent)?,

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

pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token, TokenError> {
    let config = Config::get();
    _generate_token(
        user,
        &config.auth.jwt.expiration,
        salt,
        (&config.auth.jwt.secret).as_ref(),
    )
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<(), TokenError> {
    let config = Config::get();
    _validate_token_sign_and_exp(origin_token, salt, &config.auth.jwt.secret)?;

    Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation

fn _generate_token(
    ident: &str,
    duration_sec: &u64,
    salt: Uuid,
    key: &str,
) -> Result<Token, TokenError> {
    // -- Compute the two first components.
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // -- Sign the two first components.
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: Uuid,
    key: &str,
) -> Result<(), TokenError> {
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    if new_sign_b64u != origin_token.sign_b64u {
        return Err(TokenError::SignatureNotMatching);
    }

    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| TokenError::ExpNotIso)?;
    let now = now_utc();

    if origin_exp < now {
        return Err(TokenError::Expired);
    }

    Ok(())
}

fn _token_sign_into_b64u(
    ident: &str,
    exp: &str,
    salt: Uuid,
    key: &str,
) -> Result<String, TokenError> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key.as_ref())
        .map_err(|_| TokenError::HmacFailNewFromSlice)?; // `?` operatörünü kullanarak hatayı ele al

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}
// endregion: --- (private) Token Gen and Validation
