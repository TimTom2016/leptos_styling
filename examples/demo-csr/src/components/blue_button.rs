use leptos::prelude::*;
use leptos_styling::{inline_style_sheet, style_sheet};

inline_style_sheet! {blue_button, "blue_button",
    .button {
        background-color: blue;
    }
}

#[component]
pub fn BlueButton() -> impl IntoView {
    view! {
        <button class=blue_button::BUTTON>"Blue Button"</button>
    }
}
