use crate::StyleSheet;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::{Style, Stylesheet};

/// A component that renders stylesheet links for all registered stylesheets.
///
/// # Properties
/// * `base_url` - The base URL path for stylesheet references (defaults to "pkg")
///
/// # Returns
/// Returns a view containing stylesheet link elements for all registered stylesheets.
#[cfg(not(feature = "csr"))]
#[component]
pub fn StyleSheets(#[prop(default = "pkg")] base_url: &'static str) -> impl IntoView {
    let links: Vec<String> = inventory::iter::<StyleSheet>
        .into_iter()
        .map(|style_sheet| {
            if style_sheet.name.ends_with(".css") {
                style_sheet.name.to_string()
            } else {
                format!("{}.css", style_sheet.name)
            }
        })
        .collect();
    view! {
        {links.into_iter().map(|link| view! { <link rel="stylesheet" href={format!("{}/{link}",base_url.trim_end_matches("/"))} /> }).collect::<Vec<_>>()}
    }
}
#[cfg(feature = "csr")]
#[component]
pub fn StyleSheets(#[prop(default = "pkg")] base_url: &'static str) -> impl IntoView {
    let style_sheet_content: String = inventory::iter::<StyleSheet>
        .into_iter()
        .map(|style_sheet| {
            if let Some(content) = style_sheet.content {
                content.to_string()
            } else {
                String::new()
            }
        })
        .collect::<String>();
    view! {
        <style>{style_sheet_content}</style>
    }
}
