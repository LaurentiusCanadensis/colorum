// src/colors_helper/sort.rs
use super::*;
use crate::core::color_types::{HexCode, ColorName};

pub fn origin_priority_of(hex: &str, name: &str) -> u8 {
    if COLORS_CSS.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 0;
    }
    if COLORS_XKCD.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 1;
    }
    if COLORS_PERSIAN.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 5;
    }
    if COLORS_PANTONE.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 2;
    }
    if COLORS_HINDI.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 8;
    }
    if COLORS_NATIONAL.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 4;
    }
    #[cfg(feature = "github-colors")]
    if COLORS_GITHUB.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 6;
    }
    if COLORS_BRANDS.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 7;
    }

    if COLORS_ITALIANBRANDS
        .iter()
        .any(|(h, n)| h.as_str() == hex && n.as_str() == name)
    {
        return 3;
    }
    9
}

// coarser 3-bucket sort for dropdown: CSS (0), XKCD (1), Others (2)
pub fn origin_group_priority(hex: &str, name: &str) -> u8 {
    if COLORS_CSS.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 0;
    }
    if COLORS_XKCD.iter().any(|(h, n)| h.as_str() == hex && n.as_str() == name) {
        return 1;
    }
    2
}

pub fn sort_dropdown_by_origin(query_lc: &str, out: &mut Vec<(HexCode, ColorName)>) {
    if out.len() <= 1 {
        return;
    }
    let mut keyed: Vec<((u8, u8, String), (HexCode, ColorName))> = out
        .iter()
        .map(|item| {
            let (h, n) = item;
            let name_lc = n.as_str().to_ascii_lowercase();
            let exact = if name_lc == query_lc { 0 } else { 1 };
            let group = origin_group_priority(h.as_str(), n.as_str());
            ((group, exact, name_lc), *item)
        })
        .collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0));
    out.clear();
    out.extend(keyed.into_iter().map(|(_, pair)| pair));
}
