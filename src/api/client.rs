use crate::{
    config::API_BASE_URL,
    models::response::{ApiResponse, ErrorResponse}
};
use std::error::Error;
use reqwest::Client;


pub async fn fetch_api<T: serde::de::DeserializeOwned>(
    path: &str,
    method: reqwest::Method,
    body: Option<serde_json::Value>,
) -> Result<T, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}{}", API_BASE_URL, path);
    
    let mut request = client.request(method, &url);

    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        let api_response: ApiResponse<T> = response.json().await?;
        Ok(api_response.data)
    } else {
        let error_response: ErrorResponse = response.json().await?;
        Err(error_response.message.into())
    }
}