//! rust_colors: helpers and widgets for hex colors via three 2-hex channels.
//!
//! Modules:
//! - `hex`: normalize `#RGB`/`#RRGGBB`/`#RRGGBBAA`, split/combine, sanitize 2-hex
//! - `rgb`: `Rgb` struct + hex/struct conversions + distance
//! - `colors`: named colors (incl. R=00 set) + nearest lookup
//! - `widgets`: iced GUI widgets (color wheel, combined wheel, etc.)
//!
//! Re-exports for convenient use: `normalize_hex`, `split_hex`, `combine_hex`,
//! `sanitize_hex2`, `Rgb`, `hex_to_rgb`, `rgb_to_hex`, `dist2`,
//! `R_EQ_00_HEX_TO_NAME`, `nearest_name_r_eq_00`, `ColorWheel`.
pub mod colors; // your color table / nearest lookup stays as-is
pub mod hex;
pub mod hindi_colors;
pub mod messages;
pub mod pantone_colors;
pub mod persian_colors;

pub mod github_colors;
pub mod rgb;
pub mod widgets; // if you expose the iced widgets
// if the widgets use Msg/Channel

// Re-exports (updated)
pub use hex::{
    HexError, combine_hex, hex_for_name, name_for_hex, normalize_hex, sanitize_hex2, split_hex,
};
pub use rgb::{Rgb, dist2, hex_to_rgb, rgb_to_hex};

// If you want these at the root:
pub use colors::{COMBINED_COLORS, nearest_name_r_eq_00};
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
