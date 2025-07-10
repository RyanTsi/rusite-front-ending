use leptos::{ev::{Event, KeyboardEvent}, prelude::*, tachys::renderer::dom::Node};

use crate::state::use_search;

#[component]
pub fn SearchPanle(
    show: RwSignal<bool>,
) -> impl IntoView { 
    let search_state = use_search();

    let input_element: NodeRef<leptos::html::Input> = NodeRef::new();
    Effect::new(move |_| {
        if let Some(input) = input_element.get() {
            let _ = input.focus();
        }
    });
    
    view! {
        <Show when=move || show.get()>
            <div
                class="fixed inset-x-0 mx-auto z-100 top-1/6 max-w-2xl items-center justify-center bg-white min-h-1/2"
                on:keydown=move|ev| {
                    if show.get() && ev.key() == "Escape" {
                        show.set(false);
                    }
                }
            >
                <input
                    id="search"
                    class="w-full rounded-full bg-gray-100 px-4 py-2 text-gray-800 dark:bg-gray-800 dark:text-gray-200"
                    type="text"
                    autocomplete="off"
                    placeholder="Search..."
                    bind:value=search_state.search_query
                    node_ref=input_element
                >

                </input>
            </div>
        </Show>
    }
}