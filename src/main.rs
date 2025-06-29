use rusite_front_ending::app::App;

pub fn main() {
    dotenv::dotenv().ok();
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}