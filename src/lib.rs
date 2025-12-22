#![doc = include_str!("../README.md")]
//! A library for managing stylesheets in Leptos applications.
//! Provides functionality for both server-side rendering (SSR) and client-side styling.

#[cfg(any(feature = "ssr", feature = "csr"))]
pub use inventory;
use leptos::config::LeptosOptions;
use std::{ops::Deref, path::Path};
pub use turf::{inline_style_sheet as inline_style_sheet_inner, style_sheet as style_sheet_inner};
mod component;
mod macros;
pub use component::StyleSheets;

/// Represents a stylesheet with its name and optional content.
/// Used to collect and manage stylesheets across the application.
#[derive(Debug)]
pub struct StyleSheet {
    /// The name of the stylesheet file
    pub name: &'static str,
    /// The optional content of the stylesheet
    pub content: Option<&'static str>,
}

impl StyleSheet {
    /// Creates a new StyleSheet instance with the given name and content.
    ///
    /// # Arguments
    /// * `name` - The name of the stylesheet file
    /// * `content` - Optional content of the stylesheet
    pub const fn new(name: &'static str, content: Option<&'static str>) -> Self {
        Self { name, content }
    }
}

// Register StyleSheet with inventory
#[cfg(any(feature = "ssr", feature = "csr"))]
inventory::collect!(StyleSheet);

/// Generates physical stylesheet files in the specified output directory.
/// Only available when the "ssr" feature is enabled.
///
/// # Arguments
/// * `leptos_option` - Configuration options for Leptos
#[cfg(feature = "ssr")]
pub fn generate_style_sheets(leptos_option: LeptosOptions) {
    use std::collections::HashMap;
    let base_path =
        Path::new(leptos_option.site_root.deref()).join(leptos_option.site_pkg_dir.deref());

    // Group stylesheets by name
    let mut grouped: HashMap<&str, String> = HashMap::new();
    for style_sheet in inventory::iter::<StyleSheet>() {
        let file_name = if style_sheet.name.ends_with(".css") {
            style_sheet.name
        } else {
            // This will always add ".css" if missing
            Box::leak(format!("{}.css", style_sheet.name).into_boxed_str())
        };
        grouped
            .entry(file_name)
            .or_default()
            .push_str(style_sheet.content.unwrap_or_default());
    }

    // Write combined stylesheets
    for (file_name, content) in grouped {
        let file_path = base_path.join(file_name);
        std::fs::write(file_path, content).unwrap();
    }
}

#[cfg(target_family = "wasm")]
unsafe extern "C" {
    fn __wasm_call_ctors();
}
/// Initializes the WASM module by calling constructors.
/// Only available in WASM environments.
#[cfg(feature = "csr")]
pub fn init() {
    unsafe {
        #[cfg(target_family = "wasm")]
        __wasm_call_ctors();
    }
}
