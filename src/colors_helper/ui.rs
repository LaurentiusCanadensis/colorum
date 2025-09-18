// src/colors_helper/ui.rs
use super::search::{TokenMode, search_in_origin, search_substring, search_tokens_any};
use super::*;

fn css_exact_match(name_lc: &str) -> Option<(&'static str, &'static str)> {
    for &(h, n) in COLORS_CSS.iter() {
        if n.eq_ignore_ascii_case(name_lc) {
            return Some((h, n));
        }
    }
    None
}

pub fn best_first_for_ui(origin: Origin, query: &str) -> Option<(&'static str, &'static str)> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }
    match origin {
        Origin::All => {
            if let Some(pair) = css_exact_match(&q.to_lowercase()) {
                return Some(pair);
            }
            let mut v = search_substring(q);
            if let Some(first) = v.first().copied() {
                return Some(first);
            }
            let mut v2 = search_tokens_any(q);
            v2.first().copied()
        }
        _ => {
            let v = search_in_origin(origin, q, TokenMode::Any);
            v.first().copied()
        }
    }
}

pub fn dropdown_results_for_ui(origin: Origin, query: &str) -> Vec<(&'static str, &'static str)> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }
    match origin {
        Origin::All => {
            if q.len() < super::SUBSTRING_THRESHOLD {
                search_substring(q)
            } else {
                search_tokens_any(q)
            }
        }
        _ => search_in_origin(origin, q, TokenMode::Any),
    }
}
