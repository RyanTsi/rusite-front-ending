use leptos::prelude::*;

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

) -> impl IntoView { 
    view! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" 
            on:click=move |_| {
            }
        >
            "Click me"
        </button>
    }
}