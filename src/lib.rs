//! rust_colors: UI-free helpers for hex colors via three 2-hex channels.
//!
//! Modules:
//! - `hex`: normalize `#RGB`/`#RRGGBB`/`#RRGGBBAA`, split/combine, sanitize 2-hex
//! - `rgb`:  `Rgb` struct + hex/struct conversions + distance
//! - `names`: R=00 named colors + nearest lookup
//!
//! Re-exports for convenient use: `normalize_hex`, `split_hex`, `combine_hex`,
//! `sanitize_hex2`, `Rgb`, `hex_to_rgb`, `rgb_to_hex`, `dist2`,
//! `R_EQ_00_HEX_TO_NAME`, `nearest_name_r_eq_00`.

pub mod hex;
pub mod rgb;

// Re-export the most commonly used items
pub use hex::{HexError, combine_hex, normalize_hex, sanitize_hex2, split_hex};
pub use rgb::{Rgb, dist2, hex_to_rgb, rgb_to_hex};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::nearest_name_r_eq_00;

    #[test]
    fn normalize_variants() {
        assert_eq!(normalize_hex("#3af").unwrap(), "#33AAFF");
        assert_eq!(normalize_hex("#33aaff").unwrap(), "#33AAFF");
        assert_eq!(normalize_hex("#33AAFFCC").unwrap(), "#33AAFF");
        assert!(normalize_hex("33AAFF").is_err());
        assert!(normalize_hex("#33AA").is_err());
    }

    #[test]
    fn split_and_combine() {
        let n = normalize_hex("#3af").unwrap();
        let (r, g, b) = split_hex(&n).unwrap();

        // Borrow the Strings as &str for the assertion (doesn’t move)

        // Now you can still use r/g/b
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
