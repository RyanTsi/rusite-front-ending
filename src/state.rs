use crate::{api::blog::{get_all_articles_details, get_categories, get_tags}, models::blog::{Article, Category, Tag}};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct SearchState {
    pub all_articles: RwSignal<HashMap<String, Article>>,
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
                article.title().to_lowercase().contains(&query) ||
                article.content().to_lowercase().contains(&query) ||
                article.tags().iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .take(10) // 最多显示10条结果
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

#[derive(Clone, Debug)]
pub struct FilterBarState {
    pub tags: RwSignal<Vec<Tag>>,
    pub categories: RwSignal<Vec<Category>>,
    pub selected_tags: RwSignal<HashSet<String>>,
    pub selected_categories: RwSignal<HashSet<String>>,
    pub articles: RwSignal<Vec<Article>>,
    pub filtered_results: RwSignal<Vec<Article>>,
}

impl FilterBarState {
    pub fn new() -> Self { 
        Self {
            tags: RwSignal::new(Vec::new()),
            categories: RwSignal::new(Vec::new()),
            selected_tags: RwSignal::new(HashSet::new()),
            selected_categories: RwSignal::new(HashSet::new()),
            articles: RwSignal::new(Vec::new()),
            filtered_results: RwSignal::new(Vec::new()),
        }
    }
    pub fn add_tag(&self, tag: String) {
        self.selected_tags.update(|tags| {
            tags.insert(tag);
        });
        self.update_filtered_results();
    }

    pub fn add_category(&self, category: String) { 
        self.selected_categories.update(|categories| {
            categories.insert(category);
        });
        self.update_filtered_results();
    }
    pub fn remove_tag(&self, tag: String) {
        self.selected_tags.update(|tags| {
            tags.remove(&tag);
        });
        self.update_filtered_results();
    }
    pub fn remove_category(&self, category: String) { 
        self.selected_categories.update(|categories| {
            categories.remove(&category);
        });
        self.update_filtered_results();
    }
    pub fn switch_tag_selected(&self, tag: String) {
        if (self.is_selected_tag(&tag)).get() {
            self.remove_tag(tag);
        } else {
            self.add_tag(tag);
        }
    }
    pub fn switch_category_selected(&self, category: String) { 
        if (self.is_selected_category(&category)).get() {
            self.remove_category(category);
        } else {
            self.add_category(category);
        }
    }
    pub fn is_selected_tag(&self, tag: &str) -> RwSignal<bool> { 
        let is_selectes = self.selected_tags.get().contains(tag);
        RwSignal::new(is_selectes)
    }
    pub fn is_selected_category(&self, category: &str) -> RwSignal<bool> { 
        let is_selectes = self.selected_categories.get().contains(category);
        RwSignal::new(is_selectes)
    }
    pub fn clear_filters(&self) { 
        self.selected_tags.update(|tags| {
            tags.clear();
        });
        self.selected_categories.update(|categories| {
            categories.clear();
        });
    }
    pub fn update_filtered_results(&self) { 
        let selected_tags = self.selected_tags.get();
        let selected_categories = self.selected_categories.get();
        let filtered_results = self.articles.get()
            .into_iter()
            .filter(|article| {
                if selected_tags.is_empty() {
                    true
                } else {
                    article.tags().iter().any(|tag| selected_tags.contains(tag))
                }
            })
            .filter(|article| {
                if selected_categories.is_empty() {
                    true
                } else {
                    article.categories().iter().any(|category| selected_categories.contains(category))
                }
            })
            .collect();
        self.filtered_results.set(filtered_results);
    }
}

#[derive(Clone, Debug)]
pub struct AppState { 
    pub active: RwSignal<bool>,
    pub current_page: RwSignal<usize>,
    pub items_per_page: RwSignal<usize>,
    pub articles: RwSignal<Vec<Article>>,
    pub aid_map: RwSignal<HashMap<String, usize>>,
    pub filter_bar_state: RwSignal<FilterBarState>,
}

impl AppState {
    pub fn new() -> Self { 
        Self {
            active: RwSignal::new(false),
            current_page: RwSignal::new(1),
            items_per_page: RwSignal::new(10),
            articles: RwSignal::new(Vec::new()),
            aid_map: RwSignal::new(HashMap::new()),
            filter_bar_state: RwSignal::new(FilterBarState::new()),
        }
    }
    pub async fn load_data(&self) { 
        let articles = get_all_articles_details().await;
        for idx in 0..articles.len() {
            self.aid_map.update(|map| {
                map.insert(articles[idx].aid().clone(), idx);
            });
        }
        let tags = get_tags().await;
        let categories = get_categories().await;
        self.articles.set(articles.clone());
        self.filter_bar_state.get().tags.set(tags);
        self.filter_bar_state.get().categories.set(categories);
        self.filter_bar_state.get().articles.set(articles.clone());
        self.filter_bar_state.get().update_filtered_results();
    }
    pub fn get_article(&self, aid: String) -> RwSignal<Option<Article>> { 
        let article = self.aid_map.with_untracked(|map| {
            map.get(&aid).and_then(|idx| {
                Some((self.articles.get_untracked())[*idx].clone())
            })
        });
        RwSignal::new(article)
    }
}

pub fn provide_app_context() { 
    provide_context(AppState::new());
}

pub fn use_app() -> AppState { 
    use_context::<AppState>().expect("AppState should be provided")
}