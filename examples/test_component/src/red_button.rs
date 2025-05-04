use leptos::prelude::*;
use leptos_styling::{inline_style_sheet, style_sheet};

inline_style_sheet! {red_button, "red_button",
    .button {
        background-color: red;
    }
}

#[component]
pub fn RedButton() -> impl IntoView {
    view! {
        <button class=red_button::BUTTON>"Click me"</button>
    }
}
