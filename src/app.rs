use std::collections::HashMap;

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
        blog::article::{
            ArticleDital,
            ArticleList
        }, 
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
    
    view! {
        <Router>
            <div>
                <Backdrop show=ass.active />
                <SearchPanle show=ass.active />
            </div>
            <Header />
            <main class="container mx-auto px-4 py-8">
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/blog") view=ArticleList />
                    <Route path=path!("/blog/:id") view=ArticleDital />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/chat") view=ChatGroupList />
                    <Route path=path!("/user") view=UserProfilePage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
