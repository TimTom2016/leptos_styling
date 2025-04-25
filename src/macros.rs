//! Provides macros for defining and managing stylesheets.

/// Creates a stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `path` - The path to the stylesheet file
/// * `file_name` - The output filename for the stylesheet
#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! style_sheet {
    ($name:ident, $path:expr, $file_name:expr) => {
        // Generate the original macro output in a module
        mod style_sheet_generated {
            $crate::style_sheet_inner!($path);
        }

        // Export the ClassName and impl
        pub use style_sheet_generated::ClassName as $name;

        // Conditionally export the STYLE_SHEET
        pub static STYLE_SHEET: &'static str = style_sheet_generated::STYLE_SHEET;
        // Submit the StyleSheet to inventory
        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                $file_name,
                Some(style_sheet_generated::STYLE_SHEET),
            )
        }
    };
}
/// Creates a stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `path` - The path to the stylesheet file
/// * `file_name` - The output filename for the stylesheet
#[cfg(not(feature = "ssr"))]
#[macro_export]
macro_rules! style_sheet {
    ($name:ident, $path:expr, $file_name:expr) => {
        // Generate the original macro output in a module
        mod style_sheet_generated {
            $crate::style_sheet_inner!($path);
        }

        // Export the ClassName and impl
        pub use style_sheet_generated::ClassName as $name;
        // Submit the StyleSheet to inventory
        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                $file_name,
                None
            )
        }
    };
}

/// Creates an inline stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `path` - The path to the stylesheet file
/// * `file_name` - The output filename for the stylesheet
#[cfg(feature = "ssr")]
macro_rules! inline_style_sheet {
    ($name:ident, $path:expr, $file_name:expr) => {
        // Generate the original macro output in a module
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($path);
        }

        // Export the ClassName and impl
        pub use style_sheet_generated::ClassName as $name;

        // Conditionally export the STYLE_SHEET
        pub static STYLE_SHEET: &'static str = style_sheet_generated::STYLE_SHEET;
        // Submit the StyleSheet to inventory
        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                $file_name,
                Some(style_sheet_generated::STYLE_SHEET),
            )
        }
    };
}

/// Creates an inline stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `path` - The path to the stylesheet file
/// * `file_name` - The output filename for the stylesheet
#[cfg(not(feature = "ssr"))]
macro_rules! inline_style_sheet {
    ($name:ident, $path:expr, $file_name:expr) => {
        // Generate the original macro output in a module
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($path);
        }

        // Export the ClassName and impl
        pub use style_sheet_generated::ClassName as $name;
        // Submit the StyleSheet to inventory
        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                $file_name,
                None
            )
        }
    };
}
