use leptos::prelude::*;

use crate::pages::blog::article::ArticleList;

#[component]
pub fn BlogIndex() -> impl IntoView {
    view! {
        <ArticleList/>

    }
}