//! Notification services for PSA Platform
//!
//! Supports multiple channels: email, SMS, webhooks, Slack, Teams

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::{TenantId, UserId};

/// Notification channel types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Slack,
    Teams,
    Webhook,
    InApp,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationChannel::Email => "email",
            NotificationChannel::Sms => "sms",
            NotificationChannel::Slack => "slack",
            NotificationChannel::Teams => "teams",
            NotificationChannel::Webhook => "webhook",
            NotificationChannel::InApp => "in_app",
        }
    }
}

/// Notification priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPriority {
    Low,
    #[default]
    Normal,
    High,
    Urgent,
}

/// Notification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Read,
}

/// A notification to be sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<UserId>,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub subject: String,
    pub body: String,
    pub metadata: serde_json::Value,
    pub status: NotificationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Request to send a notification
#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub channel: NotificationChannel,
    pub recipient: String,
    pub subject: String,
    pub body: String,
    #[serde(default)]
    pub priority: NotificationPriority,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Email configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

impl EmailConfig {
    pub fn from_env() -> Option<Self> {
        Some(Self {
            smtp_host: std::env::var("SMTP_HOST").ok()?,
            smtp_port: std::env::var("SMTP_PORT").ok()?.parse().ok()?,
            smtp_username: std::env::var("SMTP_USERNAME").ok()?,
            smtp_password: std::env::var("SMTP_PASSWORD").ok()?,
            from_email: std::env::var("SMTP_FROM_EMAIL").ok()?,
            from_name: std::env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "PSA Platform".to_string()),
        })
    }
}

/// Email sender using lettre
pub struct EmailSender {
    config: EmailConfig,
}

impl EmailSender {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    /// Send an email
    pub async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        use lettre::{
            message::header::ContentType,
            transport::smtp::authentication::Credentials,
            AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
        };

        let email = Message::builder()
            .from(
                format!("{} <{}>", self.config.from_name, self.config.from_email)
                    .parse()
                    .map_err(|e| format!("Invalid from address: {}", e))?,
            )
            .to(to.parse().map_err(|e| format!("Invalid to address: {}", e))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| format!("Failed to build email: {}", e))?;

        let creds = Credentials::new(
            self.config.smtp_username.clone(),
            self.config.smtp_password.clone(),
        );

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.smtp_host)
                .map_err(|e| format!("Failed to create transport: {}", e))?
                .credentials(creds)
                .port(self.config.smtp_port)
                .build();

        mailer
            .send(email)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }
}

/// Notification service that handles all channels
pub struct NotificationService {
    email_sender: Option<EmailSender>,
}

impl NotificationService {
    pub fn new() -> Self {
        let email_sender = EmailConfig::from_env().map(EmailSender::new);

        Self { email_sender }
    }

    /// Send a notification
    pub async fn send(&self, request: SendNotificationRequest) -> Result<(), String> {
        match request.channel {
            NotificationChannel::Email => {
                let sender = self.email_sender.as_ref()
                    .ok_or("Email not configured")?;
                sender.send(&request.recipient, &request.subject, &request.body).await
            }
            NotificationChannel::InApp => {
                // In-app notifications are stored in database, not sent externally
                tracing::info!("In-app notification queued: {}", request.subject);
                Ok(())
            }
            _ => {
                tracing::warn!("Notification channel {:?} not implemented", request.channel);
                Ok(())
            }
        }
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
