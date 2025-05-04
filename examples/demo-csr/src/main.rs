use demo_csr::App;
use leptos::prelude::*;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos_styling::init();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
