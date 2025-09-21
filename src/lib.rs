//! Library entry for `rust_colors`
//! - Re-exports core modules so call sites can `use rust_colors::*`.
//! - Provides `run_app()` that `main.rs` can call to start the Iced app_gui.

#![forbid(unsafe_code)]
extern crate core;

pub mod colors; // src/colors/
pub mod colors_helper; // src/colors_helper/
pub mod hex;
pub mod messages;
pub mod rgb;

// If you keep top-level widgets separate from ui/widgets, expose them here.
// Prefer folding them into `ui::widgets` long-term.
pub mod widgets;

// ---- Re-exports for ergonomics ---------------------------------------------

// Common search/selection surface the UI uses
pub use colors_helper::{Origin, best_first_for_ui, dropdown_results_for_ui};

// Frequently used color tables (optional, but convenient)
pub use colors::{
    css_colors::COLORS_CSS, hindi_colors::COLORS_HINDI, national_colors::COLORS_NATIONAL,
    pantone_colors::COLORS_PANTONE, persian_colors::COLORS_PERSIAN, xkcd_colors::COLORS_XKCD,
};

pub use colors::brand_colors::COLORS_BRANDS;
#[cfg(feature = "github-colors")]
pub use colors::github_colors::COLORS_GITHUB;

// ---- App runner glue for main.rs -------------------------------------------

use iced::Result as IcedResult;

/// Run the Iced application with default settings.
///
/// Typical `main.rs`:
/// ```
/// fn main() -> iced::Result {
///     rust_colors::run_app()
/// }
/// ```
// ---- App module -------------------------------------------------------------
// Expecting `src/app_gui.rs` to define `pub struct App;` that implements
// `iced::Application`. If your app_gui lives under `src/app_gui/mod.rs`, keep `mod app_gui;`.
pub mod app_gui;
// Re-exports (updated)
pub use hex::{
    HexError, combine_hex, hex_for_name, name_for_hex, normalize_hex, sanitize_hex2, split_hex,
};
pub use rgb::{Rgb, dist2, hex_to_rgb, rgb_to_hex};

// If you want these at the root:
pub use colors_helper::COMBINED_COLORS;
pub use messages::{Channel, Msg};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_variants() {
        assert_eq!(normalize_hex("#3af").unwrap(), "#33AAFF");
        assert_eq!(normalize_hex("#33aaff").unwrap(), "#33AAFF");
        assert_eq!(normalize_hex("#33AAFFCC").unwrap(), "#33AAFF"); // strips alpha
        assert!(normalize_hex("33AAFF").is_err()); // must start with '#'
        assert!(normalize_hex("#33AA").is_err()); // #RGBA not supported
    }

    #[test]
    fn split_and_combine() {
        let n = normalize_hex("#3af").unwrap(); // -> #33AAFF
        let (r, g, b) = split_hex(&n).unwrap();
        assert_eq!((r.as_str(), g.as_str(), b.as_str()), ("33", "AA", "FF"));

        let c = combine_hex(&r, &g, &b);
        assert_eq!(c, "#33AAFF");
    }

    #[test]
    fn roundtrip_rgb() {
        let rgb = Rgb {
            r: 0x12,
            g: 0x34,
            b: 0x56,
        };
        let h = rgb_to_hex(rgb);
        assert_eq!(h, "#123456");
        let back = hex_to_rgb(&h).unwrap();
        assert_eq!(back, rgb);
    }

    #[test]
    fn nearest_re00() {
        let (name, hex, _d2) = nearest_name_r_eq_00("#00FE7F");
        assert_eq!(hex, "#00FF7F");
        assert_eq!(name, "springgreen");
    }
}
#[cfg(feature = "profile")]
pub fn init_profiling() {
    use tracing_subscriber::{EnvFilter, fmt};
    // RUST_LOG controls verbosity at runtime; defaults below if not set:
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,rust_colors=trace".into());

    // Ignore double-init errors if multiple crates call it:
    let _ = fmt().with_env_filter(filter).compact().try_init();
}

#[cfg(not(feature = "profile"))]
pub fn init_profiling() {}
