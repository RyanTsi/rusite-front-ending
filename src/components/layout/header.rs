use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView { 
    view! {
        <header>
            <nav class="sticky top-0 flex items-center justify-between p-4 bg-white/80 backdrop-blur-sm shadow-sm z-30">
                <div id="navleft" class="flex items-center space-x-12">
                    <a href="/">"Home"</a>
                    <a href="/blog">"Blog"</a>
                    <a href="/about">"About"</a>
                </div>
                <div id="navright" class="flex items-center space-x-12"> 
                    <b> "Search"</b>
                    <b>"User"</b>
                </div>
            </nav>
        </header>
    }
}