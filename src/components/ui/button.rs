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