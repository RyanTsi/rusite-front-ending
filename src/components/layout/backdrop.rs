use leptos::prelude::*;

#[component]
pub fn Backdrop(
    show: RwSignal<bool>,
) -> impl IntoView { 
    view! {
        <Show when=move || show.get()>
            <div
                class="fixed inset-0 bg-black/50 min-h-screen backdrop-blur-md z-50"
                on:click=move |_| show.set(false)
            ></div>
        </Show>
    }
}