use std::sync::Arc;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Article {
    info: Arc<ArticleInfo>,
    content: String,
    comments: Arc<Vec<Comment>>
}
impl Article {
    pub fn new(info: ArticleInfo, content: &str, comments: Vec<Comment>) -> Self {
        Article {
            info: Arc::new(info),
            content: String::from(content),
            comments: Arc::new(comments),
        }
    }
    pub fn aid(&self) -> String {
        self.info.aid.clone()
    }
    pub fn title(&self) -> &str {
        &self.info.title
    }
    pub fn tags(&self) -> &[String] {
        &self.info.tags
    }
    pub fn categories(&self) -> &[String] {
        &self.info.categories
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }
    pub fn created_at(&self) -> String {
        format_date_cn(self.info.created_at)
    }
    pub fn updated_at(&self) -> String {
        format_date_cn(self.info.updated_at)
    }
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

#[derive(Debug, Clone)]
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
    pub uid: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub count: i32,
}