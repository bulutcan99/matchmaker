use crate::shared::error::Result;
use tera::{Context, Tera};

pub mod base64;
pub mod date;

pub fn render_string(tera_template: &str, locals: &serde_json::Value) -> Result<String> {
    let text = Tera::one_off(tera_template, &Context::from_serialize(locals)?, false)?;
    Ok(text)
}
