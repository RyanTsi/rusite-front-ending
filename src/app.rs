use std::sync::Arc;

use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::{
    components::{
        ParentRoute, Route, Router, Routes
    },
    path
};
use crate::{
    api::blog::get_all_articles_details,
    components::{layout::{
        backdrop::Backdrop, footer::Footer, header::Header
    }, ui::panle::SearchPanle},
    pages::{
        about::AboutPage,
        blog::{article::{
            ArticleDital,
            ArticleList
        }, index::BlogIndex}, 
        chat::ChatGroupList, home::HomePage, notfound::NotFoundPage, user::UserProfilePage
    },
    state::{
        provide_app_context,
        provide_search_context,
        use_app,
        use_search
    }
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_search_context();
    provide_app_context();

    let ass = use_app();
    let ass_clone = ass.clone();
    spawn_local(async move {
        ass_clone.load_data().await;
    });
    
    view! {
        <Router>
            <div>
                <Backdrop show=ass.active />
                <SearchPanle show=ass.active />
            </div>
            <Header />
            <div class="flex flex-col bg-gray-100 min-h-screen w-full h-full -z-50 dark:bg-blue-800">
                <main class="px-4 py-8">
                    <Routes fallback=NotFoundPage>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/blog") view=BlogIndex />
                        <Route path=path!("/blog/:id") view=ArticleDital />
                        <Route path=path!("/about") view=AboutPage />
                        <Route path=path!("/chat") view=ChatGroupList />
                        <Route path=path!("/user") view=UserProfilePage />
                    </Routes>
                </main>
            </div>
            <Footer />
        </Router>
    }
}
