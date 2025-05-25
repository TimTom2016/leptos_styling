use leptos::prelude::*;
use leptos_styling::style_sheet;

style_sheet!(green_button, "src/green_button.css", "green_button");

#[component]
pub fn GreenButton() -> impl IntoView {
    view! {
        <button class=green_button::BUTTON>"Click me"</button>
    }
}
