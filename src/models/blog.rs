use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub aid: String,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleInfo {
    pub aid: String,
    pub title: String,
    pub summary: String,
    pub secret: Option<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleCreateRequest {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub summary: String,
    pub content: String,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleModifyRequest {
    pub aid: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub cid: String,
    pub aid: String,
    pub uid: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreateRequest {
    pub aid: String,
    pub uid: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentModifyRequest {
    pub cid: String,
    pub content: String,
}