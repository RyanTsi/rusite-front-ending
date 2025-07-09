use std::error::Error;

use crate::{api::client::fetch_api, models::blog::ArticleInfo};
use reqwest::Method;


pub async fn get_articles_list() -> Vec<ArticleInfo> {
    fetch_api("/article/list", Method::GET, None).await.unwrap_or_default()
}