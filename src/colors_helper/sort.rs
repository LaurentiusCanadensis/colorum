// src/colors_helper/sort.rs
use super::*;

pub fn origin_priority_of(hex: &str, name: &str) -> u8 {
    if COLORS_CSS.iter().any(|&(h, n)| h == hex && n == name) {
        return 0;
    }
    if COLORS_XKCD.iter().any(|&(h, n)| h == hex && n == name) {
        return 1;
    }
    if COLORS_PERSIAN.iter().any(|&(h, n)| h == hex && n == name) {
        return 2;
    }
    if COLORS_PANTONE.iter().any(|&(h, n)| h == hex && n == name) {
        return 3;
    }
    if COLORS_HINDI.iter().any(|&(h, n)| h == hex && n == name) {
        return 4;
    }
    if COLORS_NATIONAL.iter().any(|&(h, n)| h == hex && n == name) {
        return 5;
    }
    #[cfg(feature = "github-colors")]
    if COLORS_GITHUB.iter().any(|&(h, n)| h == hex && n == name) {
        return 6;
    }
    if COLORS_BRANDS.iter().any(|&(h, n)| h == hex && n == name) {
        return 7;
    }
    8
}

// coarser 3-bucket sort for dropdown: CSS (0), XKCD (1), Others (2)
pub fn origin_group_priority(hex: &str, name: &str) -> u8 {
    if COLORS_CSS.iter().any(|&(h, n)| h == hex && n == name) {
        return 0;
    }
    if COLORS_XKCD.iter().any(|&(h, n)| h == hex && n == name) {
        return 1;
    }
    2
}

pub fn sort_dropdown_by_origin(query_lc: &str, out: &mut Vec<(&'static str, &'static str)>) {
    if out.len() <= 1 {
        return;
    }
    let mut keyed: Vec<((u8, u8, String), (&'static str, &'static str))> = out
        .iter()
        .map(|&(h, n)| {
            let name_lc = n.to_ascii_lowercase();
            let exact = if name_lc == query_lc { 0 } else { 1 };
            let group = origin_group_priority(h, n);
            ((group, exact, name_lc), (h, n))
        })
        .collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0));
    out.clear();
    out.extend(keyed.into_iter().map(|(_, pair)| pair));
}
