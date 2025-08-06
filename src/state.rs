use crate::{api::blog::{get_all_articles_details, get_categories, get_tags}, models::blog::{Article, Category, Tag}};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct AppState { 
    // common
    pub loading: RwSignal<bool>,
    pub active: RwSignal<bool>,
    pub current_page: RwSignal<usize>,
    pub items_per_page: RwSignal<usize>,
    pub articles: RwSignal<Vec<Article>>,
    pub aid_map: RwSignal<HashMap<String, usize>>,
    // filter bar state
    pub tags: RwSignal<Vec<Tag>>,
    pub categories: RwSignal<Vec<Category>>,
    pub selected_tags: RwSignal<HashSet<String>>,
    pub selected_categories: RwSignal<HashSet<String>>,
    pub filtered_results: Memo<Vec<Article>>,
    // search bar state
    pub search_query: RwSignal<String>,
    pub search_results: Memo<Vec<Article>>,
}

impl AppState {
    pub fn new() -> Self {
        let articles = RwSignal::new(Vec::new());
        let selected_tags = RwSignal::new(HashSet::new());
        let selected_categories = RwSignal::new(HashSet::new());
        let search_query = RwSignal::new(String::new());
        let filtered_results = create_filtered_results(articles, selected_tags, selected_categories);
        let search_results = create_search_results(articles, search_query);
        Self {
            loading: RwSignal::new(true),
            active: RwSignal::new(false),
            current_page: RwSignal::new(1),
            items_per_page: RwSignal::new(10),
            articles,
            aid_map: RwSignal::new(HashMap::new()),
            //
            tags: RwSignal::new(Vec::new()),
            categories: RwSignal::new(Vec::new()),
            selected_tags,
            selected_categories,
            filtered_results,
            // 
            search_query,
            search_results,
        }
    }
    pub async fn load_data(&self) { 
        let articles = get_all_articles_details().await;
        self.aid_map.update(|map| map.clear());
        for idx in 0..articles.len() {
            self.aid_map.update(|map| {
                map.insert(articles[idx].aid().clone(), idx);
            });
        }
        let tags = get_tags().await;
        let categories = get_categories().await;
        self.articles.set(articles.clone());
        self.tags.set(tags);
        self.categories.set(categories);
        self.loading.update(|v| *v = false);
    }
    pub fn get_article(&self, aid: String) -> Memo<Option<Article>> {
        let aid_map = self.aid_map.clone();
        let articles = self.articles.clone();
        Memo::new(move |_| {
            aid_map.with(|map| map.get(&aid).copied())
                .and_then(|idx| articles.with(|articles| articles.get(idx).cloned()))
        })
    }

    pub fn add_tag(&self, tag: String) {
        add_tag(self.selected_tags, tag);
    }

    pub fn add_category(&self, category: String) { 
        add_category(self.selected_categories, category);
    }
    pub fn remove_tag(&self, tag: String) {
        remove_tag(self.selected_tags, tag);
    }
    pub fn remove_category(&self, category: String) { 
        remove_category(self.selected_categories, category);
    }
    pub fn switch_tag_selected(&self, tag: String) {
        switch_tag_selected(self.selected_tags, tag);
    }
    pub fn switch_category_selected(&self, category: String) { 
        switch_category_selected(self.selected_categories, category);
    }
    pub fn is_selected_tag(&self, tag: &str) -> impl Fn() -> bool {
        is_selected_tag(self.selected_tags, tag)
    }
    pub fn is_selected_category(&self, category: &str) -> impl Fn() -> bool { 
        is_selected_category(self.selected_categories, category)
    }
    pub fn clear_filters(&self) { 
        clear_filters(self.selected_tags, self.selected_categories);
    }

}

fn create_filtered_results(
    articles: RwSignal<Vec<Article>>, 
    selected_tags: RwSignal<HashSet<String>>,
    selected_categories: RwSignal<HashSet<String>>,
) -> Memo<Vec<Article>> {
    Memo::new( move |_| {
        articles.with(|articles| {
            articles
                .iter()
                .filter(|article| {
                let tags_empty = selected_tags.with(|t| t.is_empty());
                tags_empty || article.tags().iter().any(|t| selected_tags.with(|st| st.contains(t)))
            })
            .filter(|article| {
                let categories_empty = selected_categories.with(|c| c.is_empty());
                categories_empty || article.categories().iter().any(|c| selected_categories.with(|sc| sc.contains(c)))
            })
            .cloned()
            .collect()
        })}
    )
}

fn create_search_results(
    articles: RwSignal<Vec<Article>>,
    search_query: RwSignal<String>, 
) -> Memo<Vec<Article>> {
    Memo::new( move |_| {
        let query = search_query.get().trim().to_lowercase();
        
        if query.is_empty() {
            return Vec::new();
        }
        articles.with(|articles| {
            articles
                .iter()
                .filter(|article| {
                    article.title().to_lowercase().contains(&query) ||
                    article.content().to_lowercase().contains(&query) ||
                    article.tags().iter().any(|tag| tag.to_lowercase().contains(&query)) ||
                    article.categories().iter().any(|category| category.to_lowercase().contains(&query))
                })
                .take(10) // 最多显示10条结果
                .cloned()
                .collect()
        })
    })
}

pub fn add_tag(selected_tags: RwSignal<HashSet<String>>, tag: String) {
    selected_tags.update(|tags| {
        tags.insert(tag);
    });
}

pub fn add_category(selected_categories: RwSignal<HashSet<String>>, category: String) { 
    selected_categories.update(|categories| {
        categories.insert(category);
    });
}
pub fn remove_tag(selected_tags: RwSignal<HashSet<String>>, tag: String) {
    selected_tags.update(|tags| {
        tags.remove(&tag);
    });
}
pub fn remove_category(selected_categories: RwSignal<HashSet<String>>, category: String) { 
    selected_categories.update(|categories| {
        categories.remove(&category);
    });
}
pub fn switch_tag_selected(selected_tags: RwSignal<HashSet<String>>, tag: String) {
    if is_selected_tag(selected_tags, &tag)() {
        remove_tag(selected_tags, tag);
    } else {
        add_tag(selected_tags, tag);
    }
}
pub fn switch_category_selected(selected_categories: RwSignal<HashSet<String>>, category: String) { 
    if is_selected_category(selected_categories, &category)() {
        remove_category(selected_categories, category);
    } else {
        add_category(selected_categories, category);
    }
}
pub fn is_selected_tag(selected_tags: RwSignal<HashSet<String>>, tag: &str) -> impl Fn() -> bool {
    let tag = tag.to_string();
    move || selected_tags.with(|tags| tags.contains(&tag))
}
pub fn is_selected_category(selected_categories: RwSignal<HashSet<String>>, category: &str) -> impl Fn() -> bool { 
    let category = category.to_string();
    move || selected_categories.with(|categories| categories.contains(&category))
}
pub fn clear_filters(selected_tags: RwSignal<HashSet<String>>, selected_categories: RwSignal<HashSet<String>>) { 
    selected_tags.update(|tags| {
        tags.clear();
    });
    selected_categories.update(|categories| {
        categories.clear();
    });
}

pub fn provide_app_context() { 
    provide_context(AppState::new());
}

pub fn use_app() -> AppState { 
    use_context::<AppState>().expect("AppState should be provided")
}