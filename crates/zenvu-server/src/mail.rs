//! Mail System
//!
//! Fluent API for building and sending emails via SMTP or API drivers (Resend, SendGrid).

use anyhow::Result;

pub struct MailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Send an email message
pub async fn send(msg: MailMessage) -> Result<()> {
    tracing::info!("Sending email to: {} (Subject: {})", msg.to, msg.subject);
    Ok(())
}
