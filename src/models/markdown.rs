use serde::{Deserialize, Serialize};

// 标题结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub id: String,
    pub text: String,
    pub level: u32,
}

// Front Matter 结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
}

// Markdown 内容结构
#[derive(Debug, Clone)]
pub struct MarkdownContent {
    pub front_matter: Option<FrontMatter>,
    pub html_content: String,
    pub toc: Vec<Heading>,
}