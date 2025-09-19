use crate::colors_helper::{
    HEAVY_MIN_QUERY, MAX_RESULTS, Origin, TokenMode, is_heavy_origin, origin_names, origin_rank,
    search_in_origin,
};

/// Build a filtered, ranked list of names for a given origin and query.
pub fn filtered_names_for_origin(origin: Origin, search: &str) -> Vec<&'static str> {
    let q = search.trim();

    if is_heavy_origin(origin) && q.len() < HEAVY_MIN_QUERY {
        return Vec::new();
    }

    if q.is_empty() && !is_heavy_origin(origin) {
        return origin_names(origin).to_vec();
    }

    let mode = if q.contains(' ') {
        TokenMode::All
    } else {
        TokenMode::Any
    };
    let mut v: Vec<&'static str> = search_in_origin(origin, q, mode)
        .into_iter()
        .map(|(_h, n)| n)
        .collect();

    let rank = origin_rank(origin);
    v.sort_unstable_by_key(|n| rank.get(n).copied().unwrap_or(usize::MAX));

    if v.len() > MAX_RESULTS {
        v.truncate(MAX_RESULTS);
    }

    v
}

/// Retain the selection if it’s still in the filtered list.
pub fn retain_selection<'a>(
    current: Option<&'a str>,
    names: &[&'static str],
) -> Option<&'static str> {
    current.and_then(|cur| names.iter().copied().find(|s| s.eq_ignore_ascii_case(cur)))
}
