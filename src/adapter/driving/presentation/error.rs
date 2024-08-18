use std::collections::HashMap;

use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub fields: Option<HashMap<String, String>>,
}

impl From<ValidationErrors> for ResponseError {
    fn from(v: ValidationErrors) -> Self {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        v.field_errors().into_iter().for_each(|(k, v)| {
            let msg = format!("invalid {}", v[0].code);

            hash_map.insert(k.into(), msg);
        });
        Self {
            fields: Some(hash_map),
        }
    }
}

// Global api response error struct
#[derive(Serialize)]
pub struct ApiResponseErrorObject {
    pub message: String,
    pub error: Option<Box<dyn ErasedSerialize>>,
}

pub enum ApiResponseError {
    Simple(String),
    Complicated {
        message: String,
        error: Box<dyn ErasedSerialize>,
    },
}

impl ApiResponseError {
    pub fn simple_error(msg: &'static str) -> Self {
        Self::Simple(msg.into())
    }
    pub fn complicated_error(msg: &'static str, error: impl Serialize + 'static) -> Self {
        Self::Complicated {
            message: msg.into(),
            error: Box::new(error),
        }
    }
}

impl From<ApiResponseError> for ApiResponseErrorObject {
    fn from(val: ApiResponseError) -> Self {
        match val {
            ApiResponseError::Simple(message) => Self {
                message,
                error: None,
            },
            ApiResponseError::Complicated { message, error } => Self {
                message,
                error: Some(error),
            },
        }
    }
}
