use std::cmp::{max, min};

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{components::ui::{button::Button, card::{ArticleCard, ArticleInfoCard, FilterBarCard}}, pages::{loading::LoadingPage, notfound::NotFoundPage}, state::use_app};

#[component]
pub fn ArticleList() -> impl IntoView {
    let state = use_app();
    let current_page = state.current_page;
    let items_per_page = state.items_per_page;
    let article_list = state.filtered_results;
    let total_pages = Memo::new(move |_| {
        let articles = state.articles.get();
        articles.len() / items_per_page.get() + (articles.len() % items_per_page.get() > 0) as usize + (articles.len() == 0) as usize
    });
    Effect::new(move |_| {
        // 当 current_page 变化时执行
        state.current_page.get();
        // 滚动到顶部
        web_sys::window()
            .unwrap()
            .scroll_to_with_x_and_y(0.0, 0.0);
    });
    view! {
        <div class="flex flex-col mx-[10%] gap-8">
            <div class="flex flex-row gap-8 w-full">
                <div class="flex flex-col gap-8 w-2/3">
                    <For
                        each=move || {
                            let all = article_list.get();
                            let start = (current_page.get() - 1) * items_per_page.get();
                            let current = all.into_iter().skip(start).take(items_per_page.get()).collect::<Vec<_>>();
                            current
                        }
                        key=|article| article.aid().clone()
                        children=move |article| {
                            view! {
                                <ArticleInfoCard info=article.info().clone() />
                            }
                        }
                    />
                </div>
                <div class="sticky top-24 z-0 self-start w-1/3">
                    <FilterBar/>
                </div>
            </div>
            <PageLeader total_pages=total_pages/>
        </div>
    }
}

#[component]
fn PageLeader(
    total_pages: Memo<usize>,
) -> impl IntoView{
    let state = use_app();

    let page_range = Memo::new(move |_| {
        let curr_page = state.current_page.get();
        let total = total_pages.get();
        let start = if curr_page > 2 { curr_page - 2 } else { 1 };
        let end = if curr_page + 2 <= total { curr_page + 2 } else { total };
        (start..=end).collect::<Vec<_>>()
    });

    view! {
        <div>
            <Show when=move || (total_pages.get() > 1)>
                <div class="flex justify-center gap-4">
                    <Button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded"
                        on_click=Callback::new(move |_| {
                            state.current_page.update(|page| *page = max(*page - 1, 1));
                        })
                    >
                        <p> "Back" </p>
                    </Button>
                    <For 
                        each=move || {page_range.get()}
                        key=|page| { *page }
                        children=move |page| {
                            let is_current = Memo::new({
                                let page = page;
                                move |_| page == state.current_page.get()
                            });
                            
                            let button_class = Memo::new(move |_| {
                                let bg_color = if is_current.get() { "bg-blue-200" } else { "bg-white" };
                                format!("px-4 py-2 mx-1 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-100 focus:outline-none focus:ring focus:ring-gray-300 {}", bg_color)
                            });
                            view! {
                                <Button 
                                    class=button_class
                                    on_click=Callback::new(move |_| {state.current_page.set(page);})
                                >
                                    {page}
                                </Button>
                            }
                            
                        }
                    />
                    <Button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded"
                        on_click=Callback::new(move |_| {
                            state.current_page.update(|page| *page = min(*page + 1, total_pages.get()));
                        })
                    >
                        <p> "Back" </p>
                    </Button>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn ArticleDital() -> impl IntoView { 
    
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let state = use_app();
    let article = state.get_article(id());
    view! {
        <Show when=move || !state.loading.get()
            fallback=move || view! { <LoadingPage /> }
        >
        <Show when=move || article.get().is_some()
            fallback=move || view! { <NotFoundPage/> }
        >
            <ArticleCard article=article/>
        </Show>
        </Show>
    }
}

#[component]
pub fn FilterBar() -> impl IntoView {
    let state = use_app();
    view! {
        <FilterBarCard
            tags=state.tags
            categories=state.categories
            selected_tags=state.selected_tags
            selected_categories=state.selected_categories
        />
    }
}