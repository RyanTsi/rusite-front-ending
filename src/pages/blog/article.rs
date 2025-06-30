use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ArticleList() -> impl IntoView { 
    view! {
        <h1>"Blog"</h1>
    }
}

#[component]
pub fn ArticleDital() -> impl IntoView { 
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    // let id = id().as_str();
    view! {
        <h1>{id()}</h1>
        <h1>"Nihao"</h1>
    }
}