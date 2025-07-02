use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView { 
    view! {
        <div class="flex flex-col justify-center items-center h-[70vh]">
            <div id="title" class="text-6xl font-bold">
                "Rusite"
            </div>
            <div id="subtitle" class="text-xl">
                "A Rust framework for building user interfaces"
            </div>
        </div>
    }
}