use crate::StyleSheet;
use leptos::logging::log;
use leptos::prelude::*;

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
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let stylesheets: SharedValue<Vec<String>> = SharedValue::new(move || {
        #[cfg(any(feature = "ssr", feature = "csr"))]
        {
            inventory::iter::<StyleSheet>
                .into_iter()
                .map(|style_sheet| {
                    if style_sheet.name.ends_with(".css") {
                        style_sheet.name.to_string()
                    } else {
                        format!("{}.css", style_sheet.name)
                    }
                })
                .collect::<Vec<_>>()
        }
        #[cfg(not(any(feature = "ssr", feature = "csr")))]
        Vec::new()
    });
    let links: Vec<String> = stylesheets
        .into_inner()
        .into_iter()
        .filter(|name| seen.insert(name.clone()))
        .collect();
    view! {
        {links.into_iter().map(|link| view! { <link rel="stylesheet" href={format!("{}/{link}",base_url.trim_end_matches("/"))} /> }).collect::<Vec<_>>()}
    }
}
#[cfg(feature = "csr")]
#[component]
pub fn StyleSheets(#[prop(default = "pkg")] base_url: &'static str) -> impl IntoView {
    use std::collections::HashMap;
    let mut grouped: HashMap<&str, String> = HashMap::new();
    for style_sheet in inventory::iter::<StyleSheet>() {
        grouped
            .entry(style_sheet.name)
            .or_default()
            .push_str(style_sheet.content.unwrap_or_default());
    }
    let style_sheet_content: String = grouped.values().cloned().collect();
    view! {
        <style>{style_sheet_content}</style>
    }
}
