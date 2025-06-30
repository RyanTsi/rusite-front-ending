use leptos::prelude::*;
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
        blog::article::{ArticleDital, ArticleList}, 
        home::HomePage,
        notfound::NotFoundPage
    }
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main class="container mx-auto px-4 py-8">
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/blog") view=ArticleList/>
                    <Route path=path!("/blog/:id") view=ArticleDital/>
                    // <Route path="/about" view=AboutPage/>
                    // <Route path="/admin" view=AdminDashboard/>
                    // <Route path="/admin/login" view=AdminLogin/>
                    // <Route path="/chat" view=ChatList/>
                    // <Route path="/chat/:id" view=ChatGroup/>
                    // <Route path="/user" view=UserProfile/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}