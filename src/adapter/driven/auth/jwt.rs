use anyhow::{anyhow, Error};
use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::config::Settings;
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::payload::Payload;
use crate::core::port::auth::TokenMaker;

#[derive(Debug, Clone)]
pub struct JwtTokenHandler {
    secret: String,
}

impl JwtTokenHandler {
    pub fn new() -> Self {
        let config = Settings::get();
        let key = config
            .password
            .secret_jwt
            .as_deref()
            .unwrap_or_default()
            .to_owned();
        Self { secret: key }
    }

    pub fn get_expire_time(&self) -> u64 {
        let expire = 1000 * 60 * 60 * 5;
        let current_time = Timestamp::now_utc();
        let expire_time = current_time + expire;
        expire_time.datetime.timestamp() as u64
    }
}

#[async_trait]
impl TokenMaker for JwtTokenHandler {
    async fn generate_token(&self, user: &User) -> String {
        let payload = Payload {
            sub: user.id.unwrap_or_default().to_string(),
            iat: Timestamp::now_utc().to_unix_timestamp(),
            exp: self.get_expire_time(),
        };

        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .unwrap();

        token
    }

    fn decode_token(&self, token: &str) -> Result<Payload, Error> {
        let token = token.trim_start_matches("Bearer ");
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let decoded_token = decode::<Payload>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        );

        match decoded_token {
            Ok(value) => Ok(value.claims),
            Err(e) => Err(anyhow!("Error while decoding token: {:?}", e).into()),
        }
    }
}
