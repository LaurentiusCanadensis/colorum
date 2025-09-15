// src/util.rs
pub use rust_colors::Rgb; // re-use the library type

pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    if s.len() != 6 { return None; }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Rgb { r, g, b }) // <- now returns rust_colors::Rgb
}

pub fn combine_hex(rr: &str, gg: &str, bb: &str) -> String {
    format!("#{}{}{}", rr, gg, bb)
}

pub fn hex_for_name(name: &str) -> Option<&'static str> {
    let n = name.trim();
    crate::colors::COMBINED_COLORS
        .iter()
        .find(|(hex, nm)| {
            let _ = hex;
            nm.eq_ignore_ascii_case(n)
        })
        .map(|(hex, _nm)| *hex)
}

pub fn name_for_hex(hex: &str) -> Option<&'static str> {
    let h = hex.trim();
    crate::colors::COMBINED_COLORS
        .iter()
        .find(|(hx, _nm)| hx.eq_ignore_ascii_case(h))
        .map(|(_hx, nm)| *nm)
}