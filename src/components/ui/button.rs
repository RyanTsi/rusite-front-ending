use leptos::{ev, prelude::*};

#[component]
pub fn Link(
    href: String,
    children: Children,
    #[prop(optional, into)]
    class: String
) -> impl IntoView { 
    let default_class = "text-gray-700 hover:text-blue-500";
    let current_class = if class.is_empty() {
        default_class.to_string()
    } else {
        class.clone()
    };
    view! {
        <a href=href class=current_class>
            {children()}
        </a>
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)]
    class: Signal<String>,
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let on_click_handler = move |e: ev::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(e);
        }
    };

    view! {
        <button class=class
            on:click=on_click_handler
        >
            {children()}
        </button>
    }
}