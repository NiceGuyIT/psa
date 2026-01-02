//! Knowledge base models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::{TenantId, UserId};

/// Article status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArticleStatus {
    #[default]
    Draft,
    Published,
    Archived,
}

/// Knowledge base article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub content_html: String,
    pub excerpt: Option<String>,
    pub status: ArticleStatus,
    pub category_id: Option<Uuid>,
    pub author_id: UserId,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub view_count: i64,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Article category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Article version (for history)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVersion {
    pub id: Uuid,
    pub article_id: Uuid,
    pub title: String,
    pub content: String,
    pub edited_by: UserId,
    pub created_at: DateTime<Utc>,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub title: String,
    pub excerpt: String,
    pub category: Option<String>,
    pub relevance: f32,
}
