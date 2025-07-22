use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{api::blog::{get_article, get_articles_list}, components::ui::card::{ArticleCard, ArticleInfoCard}, models::blog::ArticleInfo};

#[component]
pub fn ArticleList() -> impl IntoView {
    let async_data = LocalResource::new(move || get_articles_list());
    let article_list = move || async_data.get();
    view! {
        <Show when=move || 
            article_list().is_some()
            fallback=|| view! { <div>Loading...</div> }
        > 
            <div class="flex mx-[10%]">
                <div class="flex flex-col gap-8 w-2/3">
                    <For
                        each=move || article_list().unwrap()
                        key=|article| article.aid.clone()
                        children=move |article| {
                            view! {
                                <ArticleInfoCard info=article />
                            }
                        }
                    />
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn ArticleDital() -> impl IntoView { 
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let async_data = LocalResource::new(move || get_article(id()));
    let article = move || async_data.get();
    view! {
        <Show when=move || article().is_some()
            fallback=move || view! { <div>Loading...</div> }
        >
            // <h1>{move || article().unwrap().title().to_string()}</h1>
            <ArticleCard article=article().unwrap()/>
        </Show>
    }
}