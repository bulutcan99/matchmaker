//todo: add appcontext and mailer from loco!

use crate::shared::mailer::mailer::MailerError::InvalidEmailFormat;
use dotenv::dotenv;
use futures_util::TryFutureExt;
use lettre::message::{header, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::error;
use std::error::Error;
use std::{env, fmt};

#[derive(Debug)]
pub enum MailerError {
    InvalidToken,
    SmtpError(String),
    EmailSendError(String),
    InvalidEmailFormat,
    MessageBuildError,
}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MailerError::InvalidToken => write!(f, "Invalid token"),
            MailerError::EmailSendError(ref err) => write!(f, "Failed to send email: {}", err),
            MailerError::InvalidEmailFormat => write!(f, "Invalid email format"),
            MailerError::SmtpError(ref err) => write!(f, "SMTP error: {}", err),
            MailerError::MessageBuildError => write!(f, "Failed to build email message"),
        }
    }
}

impl Error for MailerError {}

pub async fn send_verification_email(user_email: &str, token: &str) -> Result<(), MailerError> {
    let email_body = format!(
        "Hello, \n\nHere is your verification token:\n\n{}\n\nPlease use this token to verify your account.",
        token
    );

    send_email(user_email, &email_body).await?;

    Ok(())
}

async fn send_email(to_email: &str, body: &str) -> Result<(), MailerError> {
    dotenv().ok();

    let from_email = env::var("FROM_EMAIL").map_err(|_| MailerError::InvalidEmailFormat)?;
    let password = env::var("EMAIL_PASSWORD")
        .map_err(|_| MailerError::SmtpError("Password not found".to_string()))?;
    let smtp_server = env::var("SMTP_SERVER")
        .map_err(|_| MailerError::SmtpError("SMTP Server not found".to_string()))?;

    println!(
        "FROM EMAIL: {}, PASSWORD: {}",
        from_email.clone(),
        password.clone()
    );
    let from_email_parsed = from_email
        .clone()
        .parse::<Mailbox>()
        .map_err(|_| MailerError::InvalidEmailFormat)?;
    let to_email_parsed = to_email
        .clone()
        .parse::<Mailbox>()
        .map_err(|_| MailerError::InvalidEmailFormat)?;

    let email = Message::builder()
        .from(from_email_parsed.clone())
        .to(to_email_parsed)
        .subject("VERIFICATION")
        .header(header::ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|_| MailerError::MessageBuildError)?;

    let mut parts = from_email.split('@');
    let username = match parts.next() {
        Some(part) => part,
        None => {
            error!("Invalid email address format");
            return Err(InvalidEmailFormat);
        }
    };
    let creds = Credentials::new(username.to_owned(), password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .map_err(|e| MailerError::SmtpError(e.to_string()))?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map_err(|e| MailerError::SmtpError(e.to_string()))?;

    Ok(())
}
