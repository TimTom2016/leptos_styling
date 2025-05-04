# Leptos Styling

[![Crates.io](https://img.shields.io/crates/v/leptos_styling.svg)](https://crates.io/crates/leptos_styling)
[![docs.rs](https://docs.rs/leptos_styling/badge.svg)](https://docs.rs/leptos_styling/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Leptos Styling is a powerful styling solution for [Leptos](https://github.com/leptos-rs/leptos) applications that enables seamless integration of CSS and SCSS stylesheets. It intelligently manages stylesheet embedding in both SSR (Server-Side Rendering) and CSR (Client-Side Rendering) modes.

This crate internally uses [turf](https://docs.rs/turf/) macros for efficient CSS processing and management.

## Features

- **CSS/SCSS Support**: Write your styles in CSS or SCSS and import them directly into your components
- **Smart Bundling**: Automatically handles stylesheet embedding based on your rendering mode (SSR/CSR)
- **Type-Safe Classes**: Generate type-safe CSS class names to prevent runtime errors
- **Zero Runtime Overhead**: All processing happens at compile time
- **Framework Integration**: Seamless integration with Axum and other web frameworks

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
leptos_styling = "0.2.0"

[features]
hydrate = ["leptos_styling/hydrate"]
ssr = ["leptos_styling/ssr"]
```

## Quick Start

### Define Your Styles

```rust
use leptos::prelude::*;
use leptos_styling::{style_sheet, inline_style_sheet};

// Import from external CSS file
style_sheet!(green_button, "src/green_button.css", "green_button");

// Or define styles inline
inline_style_sheet! {red_button, "red_button",
    .button {
        background-color: red;
        padding: 8px 16px;
        border-radius: 4px;
    }
}
```

### Use in Components

```rust
#[component]
pub fn MyButtons() -> impl IntoView {
    view! {
        <div>
            <button class=green_button::BUTTON>"Green Button"</button>
            <button class=red_button::BUTTON>"Red Button"</button>
        </div>
    }
}
```

## Setup

### Client-Side Setup

```rust
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // Initialize styling system
    leptos_styling::init();

    // Mount your app
    leptos::mount::hydrate_body(App);
}
```

### Server-Side Setup

```rust
async fn main() {
    let leptos_options = get_configuration(None).unwrap().leptos_options;

    // Generate stylesheets (Required for SSR)
    generate_style_sheets(leptos_options.clone());

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, move || {
            provide_meta_context();
            view! { <App/> }
        });

    // Start your server...
}
```

### App Integration

```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en"/>
        <StyleSheets/> // Include this to inject styles
        <Router>
            // Your routes here
        </Router>
    }
}
```

## Feature Flags

| Flag      | Description                                      |
|-----------|--------------------------------------------------|
| `ssr`     | Enable server-side rendering support              |
| `csr`     | Enable client-side rendering support              |
| `hydrate` | Enable hydration support (use with SSR)           |

## Version Compatibility

| Leptos Styling | Leptos Version |
|----------------|----------------|
| 0.2.x         | 0.8           |
| 0.1.x         | 0.7           |

## Documentation

- [API Documentation](https://docs.rs/leptos_styling/)
- [Examples](./examples)

## Contributing

We welcome contributions!

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgements

- [Leptos](https://github.com/leptos-rs/leptos) - The web framework
- [turf](https://docs.rs/turf/) - CSS processing macros
