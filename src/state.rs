use crate::{api::blog::{get_all_articles_details, get_categories, get_tags}, models::blog::{Article, Category, Tag}, pages::blog::article};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct AppState { 
    // article and page view
    pub current_page: RwSignal<usize>,
    pub items_per_page: RwSignal<usize>,
    pub articles: RwSignal<Vec<Article>>,
    pub aid_map: RwSignal<HashMap<String, usize>>,
    
    // filter bar state
    pub filter_bar_state: RwSignal<FilterBarState>,
    pub filtered_results: RwSignal<Vec<Article>>,
    // search bar state
    pub active: RwSignal<bool>,
    pub search_query: RwSignal<String>,
    pub search_results: RwSignal<Vec<Article>>,
}

#[derive(Clone, Debug)]
pub struct FilterBarState {
    pub tags: Vec<Tag>,
    pub categories: Vec<Category>,
    pub selected_tags: HashSet<String>,
    pub selected_categories: HashSet<String>,
}

impl FilterBarState {
    pub fn new() -> Self { 
        Self {
            tags: Vec::new(),
            categories: Vec::new(),
            selected_tags: HashSet::new(),
            selected_categories: HashSet::new(),
        }
    }
    pub fn remove_tag(&mut self, tag: String) { 
        self.selected_tags.remove(&tag);
    }
    pub fn add_tag(&mut self, tag: String) { 
        self.selected_tags.insert(tag);
    }
}

impl AppState {
    pub fn new() -> Self { 
        Self {
            current_page: RwSignal::new(1),
            items_per_page: RwSignal::new(10),
            articles: RwSignal::new(Vec::new()),
            aid_map: RwSignal::new(HashMap::new()),
            filter_bar_state: RwSignal::new(FilterBarState::new()),
            filtered_results: RwSignal::new(Vec::new()),
            active: RwSignal::new(false),
            search_query: RwSignal::new(String::new()),
            search_results: RwSignal::new(Vec::new()),
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
        self.articles.set(articles);
        self.filter_bar_state.update(|state| state.tags = tags);
        self.filter_bar_state.update( |state| state.categories = categories);
        self.update_filtered_results();
    }
    pub fn get_article(&self, aid: String) -> RwSignal<Option<Article>> { 
        let article = self.aid_map.with_untracked(|map| {
            map.get(&aid).and_then(|idx| {
                Some((self.articles.get_untracked())[*idx].clone())
            })
        });
        RwSignal::new(article)
    }
    pub fn add_tag(&self, tag: String) {
        self.filter_bar_state.update(|state| { state.selected_tags.insert(tag); } );
    }

    // pub fn add_category(&self, category: String) { 
    //     self.selected_categories.update(|categories| {
    //         categories.insert(category);
    //     });
    // }
    pub fn remove_tag(&self, tag: String) {
        self.filter_bar_state.update(|state| { state.selected_tags.remove(&tag); } );
    }
    // pub fn remove_category(&self, category: String) { 
    //     self.selected_categories.update(|categories| {
    //         categories.remove(&category);
    //     });
    // }
    pub fn switch_tag_selected(&self, tag: String) {
        if (self.is_selected_tag(&tag)).get() {
            self.remove_tag(tag);
        } else {
            self.add_tag(tag);
        }
    }
    // pub fn switch_category_selected(&self, category: String) { 
    //     if (self.is_selected_category(&category)).get() {
    //         self.remove_category(category);
    //     } else {
    //         self.add_category(category);
    //     }
    // }
    pub fn is_selected_tag(&self, tag: &str) -> RwSignal<bool> { 
        let is_selectes = self.filter_bar_state.get().selected_tags.contains(tag);
        RwSignal::new(is_selectes)
    }
    // pub fn is_selected_category(&self, category: &str) -> RwSignal<bool> { 
    //     let is_selectes = self.selected_categories.get().contains(category);
    //     RwSignal::new(is_selectes)
    // }
    pub fn update_filtered_results(&self) { 
        let selected_tags = self.filter_bar_state.get().selected_tags;
        let selected_categories = self.filter_bar_state.get().selected_categories;
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
    // pub fn clear_filters(state: &AppState) { 
    //     state.selected_tags.update(|tags| {
    //         tags.clear();
    //     });
    //     state.selected_categories.update(|categories| {
    //         categories.clear();
    //     });
    // }
}

pub fn provide_app_context() { 
    provide_context(AppState::new());
}

pub fn use_app() -> AppState { 
    use_context::<AppState>().expect("AppState should be provided")
}

#[derive(Clone, Debug)]
pub struct SearchState {
    pub search_query: RwSignal<String>,
    pub search_results: RwSignal<Vec<Article>>,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            search_query: RwSignal::new(String::new()),
            search_results: RwSignal::new(Vec::new()),
        }
    }
    
    // 更新搜索结果
    pub fn update_search_results(&self, articles: &RwSignal<Vec<Article>>) {
        let query = self.search_query.get().to_lowercase();
        
        if query.is_empty() {
            self.search_results.set(Vec::new());
            return;
        }
        
        let results: Vec<Article> = articles.get()
            .into_iter()
            .filter(|article| {
                article.title().to_lowercase().contains(&query) ||
                article.content().to_lowercase().contains(&query) ||
                article.tags().iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .take(10) // 最多显示10条结果
            .collect();
        
        self.search_results.set(results);
    }
}