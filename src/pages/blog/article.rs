use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{api::blog::get_articles_list, models::blog::ArticleInfo};

#[component]
pub fn ArticleList() -> impl IntoView {
    let async_data = LocalResource::new(move || get_articles_list());
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };
    view! {
        <h1 class="text-2xl font-bold mb-4">"博客文章"</h1>
        {async_result}
        // // 使用Transition处理加载状态
        // <Transition
        //     fallback=move || view! { 
        //         <div class="text-center py-8">
        //             <p>"加载中..."</p>
        //         </div>
        //     }
        // >
        //     // 处理资源的不同状态
        //     {move || match async_data.get() {
        //         None => view! { <div>"等待数据..."</div> }.into_view(),
        //         Some(list) => view! {}.into_view()
        //     }}
        // </Transition>
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