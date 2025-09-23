//! Library entry for `colorum`
//! - Re-exports core modules so call sites can `use colorum::*`.
//! - Provides `run_app()` that `main.rs` can call to start the Iced app_gui.

#![forbid(unsafe_code)]
extern crate core;

pub mod colors; // src/colors/
pub mod colors_helper; // src/colors_helper/
pub mod color_types; // src/color_types.rs
pub mod hex;
pub mod messages;
pub mod rgb;

// If you keep top-level widgets separate from ui/widgets, expose them here.
// Prefer folding them into `ui::widgets` long-term.
pub mod widgets;

pub mod brand;

// ---- Re-exports for ergonomics ---------------------------------------------

// Common search/selection surface the UI uses
pub use colors_helper::Origin;
// Temporarily disabled: pub use colors_helper::{best_first_for_ui, dropdown_results_for_ui};

// Frequently used color tables (optional, but convenient)
pub use colors::{
    css_colors::COLORS_CSS, hindi_colors::COLORS_HINDI, national_colors::COLORS_NATIONAL,
    pantone_colors::COLORS_PANTONE, persian_colors::COLORS_PERSIAN, xkcd_colors::COLORS_XKCD,
};

pub use colors::brand_colors::COLORS_BRANDS;
#[cfg(feature = "github-colors")]
pub use colors::github_colors::COLORS_GITHUB;

// Import new palette modules to ensure inventory registration
// Disabled for now: use colors::{seasons, canadian_provinces};

// ---- App runner glue for main.rs -------------------------------------------

// Unused: use iced::Result as IcedResult;

/// Run the Iced application with default settings.
///
/// Typical `main.rs`:
/// ```
/// fn main() -> iced::Result {
///     colorum::run_app()
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
    use crate::colors::kelvin_colors::KELVIN_COLORS;

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

    // #[test]
    // fn nearest_re00() {
    //     let (name, hex, _d2) = nearest_name_r_eq_00("#00FE7F");
    //     assert_eq!(hex, "#00FF7F");
    //     assert_eq!(name, "springgreen");
    // }

    #[test]
    fn kelvin_colors_sorted() {
        println!("First 10 Kelvin colors:");
        for (i, (hex, name)) in KELVIN_COLORS.iter().take(10).enumerate() {
            println!("{}: {} - {}", i + 1, name.as_str(), hex.as_str());
        }

        // Check that first color has highest temperature
        let first_name = KELVIN_COLORS[0].1.as_str();
        println!("First color: {}", first_name);
        assert!(first_name.contains("20000K"));
    }

    #[test]
    fn test_italian_brands_search() {
        use crate::colors_helper::{search_in_origin, Origin, TokenMode, origin_slice};

        println!("Testing Italian Brands search...");

        // Test direct access to Italian Brands colors
        let slice = origin_slice(Origin::ItalianBrands);
        println!("Total Italian Brands colors: {}", slice.len());

        // Test single character search "a"
        let results_a = search_in_origin(Origin::ItalianBrands, "a", TokenMode::Any);
        println!("Query 'a': {} results", results_a.len());
        for (i, (hex, name)) in results_a.iter().take(3).enumerate() {
            println!("  {}: {} - {}", i + 1, name.as_str(), hex.as_str());
        }

        // Test two character search "as"
        let results_as = search_in_origin(Origin::ItalianBrands, "as", TokenMode::Any);
        println!("Query 'as': {} results", results_as.len());
        for (i, (hex, name)) in results_as.iter().take(3).enumerate() {
            println!("  {}: {} - {}", i + 1, name.as_str(), hex.as_str());
        }

        // Manual check of colors containing 'a'
        let matching_a_count = slice.iter()
            .filter(|(_, name)| name.as_str().to_lowercase().contains("a"))
            .count();
        let matching_a_sample: Vec<_> = slice.iter()
            .filter(|(_, name)| name.as_str().to_lowercase().contains("a"))
            .take(5)
            .collect();
        println!("Manual search - colors containing 'a': {} total (showing first 5)", matching_a_count);
        for (i, (hex, name)) in matching_a_sample.iter().enumerate() {
            println!("  {}: {} - {}", i + 1, name.as_str(), hex.as_str());
        }

        assert!(slice.len() > 0, "Italian Brands should have colors");
        assert!(matching_a_count > 0, "Should have colors containing 'a'");

        // The search should return the same results as manual filtering
        assert_eq!(results_a.len(), matching_a_count,
                   "Search for 'a' should return same number as manual filter");
    }

    #[test]
    fn test_entity_filtering() {
        use crate::colors_helper::{search_in_origin, Origin, TokenMode};
        use crate::color_types::Entity;

        // Test Entity:Temperature
        let temp_results = search_in_origin(Origin::All, "Entity:Temperature", TokenMode::Any);
        println!("Temperature results: {} colors", temp_results.len());
        for (i, (hex, name)) in temp_results.iter().take(5).enumerate() {
            println!("  {}: {} - {} (entity: {:?})", i + 1, name.as_str(), hex.as_str(), name.entity());
        }

        // Test Entity:Brand
        let brand_results = search_in_origin(Origin::All, "Entity:Brand", TokenMode::Any);
        println!("Brand results: {} colors", brand_results.len());
        for (i, (hex, name)) in brand_results.iter().take(5).enumerate() {
            println!("  {}: {} - {} (entity: {:?})", i + 1, name.as_str(), hex.as_str(), name.entity());
        }

        // Test Entity:Temperature|Brand
        let combined_results = search_in_origin(Origin::All, "Entity:Temperature|Brand", TokenMode::Any);
        println!("Combined Temperature|Brand results: {} colors", combined_results.len());

        // Test Entity:Temperature|Brands (with 's')
        let combined_results_s = search_in_origin(Origin::All, "Entity:Temperature|Brands", TokenMode::Any);
        println!("Combined Temperature|Brands results: {} colors", combined_results_s.len());

        assert!(temp_results.len() > 0, "Should have Temperature colors");
        assert!(brand_results.len() > 0, "Should have Brand colors");
        assert!(combined_results.len() > 0, "Should have Temperature|Brand colors");
        assert!(combined_results_s.len() > 0, "Should have Temperature|Brands colors");

        // Test exactly as UI would call it - with TokenMode::Any (no spaces in query)
        let ui_style_results = search_in_origin(Origin::All, "Entity:Temperature|Brand", TokenMode::Any);
        println!("UI-style search results: {} colors", ui_style_results.len());
        assert!(ui_style_results.len() > 0, "Should have UI-style Temperature|Brand colors");
    }
}
#[cfg(feature = "profile")]
pub fn init_profiling() {
    use tracing_subscriber::{EnvFilter, fmt};
    // RUST_LOG controls verbosity at runtime; defaults below if not set:
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,colorum=trace".into());

    // Ignore double-init errors if multiple crates call it:
    let _ = fmt().with_env_filter(filter).compact().try_init();
}

#[cfg(not(feature = "profile"))]
pub fn init_profiling() {}
