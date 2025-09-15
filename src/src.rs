use crate::hex::normalize_hex;
use crate::rgb::{hex_to_rgb, Rgb, dist2};

/// Named colors where the **red channel is 0x00**.
/// (CSS/X11 “reasonable” subset, including aqua/cyan alias)
pub const R_EQ_00_HEX_TO_NAME: &[(&str, &str)] = &[
    ("#00FFFF", "aqua"),
    ("#00FFFF", "cyan"), // alias
    ("#008B8B", "darkcyan"),
    ("#006400", "darkgreen"),
    ("#483D8B", "darkslateblue"),
    ("#2F4F4F", "darkslategray"),
    ("#00BFFF", "deepskyblue"),
    ("#008000", "green"),
    ("#7CFC00", "lawngreen"),
    ("#00FF00", "lime"),
    ("#32CD32", "limegreen"),
    ("#20B2AA", "lightseagreen"),
    ("#66CDAA", "mediumaquamarine"),
    ("#3CB371", "mediumseagreen"),
    ("#00FA9A", "mediumspringgreen"),
    ("#48D1CC", "mediumturquoise"),
    ("#191970", "midnightblue"),
    ("#000080", "navy"),
    ("#2E8B57", "seagreen"),
    ("#00FF7F", "springgreen"),
    ("#008080", "teal"),
    ("#40E0D0", "turquoise"),
];

/// Find the nearest named color **within the R=00 set**.
/// Accepts raw hex variants; normalizes internally.
/// Returns `(name, named_hex, distance_squared)`.
pub fn nearest_name_r_eq_00(hex_or_norm: &str) -> (&'static str, &'static str, u32) {
    // Accept raw input; normalize if needed
    let norm = if hex_or_norm.len() == 7 && hex_or_norm.starts_with('#') {
        hex_or_norm.to_string()
    } else {
        match normalize_hex(hex_or_norm) {
            Ok(n) => n,
            Err(_) => "#000000".to_string(),
        }
    };

    let target = hex_to_rgb(&norm).unwrap_or(Rgb { r: 0, g: 0, b: 0 });

    let mut best_name = "unknown";
    let mut best_hex = "#000000";
    let mut best_d2 = u32::MAX;

    for (h, name) in R_EQ_00_HEX_TO_NAME {
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