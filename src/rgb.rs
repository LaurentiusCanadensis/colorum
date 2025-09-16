/// Simple RGB struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Convert normalized "#RRGGBB" to `Rgb`.
pub fn hex_to_rgb(norm: &str) -> Option<Rgb> {
    if norm.len() != 7 || !norm.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&norm[1..3], 16).ok()?;
    let g = u8::from_str_radix(&norm[3..5], 16).ok()?;
    let b = u8::from_str_radix(&norm[5..7], 16).ok()?;
    Some(Rgb { r, g, b })
}

/// Convert `Rgb` to `#RRGGBB`.
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

/// Euclidean distance squared in RGB (no gamma/perc.).
pub fn dist2(a: Rgb, b: Rgb) -> u32 {
    let dr = a.r as i32 - b.r as i32;
    let dg = a.g as i32 - b.g as i32;
    let db = a.b as i32 - b.b as i32;
    (dr * dr + dg * dg + db * db) as u32
}
