//! Provides macros for defining and managing stylesheets.

/// Creates a stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `path` - The path to the stylesheet file
/// * `file_name` - The output filename for the stylesheet
#[cfg(any(feature = "ssr", feature = "csr"))]
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
#[cfg(not(any(feature = "ssr", feature = "csr")))]
#[macro_export]
macro_rules! style_sheet {
    ($name:ident, $path:expr, $file_name:expr) => {
        // Generate the original macro output in a module
        mod style_sheet_generated {
            $crate::style_sheet_inner!($path);
        }

        // Export the ClassName and impl
        pub use style_sheet_generated::ClassName as $name;
    };
}

/// Creates an inline stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `file_name` - The output filename for the stylesheet
/// * `tokens` - The CSS rules to be included in the stylesheet
#[cfg(any(feature = "ssr", feature = "csr"))]
#[macro_export]
macro_rules! inline_style_sheet {
    // Version with name and file_name
    ($name:ident, $file_name:expr, $($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }

        pub use style_sheet_generated::ClassName as $name;
        pub static STYLE_SHEET: &'static str = style_sheet_generated::STYLE_SHEET;

        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                $file_name,
                Some(style_sheet_generated::STYLE_SHEET),
            )
        }
    };

    // Version with just name (auto-generated file_name)
    ($name:ident, $($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }

        pub use style_sheet_generated::ClassName as $name;
        pub static STYLE_SHEET: &'static str = style_sheet_generated::STYLE_SHEET;

        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                concat!("inline_", module_path!(), ".css"),
                Some(style_sheet_generated::STYLE_SHEET),
            )
        }
    };

    // Version without name (auto-generated file_name)
    ($($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }

        pub static STYLE_SHEET: &'static str = style_sheet_generated::STYLE_SHEET;

        $crate::inventory::submit! {
            $crate::StyleSheet::new(
                concat!("inline_", module_path!(), ".css"),
                Some(style_sheet_generated::STYLE_SHEET),
            )
        }
    };
}

/// Creates an inline stylesheet with the specified name and path.
///
/// # Arguments
/// * `name` - The identifier for the stylesheet
/// * `file_name` - The output filename for the stylesheet
/// * `tokens` - The CSS rules to be included in the stylesheet
#[cfg(not(any(feature = "ssr", feature = "csr")))]
#[macro_export]
macro_rules! inline_style_sheet {
    // Version with name and file_name
    ($name:ident, $file_name:expr, $($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }

        pub use style_sheet_generated::ClassName as $name;

    };

    // Version with just name (auto-generated file_name)
    ($name:ident, $($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }

        pub use style_sheet_generated::ClassName as $name;

    };

    // Version without name (auto-generated file_name)
    ($($tokens:tt)*) => {
        mod style_sheet_generated {
            $crate::inline_style_sheet_inner!($($tokens)*);
        }
    };
}
