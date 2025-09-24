use std::fmt;

/// Errors for hex parsing/normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexError {
    BadFormat,
    UnsupportedLength, // must be 3, 6, or 8 hex digits after '#'
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexError::BadFormat => write!(f, "invalid hex format"),
            HexError::UnsupportedLength => write!(f, "supported: #RGB, #RRGGBB, #RRGGBBAA"),
        }
    }
}
impl std::error::Error for HexError {}

/// Normalize `#RGB`, `#RRGGBB`, or `#RRGGBBAA` into uppercase `#RRGGBB`.
/// Alpha, if present, is ignored.
pub fn normalize_hex(s: &str) -> Result<String, HexError> {
    let s = s.trim();
    if !s.starts_with('#') {
        return Err(HexError::BadFormat);
    }
    let digits = &s[1..];
    if !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(HexError::BadFormat);
    }
    let out = match digits.len() {
        3 => {
            // #RGB -> #RRGGBB
            let mut o = String::with_capacity(7);
            o.push('#');
            for ch in digits.chars() {
                o.push(ch);
                o.push(ch);
            }
            o
        }
        6 | 8 => {
            let mut o = String::with_capacity(7);
            o.push('#');
            o.push_str(&digits[..6]);
            o
        }
        _ => return Err(HexError::UnsupportedLength),
    };
    Ok(out.to_ascii_uppercase())
}

/// Split a normalized "#RRGGBB" into ("RR","GG","BB") if valid.
pub fn split_hex(norm: &str) -> Option<(String, String, String)> {
    if norm.len() == 7 && norm.starts_with('#') && norm[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        Some((
            norm[1..3].to_string(),
            norm[3..5].to_string(),
            norm[5..7].to_string(),
        ))
    } else {
        None
    }
}

/// Combine three 2-hex strings into `#RRGGBB`. Uppercases the inputs.
pub fn combine_hex(rr: &str, gg: &str, bb: &str) -> String {
    let mut out = String::with_capacity(7);
    out.push('#');
    out.push_str(&rr.to_ascii_uppercase());
    out.push_str(&gg.to_ascii_uppercase());
    out.push_str(&bb.to_ascii_uppercase());
    out
}

/// Sanitize arbitrary user input into **at most two uppercase hex digits**.
/// Accepts optional leading '#'.
pub fn sanitize_hex2(mut s: &str) -> String {
    if let Some('#') = s.chars().next() {
        s = &s[1..];
    }
    let mut out = String::with_capacity(2);
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            out.push(ch.to_ascii_uppercase());
            if out.len() == 2 {
                break;
            }
        }
    }
    out
}

/* -------------------------
Name lookups (moved from util.rs)
------------------------- */

/// Find hex for a given color `name` (case-insensitive) from `colors::COMBINED_COLORS`.
pub fn hex_for_name(name: &str) -> Option<&'static str> {
    let n = name.trim();
    crate::colors_helper::COMBINED_COLORS
        .iter()
        .find(|(_hex, nm)| nm.as_str().eq_ignore_ascii_case(n))
        .map(|(hex, _nm)| hex.as_str())
}

/// Find name for a given `hex` (case-insensitive) from `colors::COMBINED_COLORS`.
pub fn name_for_hex(hex: String) -> Option<&'static str> {
    let h = hex.trim();
    crate::colors_helper::COMBINED_COLORS
        .iter()
        .find(|(hx, _nm)| hx.as_str().eq_ignore_ascii_case(h))
        .map(|(_hx, nm)| nm.as_str())
}
