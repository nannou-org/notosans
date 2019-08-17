//! A simple crate providing several of Google's Noto Sans true type fonts as slices of bytes.
//!
//! Please see the `fonts/LICENSE_OFL.txt` in this repository for the license distributed by Google
//! with these fonts.

/// Noto Sans Bold true-type font bytes.
pub const BOLD_TTF: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/NotoSans-Bold.ttf"));

/// Noto Sans Bold Italic true-type font bytes.
pub const BOLD_ITALIC_TTF: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/NotoSans-BoldItalic.ttf"));

/// Noto Sans Italic true-type font bytes.
pub const ITALIC_TTF: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/NotoSans-Italic.ttf"));

/// Noto Sans Regular true-type font bytes.
pub const REGULAR_TTF: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/NotoSans-Regular.ttf"));
