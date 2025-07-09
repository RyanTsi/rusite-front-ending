use leptos::prelude::*;

#[component]
pub fn Backdrop(
    #[prop(default = false)]
    show: bool,
    #[prop(optional)]
    on_click: Option<Box<dyn Fn()>>,
    #[prop(optional, into)]
    class: String
) -> impl IntoView { 
    let default_class = "fixed inset-0 bg-black/50 min-h-screen backdrop-blur-md";
    let current_class = if class.is_empty() {
        default_class.to_string()
    } else {
        class.clone()
    };
    view! {
        <div id="backdrop" class=current_class>
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        </div>
    }
}