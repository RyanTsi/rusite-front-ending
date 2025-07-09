use crate::models::blog::Article;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SearchState {
    pub all_articles: RwSignal<HashMap<String, Article>>, // 文章ID -> 文章
    pub search_query: RwSignal<String>,
    pub search_results: RwSignal<Vec<Article>>,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            all_articles: RwSignal::new(HashMap::new()),
            search_query: RwSignal::new("".to_string()),
            search_results: RwSignal::new(Vec::new()),
        }
    }
    
    // 更新搜索结果
    pub fn update_search_results(&self) {
        let query = self.search_query.get().to_lowercase();
        let articles = self.all_articles.get();
        
        if query.is_empty() {
            self.search_results.set(Vec::new());
            return;
        }
        
        let results: Vec<Article> = articles.values()
            .filter(|article| {
                article.title.to_lowercase().contains(&query) ||
                article.content.to_lowercase().contains(&query) ||
                article.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .take(5) // 最多显示5条结果
            .cloned()
            .collect();
        
        self.search_results.set(results);
    }
}

pub fn provide_search_context() {
    provide_context(SearchState::new());
}

pub fn use_search() -> SearchState {
    use_context::<SearchState>().expect("SearchState should be provided")
}