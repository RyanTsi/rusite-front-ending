use leptos::prelude::*;

use crate::{components::ui::button::Link, state::use_app};
use leptos_icons::Icon;
use icondata as i;

#[component]
pub fn Header() -> impl IntoView { 
    view! {
        <header class="sticky top-0 z-40">
            <nav class="flex items-center justify-between p-4 bg-white/80 backdrop-blur-sm shadow-sm z-30">
                <div id="navleft" class="flex items-center space-x-12">
                    <Favicon/>
                    <RouterLinks/>
                </div>
                <div id="navright" class="flex items-center space-x-12"> 
                    <SearchBox/>
                    <UserBox/>
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
            <Link href="/".to_string()>"Home"</Link>
            <Link href="/blog".to_string()>"Blog"</Link>
            <Link href="/about".to_string()>"About"</Link>
        </div>
    }
}

#[component]
fn SearchBox() -> impl IntoView {
    let ass = use_app();
    view! {
        <button
            on:click=move |_| {
                ass.active.set(true);
            }
        >
            <Icon icon={i::ChSearch} />
        </button>

    }
}

#[component]
fn UserBox() -> impl IntoView {
    view! {
        <b>"User"</b>
    }
}