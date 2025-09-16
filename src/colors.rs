use crate::hex::*;
use crate::hindi_colors::COLORS_HINDI;
use crate::national_colors::COLORS_NATIONAL;

use crate::pantone_colors::*;
use crate::persian_colors::*;
use crate::rgb::*;
use once_cell::sync::Lazy;

// src/colors.rs
#[cfg(feature = "github-colors")]
use crate::github_colors::COLORS_GITHUB;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin {
    #[default]
    All, // show all lists together
    Css,
    Hindi,
    Persian,
    Pantone,
    XKCD,
    #[cfg(feature = "github-colors")]
    GitHub,
    National,
}

impl Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Origin::All => "all",
            Origin::Css => "Css",
            Origin::Hindi => "Hindi",
            Origin::Persian => "Persian",
            Origin::Pantone => "Pantone",
            #[cfg(feature = "github-colors")]
            Origin::GitHub => "github",
            Origin::XKCD => "Xkcd",
            Origin::National => "National",
        };
        f.write_str(s)
    }
}

// ---- Per-origin arrays ------------------------------------------------------
// Fill these out with your real data. I've put a few examples so it compiles.

pub const COLORS_CSS: &[(&str, &str)] = &[
    ("#FF6347", "tomato"),
    ("#4169E1", "royalblue"),
    ("#87CEFA", "lightskyblue"),
    ("#8A2BE2", "blueviolet"),
];

pub const COLORS_XKCD: &[(&str, &str)] = &[
    ("#952E8F", "warm purple"),
    ("#ACC2D9", "cloudy blue"),
    ("#FF6F52", "orange pink"),
];

// Optional: keep your existing combined list if you already have one.
// Otherwise we can just “flatten” at runtime when Origin::All is chosen.
pub const COLORS_ALL_FALLBACK: &[(&str, &str)] = &[
    // You can leave this empty and we'll build a flattened Vec when needed.
    // Or, if you already have a giant combined array, point to it here.
];

/// Return the slice for a given origin.
/// For `All`, we *build* a flattened Vec on demand (so we return a Vec).
pub fn colors_for(origin: Origin) -> ColorsFor {
    match origin {
        #[cfg(feature = "github-colors")]
        Origin::GitHub => ColorsFor::Slice(COLORS_GITHUB),

        Origin::Css => ColorsFor::Slice(COLORS_XKCD),
        Origin::XKCD => ColorsFor::Slice(COLORS_XKCD),
        Origin::Hindi => ColorsFor::Slice(COLORS_HINDI),
        Origin::Persian => ColorsFor::Slice(COLORS_PERSIAN),
        Origin::Pantone => ColorsFor::Slice(COLORS_PANTONE),
        Origin::National => ColorsFor::Slice(COLORS_NATIONAL.as_slice()),

        Origin::All => {
            if !COLORS_ALL_FALLBACK.is_empty() {
                ColorsFor::Slice(COLORS_ALL_FALLBACK)
            } else {
                // build a flattened Vec<&'static (&str, &str)>
                let mut v: Vec<(&'static str, &'static str)> = Vec::new();
                v.extend_from_slice(COLORS_CSS);
                v.extend_from_slice(COLORS_XKCD);
                v.extend_from_slice(COLORS_HINDI);
                v.extend_from_slice(COLORS_PERSIAN);
                v.extend_from_slice(COLORS_PANTONE);
                v.extend_from_slice(COLORS_NATIONAL.as_slice());

                #[cfg(feature = "github-colors")]
                v.extend_from_slice(COLORS_GITHUB);

                ColorsFor::Owned(v)
            }
        }
        _ => {
            todo!()
        }
    }
}

/// A borrowing or owned list, so the caller can handle both cases uniformly.
pub enum ColorsFor {
    Slice(&'static [(&'static str, &'static str)]),
    Owned(Vec<(&'static str, &'static str)>),
}

impl ColorsFor {
    pub fn as_slice(&self) -> &[(&'static str, &'static str)] {
        match self {
            ColorsFor::Slice(s) => s,
            ColorsFor::Owned(v) => v.as_slice(),
        }
    }
}

pub static COMBINED_COLORS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
    // Base sets (always included)
    let base = COLORS_PERSIAN
        .iter()
        .copied()
        .chain(COLORS_XKCD.iter().copied())
        .chain(COLORS_NATIONAL.iter().copied())
        .chain(COLORS_HINDI.iter().copied())
        .chain(COLORS_PANTONE.iter().copied());

    // Conditionally add GitHub colors
    #[cfg(feature = "github-colors")]
    let it = base.chain(COLORS_GITHUB.iter().copied());

    #[cfg(not(feature = "github-colors"))]
    let it = base;

    it.collect()
});

/// Convenience: only the subset where red channel = 0x00.
pub fn r_eq_00_names() -> Vec<(&'static str, &'static str)> {
    COMBINED_COLORS
        .iter()
        .cloned()
        .filter(|(hex, _)| &hex[1..3] == "00")
        .collect()
}

/// Find the nearest color in the full CSS table.
pub fn nearest_css_color(hex_or_norm: &str) -> (&'static str, &'static str, u32) {
    let norm = if hex_or_norm.len() == 7 && hex_or_norm.starts_with('#') {
        hex_or_norm.to_string()
    } else {
        normalize_hex(hex_or_norm).unwrap_or_else(|_| "#000000".to_string())
    };
    let target = hex_to_rgb(&norm).unwrap_or(Rgb { r: 0, g: 0, b: 0 });

    let mut best_name = "unknown";
    let mut best_hex = "#000000";
    let mut best_d2 = u32::MAX;

    for (h, name) in COMBINED_COLORS.iter() {
        if let Some(rgb) = hex_to_rgb(h) {
            let d2 = dist2(target, rgb);
            if d2 < best_d2 {
                best_d2 = d2;
                best_name = name;
                best_hex = h;
            }
        }
    }
    (best_name, best_hex, best_d2)
}

// in src/names.orig
pub fn nearest_name_r_eq_00(hex_or_norm: &str) -> (&'static str, &'static str, u32) {
    let subset = r_eq_00_names();
    let norm = if hex_or_norm.len() == 7 && hex_or_norm.starts_with('#') {
        hex_or_norm.to_string()
    } else {
        normalize_hex(hex_or_norm).unwrap_or_else(|_| "#000000".to_string())
    };
    let target = hex_to_rgb(&norm).unwrap_or(Rgb { r: 0, g: 0, b: 0 });

    let mut best_name = "unknown";
    let mut best_hex = "#000000";
    let mut best_d2 = u32::MAX;

    for (h, name) in subset {
        if let Some(rgb) = hex_to_rgb(h) {
            let d2 = dist2(target, rgb);
            if d2 < best_d2 {
                best_d2 = d2;
                best_name = name;
                best_hex = h;
            }
        }
    }
    (best_name, best_hex, best_d2)
}
