use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{api::blog::get_articles_list, components::ui::card::ArticleInfoCard, models::blog::ArticleInfo};

#[component]
pub fn ArticleList() -> impl IntoView {
    let async_data = LocalResource::new(move || get_articles_list());
    let article_list = move || async_data.get();
    view! {
        <h1 class="text-2xl font-bold mb-4">"博客文章"</h1>
        <Show when=move || 
            article_list().is_some()
            fallback=|| view! { <div>Loading...</div> }
        > 
            <For
                each=move || article_list().unwrap()
                key=|article| article.aid.clone()
                children=move |article| {
                    view! {
                        <ArticleInfoCard info=article />
                    }
                }
            />
        </Show>
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