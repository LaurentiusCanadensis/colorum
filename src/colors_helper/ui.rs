// src/colors_helper/ui.rs
use super::search::{TokenMode, search_in_origin, search_substring, search_tokens_any};
use super::*;

fn css_exact_match(name_lc: &str) -> Option<(&'static str, &'static str)> {
    for (h, n) in COLORS_CSS.iter() {
        if n.as_str().eq_ignore_ascii_case(name_lc) {
            return Some((h.as_str(), n.as_str()));
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
            let v = search_substring(q);
            if let Some(first) = v.first() {
                return Some((first.0.as_str(), first.1.as_str()));
            }
            let v2 = search_tokens_any(q);
            v2.first().map(|(h, n)| (h.as_str(), n.as_str()))
        }
        _ => {
            let v = search_in_origin(origin, q, TokenMode::Any);
            v.first().map(|(h, n)| (h.as_str(), n.as_str()))
        }
    }
}

pub fn dropdown_results_for_ui(origin: Origin, query: &str) -> Vec<(&'static str, &'static str)> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }
    let results = match origin {
        Origin::All => {
            if q.len() < super::SUBSTRING_THRESHOLD {
                search_substring(q)
            } else {
                search_tokens_any(q)
            }
        }
        _ => search_in_origin(origin, q, TokenMode::Any),
    };
    results.into_iter().map(|(h, n)| (h.as_str(), n.as_str())).collect()
}
