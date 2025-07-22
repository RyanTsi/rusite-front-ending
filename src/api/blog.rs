
use crate::{api::client::fetch_api, models::blog::{Article, ArticleInfo, Comment}};
use reqwest::Method;

pub async fn get_articles_list() -> Vec<ArticleInfo> {
    fetch_api("/article/list", Method::GET, None).await.unwrap_or_default()
}

pub async fn get_article_content(aid: &str) -> String {
    fetch_api(format!("/article/{aid}/content").as_str(), Method::GET, None).await.unwrap_or_default()
}

pub async fn get_article_comments(aid: &str) -> Vec<Comment> {
    fetch_api(format!("/article/{aid}/comment").as_str(), Method::GET, None).await.unwrap_or_default()
}

pub async fn get_article_details(info: ArticleInfo) -> Article {
    let aid = &info.aid;
    let content = get_article_content(aid).await;
    let comments = get_article_comments(aid).await;
    Article::new(info, &content, comments)
}

pub async fn get_all_articles_details() -> Vec<Article> {
    let infos = get_articles_list().await;
    let mut articles = Vec::new();
    for info in infos {
        let article = get_article_details(info).await;
        articles.push(article);
    }
    articles
}

pub async fn get_article(aid: String) -> Article { 
    let info = get_articles_list().await.into_iter().find(|info| info.aid == aid).unwrap();
    let article = get_article_details(info).await;
    article
}

// TODO: 

// use std::error::Error;
// pub async fn get_articles_list() -> Result<Vec<ArticleInfo>, Box<dyn Error>> {
//     fetch_api("/article/list", Method::GET, None).await
// }

// pub async fn get_article_content(aid: &str) -> Result<String, Box<dyn Error>> {
//     fetch_api(format!("/article/{aid}/content").as_str(), Method::GET, None).await
// }

// pub async fn get_article_comments(aid: &str) -> Result<Vec<Comment>, Box<dyn Error>> {
//     fetch_api(format!("/article/{aid}/comments").as_str(), Method::GET, None).await
// }

// pub async fn get_article_details(info: ArticleInfo) -> Result<Article, Box<dyn Error>> {
//     let aid = &info.aid;
//     let content = get_article_content(aid).await?;
//     let comments = get_article_comments(aid).await?;
//     Ok(Article::new(info, &content, comments))
// }

// pub async fn get_all_articles_details() -> Result<Vec<Article>, Box<dyn Error>> {
//     let infos = get_articles_list().await?;
//     let mut articles = Vec::new();
//     for info in infos {
//         let article = get_article_details(info).await?;
//         articles.push(article);
//     }
//     Ok(articles)
// }