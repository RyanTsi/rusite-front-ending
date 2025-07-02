use leptos::prelude::*;

use crate::components::ui::Link;

#[component]
pub fn Header() -> impl IntoView { 
    view! {
        <header>
            <nav class="sticky top-0 flex items-center justify-between p-4 bg-white/80 backdrop-blur-sm shadow-sm z-30">
                <div id="navleft" class="flex items-center space-x-12">
                    <Favicon/>
                    <RouterLinks/>
                </div>
                <div id="navright" class="flex items-center space-x-12"> 
                    <b>"Search"</b>
                    <b>"User"</b>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Favicon() -> impl IntoView { 
    view! {
        <div>
            <img src="images/favicon.ico" alt="logo" class="h-12 w-12"/>
        </div>
    }
}

#[component]
fn RouterLinks() -> impl IntoView { 
    view! {
        <div id="links" class="space-x-8">
            <Link href="/">"Home"</Link>
            <Link href="/blog">"Blog"</Link>
            <Link href="/about">"About"</Link>
        </div>
    }
}