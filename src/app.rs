use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::{
    components::{
        Route, Router, Routes
    },
    path
};
use crate::{
    components::{layout::{
        backdrop::Backdrop, footer::Footer, header::Header
    }, ui::panle::SearchPanle},
    pages::{
        about::AboutPage,
        blog::{article::{
            ArticleDital,
        }, index::BlogIndex}, 
        chat::ChatGroupList, home::index::HomePage, notfound::NotFoundPage, user::UserProfilePage
    },
    state::{
        provide_app_context,
        use_app,
    }
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_app_context();

    let state = use_app();
    let state_clone = state.clone();
    spawn_local(async move {
        state_clone.load_data().await;
    });
    
    view! {
        <Router>
            <div>
                <Backdrop show=state.active />
                <SearchPanle show=state.active />
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
