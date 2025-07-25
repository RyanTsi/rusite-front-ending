use std::cmp::{max, min};

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{api::blog::{get_article, get_articles_list}, components::ui::card::{ArticleCard, ArticleInfoCard}, models::blog::ArticleInfo, state::use_app};

#[component]
pub fn ArticleList() -> impl IntoView {
    let state = use_app();
    let async_data = LocalResource::new(move || get_articles_list());
    let article_list = move || async_data.get();
    let current_page = move || state.current_page.get();
    let items_per_page = move || state.items_per_page.get();
    let total_pages = move || {
        let articles = article_list().unwrap();
        articles.len() / items_per_page() + (articles.len() % items_per_page() > 0) as usize + (articles.len() == 0) as usize
    };
    view! {
        <Show when=move || 
            article_list().is_some()
            fallback=|| view! { <div>Loading...</div> }
        > 
            <div class="flex flex-col mx-[10%]">
                <div class="flex flex-col gap-8 w-2/3">
                    <For
                        each=move || {
                            let all = article_list().unwrap();
                            let start = (current_page() - 1) * items_per_page();
                            let current = all.into_iter().skip(start).take(items_per_page()).collect::<Vec<_>>();
                            current
                        }
                        key=|article| article.aid.clone()
                        children=move |article| {
                            view! {
                                <ArticleInfoCard info=article />
                            }
                        }
                    />
                </div>
                <div>
                    <Show when=move || (article_list().unwrap().len() > items_per_page())>
                        <div class="flex justify-center gap-4">
                            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded" on:click=move |_| {
                                    state.current_page.update(|page| *page = max(*page - 1, 1));
                            }>
                                <p>"Back"</p>
                            </button>
                            <For 
                                each=move || {
                                    let start = if current_page() > 2 { 
                                        current_page() - 2 
                                    } else { 
                                        1 
                                    };
                                    let end = if current_page() + 2 <= total_pages() { 
                                        current_page() + 2 
                                    } else { 
                                        total_pages() 
                                    };
                                    start..=end
                                }
                                key=|page| { page.clone() }
                                children=move |page| {
                                    let is_current = page == current_page();
                                    let bg_color = if is_current { 
                                        "bg-blue-200" 
                                    } else { 
                                        "bg-white" 
                                    };
                                    
                                    view! {
                                        <button 
                                            class={"px-4 py-2 mx-1 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-100 focus:outline-none focus:ring focus:ring-gray-300 ".to_string() + bg_color}
                                            on:click=move |_| {
                                                state.current_page.set(page);
                                            }>
                                            {page}
                                        </button>
                                    }
                                    
                                }
                            />
                            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded" on:click=move |_| {
                                    state.current_page.update(|page| *page = min(*page + 1, total_pages()));
                            }>
                                <p>"Next"</p>
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn PageLeader() -> impl IntoView{

    view! {

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