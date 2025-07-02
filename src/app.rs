use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{
        ParentRoute, Route, Router, Routes
    },
    path
};
use crate::{
    components::layout::{
        footer::Footer,
        header::Header
    },
    pages::{
        about::AboutPage, blog::article::{
            ArticleDital,
            ArticleList
        }, chat::ChatGroupList, home::HomePage, notfound::NotFoundPage, user::UserProfilePage
    }
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Link/>
            <Header/>
            <main class="container mx-auto px-4 py-8">
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/blog") view=ArticleList/>
                    <Route path=path!("/blog/:id") view=ArticleDital/>
                    <Route path=path!("/about") view=AboutPage/>
                    <Route path=path!("/chat") view=ChatGroupList/>
                    <Route path=path!("/user") view=UserProfilePage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}