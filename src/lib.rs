//! Library entry for `colorum`
//! - Re-exports core modules so call sites can `use colorum::*`.
//! - Provides `run_app()` that `main.rs` can call to start the Iced app_gui.

#![forbid(unsafe_code)]
extern crate core as std_core;

pub mod colors; // src/colors/
pub mod colors_helper; // src/colors_helper/
pub mod core; // src/core/ - Core types and utilities
pub mod ui; // src/ui/ - User interface components

// Re-export core types for convenience
pub use core::{color_types, hex, rgb};
pub use ui::{messages, widgets};

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
/// This function would be used to start the GUI application.
// ---- App module -------------------------------------------------------------
// UI module already declared above
// Re-exports (updated)
pub use core::hex::{
    HexError, combine_hex, hex_for_name, name_for_hex, normalize_hex, sanitize_hex2, split_hex,
};
pub use core::rgb::{Rgb, dist2, hex_to_rgb, rgb_to_hex};

// If you want these at the root:
pub use colors_helper::{
    COMBINED_COLORS,
    find_closest_color_name,
    search_in_origin,
    origin_slice,
    lookup_by_name,
    lookup_by_name_ci,
    TokenMode,
};
pub use ui::messages::{Channel, Msg};

// ---- Convenience functions --------------------------------------------------

/// Get the closest color name for an RGB color
///
/// # Example
/// ```
/// use colorum::{Rgb, get_closest_color_name_from_rgb};
///
/// let rgb = Rgb { r: 255, g: 0, b: 0 };
/// if let Some(name) = get_closest_color_name_from_rgb(rgb) {
///     println!("Closest color name: {}", name);
/// }
/// ```
pub fn get_closest_color_name_from_rgb(rgb: Rgb) -> Option<&'static str> {
    find_closest_color_name(rgb)
}

/// Convert hex string to RGB and find closest color name
///
/// # Example
/// ```
/// use colorum::get_closest_color_name_from_hex;
///
/// if let Some(name) = get_closest_color_name_from_hex("#FF0000") {
///     println!("Closest color name: {}", name);
/// }
/// ```
pub fn get_closest_color_name_from_hex(hex: &str) -> Option<&'static str> {
    hex_to_rgb(hex).and_then(find_closest_color_name)
}

/// Search for colors by name across all origins
///
/// # Example
/// ```
/// use colorum::{search_colors, TokenMode};
///
/// let results = search_colors("red", TokenMode::Any);
/// for (hex, name) in results.iter().take(5) {
///     println!("{}: {}", name.as_str(), hex.as_str());
/// }
/// ```
pub fn search_colors(query: &str, mode: TokenMode) -> Vec<(crate::core::color_types::HexCode, crate::core::color_types::ColorName)> {
    search_in_origin(Origin::All, query, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::kelvin_colors::KELVIN_COLORS;
    use crate::core::rgb::{rgb_to_hsl, format_rgb, CopyFormat};

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
        use crate::core::color_types::Entity;

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

    #[test]
    fn test_get_closest_color_name_from_rgb() {
        // Test with known colors
        let red = Rgb { r: 255, g: 0, b: 0 };
        let closest_to_red = get_closest_color_name_from_rgb(red);
        assert!(closest_to_red.is_some(), "Should find a closest color name for pure red");
        println!("Closest to red RGB(255,0,0): {:?}", closest_to_red);

        let blue = Rgb { r: 0, g: 0, b: 255 };
        let closest_to_blue = get_closest_color_name_from_rgb(blue);
        assert!(closest_to_blue.is_some(), "Should find a closest color name for pure blue");
        println!("Closest to blue RGB(0,0,255): {:?}", closest_to_blue);

        let white = Rgb { r: 255, g: 255, b: 255 };
        let closest_to_white = get_closest_color_name_from_rgb(white);
        assert!(closest_to_white.is_some(), "Should find a closest color name for white");
        println!("Closest to white RGB(255,255,255): {:?}", closest_to_white);

        // Test with a specific color that should match closely
        let tomato = Rgb { r: 255, g: 99, b: 71 }; // Close to CSS tomato
        let closest_to_tomato = get_closest_color_name_from_rgb(tomato);
        assert!(closest_to_tomato.is_some(), "Should find a closest color name for tomato-like color");
        println!("Closest to tomato RGB(255,99,71): {:?}", closest_to_tomato);
    }

    #[test]
    fn test_get_closest_color_name_from_hex() {
        // Test with valid hex colors
        let closest_red = get_closest_color_name_from_hex("#FF0000");
        assert!(closest_red.is_some(), "Should find closest color for red hex");
        println!("Closest to #FF0000: {:?}", closest_red);

        let closest_blue = get_closest_color_name_from_hex("#0000FF");
        assert!(closest_blue.is_some(), "Should find closest color for blue hex");
        println!("Closest to #0000FF: {:?}", closest_blue);

        // Test with lowercase hex
        let closest_green = get_closest_color_name_from_hex("#00ff00");
        assert!(closest_green.is_some(), "Should find closest color for lowercase green hex");
        println!("Closest to #00ff00: {:?}", closest_green);

        // Test with short hex format - first normalize it
        let normalized_short = normalize_hex("#f00").unwrap();
        let closest_short = get_closest_color_name_from_hex(&normalized_short);
        assert!(closest_short.is_some(), "Should find closest color for normalized short hex");
        println!("Closest to #f00 (normalized to {}): {:?}", normalized_short, closest_short);

        // Test with invalid hex
        let closest_invalid = get_closest_color_name_from_hex("invalid");
        assert!(closest_invalid.is_none(), "Should return None for invalid hex");

        let closest_no_hash = get_closest_color_name_from_hex("FF0000");
        assert!(closest_no_hash.is_none(), "Should return None for hex without #");
    }

    #[test]
    fn test_search_colors() {
        // Test basic search
        let red_results = search_colors("red", TokenMode::Any);
        assert!(!red_results.is_empty(), "Should find colors containing 'red'");
        println!("Found {} colors containing 'red'", red_results.len());

        // Verify results contain the search term
        for (hex, name) in red_results.iter().take(3) {
            let name_str = name.as_str().to_lowercase();
            assert!(name_str.contains("red"), "Color name '{}' should contain 'red'", name.as_str());
            println!("  {} - {}", name.as_str(), hex.as_str());
        }

        // Test search with no results (very unlikely term)
        let rare_results = search_colors("xyzzyx", TokenMode::Any);
        println!("Found {} colors containing 'xyzzyx'", rare_results.len());
        // We don't assert empty since some color names might surprisingly contain this

        // Test search with common term
        let blue_results = search_colors("blue", TokenMode::Any);
        assert!(!blue_results.is_empty(), "Should find colors containing 'blue'");
        assert!(blue_results.len() >= 5, "Should find at least 5 blue colors");
        println!("Found {} colors containing 'blue'", blue_results.len());

        // Test TokenMode::All (all words must match)
        let dark_blue_results = search_colors("dark blue", TokenMode::All);
        println!("Found {} colors containing both 'dark' and 'blue'", dark_blue_results.len());
        // Should be fewer results than just "blue" since both words must match
    }

    #[test]
    fn test_library_integration() {
        // Test the complete workflow: hex -> rgb -> closest name
        let test_hex = "#FF6347"; // CSS tomato

        // Convert hex to RGB
        let rgb = hex_to_rgb(test_hex).expect("Should convert valid hex to RGB");
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 99);
        assert_eq!(rgb.b, 71);

        // Convert RGB back to hex
        let hex_back = rgb_to_hex(rgb);
        assert_eq!(hex_back, test_hex);

        // Find closest color name
        let closest_name = get_closest_color_name_from_rgb(rgb);
        assert!(closest_name.is_some(), "Should find a closest color name");
        println!("Hex {} -> RGB {:?} -> Closest name: {:?}", test_hex, rgb, closest_name);

        // Test search functionality
        if let Some(name) = closest_name {
            let search_results = search_colors(&name[..3], TokenMode::Any); // Search with first 3 chars
            assert!(!search_results.is_empty(), "Should find colors when searching with part of the closest name");
        }
    }

    #[test]
    fn test_color_distance_calculation() {
        // Test RGB distance calculation
        let red = Rgb { r: 255, g: 0, b: 0 };
        let green = Rgb { r: 0, g: 255, b: 0 };
        let blue = Rgb { r: 0, g: 0, b: 255 };

        let red_green_dist = dist2(red, green);
        let red_blue_dist = dist2(red, blue);
        let green_blue_dist = dist2(green, blue);

        println!("Distance red->green: {}", red_green_dist);
        println!("Distance red->blue: {}", red_blue_dist);
        println!("Distance green->blue: {}", green_blue_dist);

        // These should all be the same since they're equidistant in RGB space
        assert_eq!(red_green_dist, red_blue_dist);
        assert_eq!(red_green_dist, green_blue_dist);

        // Distance from a color to itself should be 0
        let same_dist = dist2(red, red);
        assert_eq!(same_dist, 0);

        // Distance should be symmetric
        let dist_ab = dist2(red, green);
        let dist_ba = dist2(green, red);
        assert_eq!(dist_ab, dist_ba);
    }

    #[test]
    fn test_hex_utilities_comprehensive() {
        // Test normalize_hex with various formats
        assert_eq!(normalize_hex("#f00").unwrap(), "#FF0000");
        assert_eq!(normalize_hex("#FF0000").unwrap(), "#FF0000");
        assert_eq!(normalize_hex("#ff0000").unwrap(), "#FF0000");
        assert_eq!(normalize_hex("#FF0000AA").unwrap(), "#FF0000"); // strips alpha

        // Test error cases
        assert!(normalize_hex("FF0000").is_err()); // missing #
        assert!(normalize_hex("#FF00").is_err()); // invalid length
        assert!(normalize_hex("#GG0000").is_err()); // invalid hex chars

        // Test split and combine
        let (r, g, b) = split_hex("#123456").unwrap();
        assert_eq!(r.as_str(), "12");
        assert_eq!(g.as_str(), "34");
        assert_eq!(b.as_str(), "56");

        let combined = combine_hex(&r, &g, &b);
        assert_eq!(combined, "#123456");

        // Test hex name lookups
        if let Some(tomato_hex) = hex_for_name("tomato") {
            println!("Tomato hex: {}", tomato_hex);
            let name_back = name_for_hex(tomato_hex.to_string());
            assert!(name_back.is_some(), "Should find name for tomato hex");
        }
    }

    #[test]
    fn test_origin_functions() {
        // Test origin_slice function
        let css_colors = origin_slice(Origin::Css);
        assert!(!css_colors.is_empty(), "CSS colors should not be empty");
        println!("CSS colors count: {}", css_colors.len());

        let xkcd_colors = origin_slice(Origin::XKCD);
        assert!(!xkcd_colors.is_empty(), "XKCD colors should not be empty");
        println!("XKCD colors count: {}", xkcd_colors.len());

        // Test lookup functions
        if let Some(red_hex) = lookup_by_name("red") {
            println!("Red color hex: {}", red_hex);
            assert!(red_hex.starts_with('#'), "Hex should start with #");
        }

        // Test case-insensitive lookup
        let red_ci = lookup_by_name_ci("RED");
        let red_lower = lookup_by_name_ci("red");
        if red_ci.is_some() && red_lower.is_some() {
            assert_eq!(red_ci, red_lower, "Case-insensitive lookup should return same result");
        }
    }

    #[test]
    fn test_sanitize_hex2() {
        // Test basic sanitization
        assert_eq!(sanitize_hex2("AB"), "AB");
        assert_eq!(sanitize_hex2("ab"), "AB");
        assert_eq!(sanitize_hex2("#AB"), "AB");
        assert_eq!(sanitize_hex2("#ab"), "AB");

        // Test with non-hex characters
        assert_eq!(sanitize_hex2("A1B2C3"), "A1"); // Only takes first 2
        assert_eq!(sanitize_hex2("AGZ"), "A"); // Only takes valid hex chars
        assert_eq!(sanitize_hex2("ZZZ"), ""); // No valid hex chars

        // Test empty and whitespace
        assert_eq!(sanitize_hex2(""), "");
        assert_eq!(sanitize_hex2("   "), "");
        assert_eq!(sanitize_hex2("#"), "");

        // Test mixed valid/invalid
        assert_eq!(sanitize_hex2("A!B@C"), "AB");
        assert_eq!(sanitize_hex2("1G2H3"), "12");
    }

    #[test]
    fn test_hsl_conversion() {
        // Test RGB to HSL conversion for pure colors
        let red = Rgb { r: 255, g: 0, b: 0 };
        let red_hsl = rgb_to_hsl(red);
        assert_eq!(red_hsl.h, 0.0);
        assert_eq!(red_hsl.s, 100.0);
        assert_eq!(red_hsl.l, 50.0);

        let green = Rgb { r: 0, g: 255, b: 0 };
        let green_hsl = rgb_to_hsl(green);
        assert_eq!(green_hsl.h, 120.0);
        assert_eq!(green_hsl.s, 100.0);
        assert_eq!(green_hsl.l, 50.0);

        let blue = Rgb { r: 0, g: 0, b: 255 };
        let blue_hsl = rgb_to_hsl(blue);
        assert_eq!(blue_hsl.h, 240.0);
        assert_eq!(blue_hsl.s, 100.0);
        assert_eq!(blue_hsl.l, 50.0);

        // Test grayscale (achromatic)
        let gray = Rgb { r: 128, g: 128, b: 128 };
        let gray_hsl = rgb_to_hsl(gray);
        assert_eq!(gray_hsl.h, 0.0);
        assert_eq!(gray_hsl.s, 0.0);
        assert!((gray_hsl.l - 50.19).abs() < 1.0); // Approximately 50%

        let white = Rgb { r: 255, g: 255, b: 255 };
        let white_hsl = rgb_to_hsl(white);
        assert_eq!(white_hsl.h, 0.0);
        assert_eq!(white_hsl.s, 0.0);
        assert_eq!(white_hsl.l, 100.0);

        let black = Rgb { r: 0, g: 0, b: 0 };
        let black_hsl = rgb_to_hsl(black);
        assert_eq!(black_hsl.h, 0.0);
        assert_eq!(black_hsl.s, 0.0);
        assert_eq!(black_hsl.l, 0.0);
    }

    #[test]
    fn test_copy_format_enum() {
        // Test format cycling
        assert_eq!(CopyFormat::Hex.next(), CopyFormat::Rgb);
        assert_eq!(CopyFormat::Rgb.next(), CopyFormat::Hsl);
        assert_eq!(CopyFormat::Hsl.next(), CopyFormat::RgbValues);
        assert_eq!(CopyFormat::RgbValues.next(), CopyFormat::Hex);

        // Test display names
        assert_eq!(CopyFormat::Hex.display_name(), "HEX");
        assert_eq!(CopyFormat::Rgb.display_name(), "RGB");
        assert_eq!(CopyFormat::Hsl.display_name(), "HSL");
        assert_eq!(CopyFormat::RgbValues.display_name(), "RGB Values");

        // Test default
        assert_eq!(CopyFormat::default(), CopyFormat::Hex);
    }

    #[test]
    fn test_format_rgb_output() {
        let color = Rgb { r: 255, g: 99, b: 71 }; // Tomato

        // Test HEX format
        let hex_output = format_rgb(color, CopyFormat::Hex);
        assert_eq!(hex_output, "#FF6347");

        // Test RGB format
        let rgb_output = format_rgb(color, CopyFormat::Rgb);
        assert_eq!(rgb_output, "rgb(255, 99, 71)");

        // Test RGB Values format
        let rgb_values_output = format_rgb(color, CopyFormat::RgbValues);
        assert_eq!(rgb_values_output, "255, 99, 71");

        // Test HSL format
        let hsl_output = format_rgb(color, CopyFormat::Hsl);
        assert!(hsl_output.starts_with("hsl("));
        assert!(hsl_output.contains("%"));
        assert!(hsl_output.ends_with(")"));

        // Test specific HSL values for tomato
        let expected_hsl = rgb_to_hsl(color);
        let expected_hsl_str = format!("hsl({:.0}, {:.0}%, {:.0}%)", expected_hsl.h, expected_hsl.s, expected_hsl.l);
        assert_eq!(hsl_output, expected_hsl_str);
    }

    #[test]
    fn test_hex_error_types() {
        // Test different error conditions
        assert_eq!(normalize_hex("FF0000"), Err(HexError::BadFormat)); // No #
        assert_eq!(normalize_hex("#FF"), Err(HexError::UnsupportedLength)); // Too short
        assert_eq!(normalize_hex("#FF00"), Err(HexError::UnsupportedLength)); // Invalid length
        assert_eq!(normalize_hex("#FF0000G"), Err(HexError::BadFormat)); // Invalid hex char
        assert_eq!(normalize_hex("#"), Err(HexError::UnsupportedLength)); // Only #

        // Test error display
        let bad_format = HexError::BadFormat;
        let unsupported = HexError::UnsupportedLength;

        assert_eq!(bad_format.to_string(), "invalid hex format");
        assert_eq!(unsupported.to_string(), "supported: #RGB, #RRGGBB, #RRGGBBAA");

        // Test error equality
        assert_eq!(HexError::BadFormat, HexError::BadFormat);
        assert_eq!(HexError::UnsupportedLength, HexError::UnsupportedLength);
        assert_ne!(HexError::BadFormat, HexError::UnsupportedLength);
    }

    #[test]
    fn test_edge_cases_hex_conversion() {
        // Test hex_to_rgb edge cases
        assert!(hex_to_rgb("").is_none());
        assert!(hex_to_rgb("#").is_none());
        assert!(hex_to_rgb("#FF").is_none());
        assert!(hex_to_rgb("#FF00").is_none());
        assert!(hex_to_rgb("#FF000G").is_none());
        assert!(hex_to_rgb("FF0000").is_none()); // No #
        assert!(hex_to_rgb("#FF0000AA").is_none()); // Too long

        // Test split_hex edge cases
        assert!(split_hex("").is_none());
        assert!(split_hex("#").is_none());
        assert!(split_hex("#FF").is_none());
        assert!(split_hex("#FF0000AA").is_none()); // Too long
        assert!(split_hex("FF0000").is_none()); // No #
        assert!(split_hex("#FF000G").is_none()); // Invalid hex

        // Test valid edge cases
        let valid = split_hex("#000000").unwrap();
        assert_eq!(valid, ("00".to_string(), "00".to_string(), "00".to_string()));

        let valid = split_hex("#FFFFFF").unwrap();
        assert_eq!(valid, ("FF".to_string(), "FF".to_string(), "FF".to_string()));
    }

    #[test]
    fn test_name_hex_lookup_edge_cases() {
        // Test hex_for_name with various cases
        assert!(hex_for_name("").is_none());
        assert!(hex_for_name("   ").is_none());
        assert!(hex_for_name("nonexistentcolor123456789").is_none());

        // Test name_for_hex with various cases
        assert!(name_for_hex("".to_string()).is_none());
        assert!(name_for_hex("   ".to_string()).is_none());
        assert!(name_for_hex("#ZZZZZZ".to_string()).is_none());
        assert!(name_for_hex("#123456".to_string()).is_none()); // Valid hex but probably no name

        // Test with whitespace (should be trimmed)
        if let Some(red_hex) = hex_for_name("  red  ") {
            assert!(red_hex.starts_with('#'));
            // Try to find name back
            if let Some(name) = name_for_hex(format!("  {}  ", red_hex)) {
                assert!(!name.is_empty());
            }
        }
    }

    #[test]
    fn test_combine_hex_case_handling() {
        // Test combine_hex with different cases
        assert_eq!(combine_hex("ff", "00", "00"), "#FF0000");
        assert_eq!(combine_hex("FF", "00", "00"), "#FF0000");
        assert_eq!(combine_hex("Ff", "0a", "bC"), "#FF0ABC");

        // Test with single characters (should work)
        assert_eq!(combine_hex("F", "0", "0"), "#F00");
        assert_eq!(combine_hex("f", "a", "b"), "#FAB");

        // Test empty strings
        assert_eq!(combine_hex("", "", ""), "#");
        assert_eq!(combine_hex("FF", "", "00"), "#FF00");
    }

    #[test]
    fn test_search_colors_advanced() {
        use crate::colors_helper::TokenMode;

        // Test with empty query
        let _empty_results = search_colors("", TokenMode::Any);
        // Empty query should return no results or all results depending on implementation

        // Test with single character
        let single_char = search_colors("a", TokenMode::Any);
        assert!(!single_char.is_empty(), "Should find colors containing 'a'");

        // Test TokenMode::All vs TokenMode::Any
        let any_results = search_colors("dark blue", TokenMode::Any);
        let all_results = search_colors("dark blue", TokenMode::All);

        // TokenMode::Any should find colors with either word
        // TokenMode::All should find colors with both words
        assert!(any_results.len() >= all_results.len(), "Any mode should return at least as many results as All mode");

        // Test case sensitivity
        let lower_results = search_colors("red", TokenMode::Any);
        let upper_results = search_colors("RED", TokenMode::Any);
        assert_eq!(lower_results.len(), upper_results.len(), "Search should be case-insensitive");

        // Test special characters and numbers
        let numeric_results = search_colors("1", TokenMode::Any);
        println!("Colors containing '1': {}", numeric_results.len());

        let special_results = search_colors("-", TokenMode::Any);
        println!("Colors containing '-': {}", special_results.len());
    }

    #[test]
    fn test_rgb_struct_properties() {
        let color = Rgb { r: 100, g: 150, b: 200 };

        // Test Debug trait
        let debug_str = format!("{:?}", color);
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("150"));
        assert!(debug_str.contains("200"));

        // Test Clone and Copy
        let cloned = color.clone();
        let copied = color;

        assert_eq!(color, cloned);
        assert_eq!(color, copied);

        // Test PartialEq
        let same_color = Rgb { r: 100, g: 150, b: 200 };
        let diff_color = Rgb { r: 101, g: 150, b: 200 };

        assert_eq!(color, same_color);
        assert_ne!(color, diff_color);
    }

    #[test]
    fn test_hsl_struct_properties() {
        let hsl = crate::core::rgb::Hsl { h: 180.0, s: 50.0, l: 75.0 };

        // Test Debug trait
        let debug_str = format!("{:?}", hsl);
        assert!(debug_str.contains("180"));
        assert!(debug_str.contains("50"));
        assert!(debug_str.contains("75"));

        // Test Clone and Copy
        let cloned = hsl.clone();
        let copied = hsl;

        assert_eq!(hsl.h, cloned.h);
        assert_eq!(hsl.s, cloned.s);
        assert_eq!(hsl.l, cloned.l);

        // Test floating point comparison with epsilon
        assert!((hsl.h - copied.h).abs() < f32::EPSILON);
        assert!((hsl.s - copied.s).abs() < f32::EPSILON);
        assert!((hsl.l - copied.l).abs() < f32::EPSILON);
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
