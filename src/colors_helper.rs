
use crate::hex::*;
use crate::hindi_colors::COLORS_HINDI;
use crate::national_colors::COLORS_NATIONAL;

use crate::pantone_colors::*;
use crate::persian_colors::*;
use crate::rgb::*;
use once_cell::sync::Lazy;

// src/colors_
#[cfg(feature = "github-colors")]
use crate::github_colors::COLORS_GITHUB;
use core::fmt::{self, Display};


use std::hash::{Hash, Hasher};


#[path = "colors/css.rs"]
mod css_colors;
pub use css_colors::COLORS_CSS;

#[path = "colors/xkcd.rs"]
mod xkcd_colors;
pub use xkcd_colors::COLORS_XKCD;
// --- Per-origin token index builder ---
fn build_token_index_for(slice: &[(&'static str, &'static str)]) -> HashMap<String, Box<[usize]>> {
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, &(_, name)) in slice.iter().enumerate() {
        for tok in tokenize_lc(name) {
            idx.entry(tok).or_default().push(i);
        }
    }
    idx.into_iter()
        .map(|(tok, mut v)| {
            v.sort_unstable();
            v.dedup();
            (tok, v.into_boxed_slice())
        })
        .collect()
}

// --- Per-origin slices as statics (borrow correctly; avoid temporary lifetimes) ---
fn origin_slice(origin: Origin) -> &'static [(&'static str, &'static str)] {
    match origin {
        Origin::All => COMBINED_COLORS.as_slice(),
        Origin::XKCD => COLORS_XKCD,
        Origin::Pantone => COLORS_PANTONE,
        Origin::Hindi => COLORS_HINDI,
        Origin::Persian => COLORS_PERSIAN,
        #[cfg(feature = "github-colors")] Origin::GitHub => COLORS_GITHUB,
        Origin::Css => COLORS_CSS,
        Origin::National => COLORS_NATIONAL.as_slice(),
    }
}

// --- Per-origin token indexes (LazyLock) ---
static IDX_ALL: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::All)));
static IDX_XKCD: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::XKCD)));
static IDX_PANTONE: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::Pantone)));
static IDX_HINDI: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::Hindi)));
static IDX_PERSIAN: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::Persian)));
static IDX_NATIONAL: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::National)));
static IDX_CSS: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::Css)));
#[cfg(feature = "github-colors")]
static IDX_GITHUB: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(origin_slice(Origin::GitHub)));

// Helper to pick the right index + slice
fn origin_index(origin: Origin) -> &'static HashMap<String, Box<[usize]>> {
    match origin {
        Origin::All => &IDX_ALL,
        Origin::XKCD => &IDX_XKCD,
        Origin::Pantone => &IDX_PANTONE,
        Origin::Hindi => &IDX_HINDI,
        Origin::Persian => &IDX_PERSIAN,
        Origin::National => &IDX_NATIONAL,
        #[cfg(feature = "github-colors")] Origin::GitHub => &IDX_GITHUB,
        Origin::Css => &IDX_CSS,
    }
}

// Helper to build a sorted name list for an origin
fn build_sorted_names(origin: Origin) -> Box<[&'static str]> {

    let set = colors_for(origin);        // keep the enum alive
    let slice = set.as_slice();          // borrow from it
    let mut names: Vec<&'static str> = slice.iter().map(|&(_h, n)| n).collect();
    names.sort_unstable_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    names.into_boxed_slice()
}

// Helper to build a rank map (name -> order index) for an origin
fn build_rank_map(names: &[&'static str]) -> HashMap<&'static str, usize> {
    let mut m = HashMap::with_capacity(names.len());
    for (i, &n) in names.iter().enumerate() {
        m.insert(n, i);
    }
    m
}

// ---- Per-origin names (sorted) ----
static ORIGIN_NAMES_ALL: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::All));
static ORIGIN_NAMES_XKCD: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::XKCD));
static ORIGIN_NAMES_PANTONE: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Pantone));
static ORIGIN_NAMES_HINDI: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Hindi));
static ORIGIN_NAMES_PERSIAN: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Persian));
static ORIGIN_NAMES_NATIONAL: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::National));
static ORIGIN_NAMES_CSS: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Css));

#[cfg(feature = "github-colors")]
static ORIGIN_NAMES_GITHUB: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::GitHub));

// ---- Per-origin rank maps ----
static ORIGIN_RANK_ALL: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_ALL));
static ORIGIN_RANK_XKCD: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_XKCD));
static ORIGIN_RANK_CSS: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_CSS));
static ORIGIN_RANK_PANTONE: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_PANTONE));
static ORIGIN_RANK_HINDI: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_HINDI));
static ORIGIN_RANK_PERSIAN: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_PERSIAN));
static ORIGIN_RANK_NATIONAL: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_NATIONAL));
#[cfg(feature = "github-colors")]
static ORIGIN_RANK_GITHUB: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_GITHUB));

// Public helpers (no HashMap keys, just a match)
pub fn origin_names(origin: Origin) -> &'static [&'static str] {
    match origin {
        Origin::All => &ORIGIN_NAMES_ALL,
        Origin::XKCD => &ORIGIN_NAMES_XKCD,
        Origin::Pantone => &ORIGIN_NAMES_PANTONE,
        Origin::Hindi => &ORIGIN_NAMES_HINDI,
        Origin::Persian => &ORIGIN_NAMES_PERSIAN,
        Origin::National => &ORIGIN_NAMES_NATIONAL,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => &ORIGIN_NAMES_GITHUB,
        // If you keep Css as an alias, point it at XKCD (or whatever you want)
        Origin::Css => &ORIGIN_NAMES_CSS,
    }
}

pub fn origin_rank(origin: Origin) -> &'static HashMap<&'static str, usize> {
    match origin {
        Origin::All => &ORIGIN_RANK_ALL,
        Origin::XKCD => &ORIGIN_RANK_XKCD,
        Origin::Pantone => &ORIGIN_RANK_PANTONE,
        Origin::Hindi => &ORIGIN_RANK_HINDI,
        Origin::Persian => &ORIGIN_RANK_PERSIAN,
        Origin::National => &ORIGIN_RANK_NATIONAL,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => &ORIGIN_RANK_GITHUB,
        Origin::Css => &ORIGIN_RANK_CSS,
    }
}
impl Hash for Origin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin {
    #[default]
    All, // show all lists together
    Css,
    Hindi,
    Persian,
    Pantone,
    XKCD,
    #[cfg(feature = "github-colors")]
    GitHub,
    National,
}

impl Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Origin::All => "all",
            Origin::Css => "Css",
            Origin::Hindi => "Hindi",
            Origin::Persian => "Persian",
            Origin::Pantone => "Pantone",
            #[cfg(feature = "github-colors")]
            Origin::GitHub => "github",
            Origin::XKCD => "Xkcd",
            Origin::National => "National",
        };
        f.write_str(s)
    }
}


// Optional: keep your existing combined list if you already have one.
// Otherwise we can just “flatten” at runtime when Origin::All is chosen.
pub const COLORS_ALL_FALLBACK: &[(&str, &str)] = &[
    // You can leave this empty and we'll build a flattened Vec when needed.
    // Or, if you already have a giant combined array, point to it here.
];

/// Return the slice for a given origin.
/// For `All`, we *build* a flattened Vec on demand (so we return a Vec).
pub fn colors_for(origin: Origin) -> ColorsFor {
    match origin {
        #[cfg(feature = "github-colors")]
        Origin::GitHub => ColorsFor::Slice(COLORS_GITHUB),

        Origin::Css => ColorsFor::Slice(COLORS_CSS),
        Origin::XKCD => ColorsFor::Slice(COLORS_XKCD),
        Origin::Hindi => ColorsFor::Slice(COLORS_HINDI),
        Origin::Persian => ColorsFor::Slice(COLORS_PERSIAN),
        Origin::Pantone => ColorsFor::Slice(COLORS_PANTONE),
        Origin::National => ColorsFor::Slice(COLORS_NATIONAL.as_slice()),

        Origin::All => {
            if !COLORS_ALL_FALLBACK.is_empty() {
                ColorsFor::Slice(COLORS_ALL_FALLBACK)
            } else {
                // build a flattened Vec<&'static (&str, &str)>
                let mut v: Vec<(&'static str, &'static str)> = Vec::new();
                v.extend_from_slice(COLORS_CSS);
                v.extend_from_slice(COLORS_XKCD);
                v.extend_from_slice(COLORS_PERSIAN);
                v.extend_from_slice(COLORS_PANTONE);
                v.extend_from_slice(COLORS_HINDI);
                v.extend_from_slice(COLORS_NATIONAL.as_slice());

                #[cfg(feature = "github-colors")]
                v.extend_from_slice(COLORS_GITHUB);

                ColorsFor::Owned(v)
            }
        }
        _ => {
            todo!()
        }
    }
}

/// A borrowing or owned list, so the caller can handle both cases uniformly.
pub enum ColorsFor {
    Slice(&'static [(&'static str, &'static str)]),
    Owned(Vec<(&'static str, &'static str)>),
}

pub static COMBINED_COLORS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
    // Base sets (always included)
    let base = COLORS_CSS
        .iter()
        .copied()
        .chain(COLORS_XKCD.iter().copied())
        .chain(COLORS_PERSIAN.iter().copied())
        .chain(COLORS_PANTONE.iter().copied())
        .chain(COLORS_HINDI.iter().copied())
        .chain(COLORS_NATIONAL.iter().copied());

    // Conditionally add GitHub colors
    #[cfg(feature = "github-colors")]
    let it = base.chain(COLORS_GITHUB.iter().copied());

    #[cfg(not(feature = "github-colors"))]
    let it = base;

    it.collect()
});

/// Convenience: only the subset where red channel = 0x00.
pub fn r_eq_00_names() -> Vec<(&'static str, &'static str)> {
    COMBINED_COLORS
        .iter()
        .cloned()
        .filter(|(hex, _)| &hex[1..3] == "00")
        .collect()
}

/// Find the nearest color in the full CSS table.
pub fn nearest_css_color(hex_or_norm: &str) -> (&'static str, &'static str, u32) {
    let norm = if hex_or_norm.len() == 7 && hex_or_norm.starts_with('#') {
        hex_or_norm.to_string()
    } else {
        normalize_hex(hex_or_norm).unwrap_or_else(|_| "#000000".to_string())
    };
    let target = hex_to_rgb(&norm).unwrap_or(Rgb { r: 0, g: 0, b: 0 });

    let mut best_name = "unknown";
    let mut best_hex = "#000000";
    let mut best_d2 = u32::MAX;

    for (h, name) in COLORS_CSS.iter() {
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

// in src/names.orig
pub fn nearest_name_r_eq_00(hex_or_norm: &str) -> (&'static str, &'static str, u32) {
    let subset = r_eq_00_names();
    let norm = if hex_or_norm.len() == 7 && hex_or_norm.starts_with('#') {
        hex_or_norm.to_string()
    } else {
        normalize_hex(hex_or_norm).unwrap_or_else(|_| "#000000".to_string())
    };
    let target = hex_to_rgb(&norm).unwrap_or(Rgb { r: 0, g: 0, b: 0 });

    let mut best_name = "unknown";
    let mut best_hex = "#000000";
    let mut best_d2 = u32::MAX;

    for (h, name) in subset {
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



pub enum TokenMode { Any, All, Substring }
// short inputs use substring (prefix) instead of token index
pub const SUBSTRING_THRESHOLD: usize = 3;


pub fn search_in_origin(origin: Origin, query: &str, mode: TokenMode)
                        -> Vec<(&'static str, &'static str)>
{
    let q = query.trim();
    let slice = origin_slice(origin);

    // AFTER
    if q.is_empty() {
        // For All, don't return the entire universe — let the UI show nothing or a small default.
        if matches!(origin, Origin::All) {
            return Vec::new();
        }
        return slice.to_vec();
    }

    // Fast path: if All and query matches a CSS name exactly, return that immediately.
    if matches!(origin, Origin::All) {
        let qlc = q.to_lowercase();
        if let Some((h, n)) = css_exact_match(&qlc) {
            return vec![(h, n)];
        }
    }

    // Tokenize once
    let tokens: Vec<String> = tokenize_lc(q).collect();

    // Partition tokens by length: short tokens use substring, full tokens use index
    let mut full_toks: Vec<&str> = Vec::new();
    let mut short_toks: Vec<&str> = Vec::new();
    for t in &tokens {
        if t.len() >= SUBSTRING_THRESHOLD {
            full_toks.push(t);
        } else {
            short_toks.push(t);
        }
    }

    // Helper: substring check that ALL short tokens appear in the name
    #[inline]
    fn name_matches_all_short(name: &str, shorts: &[&str]) -> bool {
        if shorts.is_empty() { return true; }
        let nlc = name.to_lowercase();
        shorts.iter().all(|s| nlc.contains(s))
    }

    // Case A: no full tokens (all tokens short) → pure substring path.
    if full_toks.is_empty() {
        let mut out: Vec<(&'static str, &'static str)> = match mode {
            TokenMode::Any => {
                // ANY of the short tokens
                if short_toks.is_empty() {
                    // Shouldn't happen (q non-empty), but guard anyway
                    slice.to_vec()
                } else {
                    slice.iter().copied().filter(|&(_h, n)| {
                        let nlc = n.to_lowercase();
                        short_toks.iter().any(|s| nlc.contains(s))
                    }).collect()
                }
            }
            _ => {
                // ALL or Substring → require ALL short tokens
                slice.iter().copied()
                    .filter(|&(_h, n)| name_matches_all_short(n, &short_toks))
                    .collect()
            }
        };

        // NEW: prefer exact name matches and CSS first when origin = All
        if matches!(origin, Origin::All) {
            let qlc = q.to_lowercase();
            out.sort_by(|&(h1, n1), &(h2, n2)| {
                let e1 = is_exact_ci(n1, &qlc);
                let e2 = is_exact_ci(n2, &qlc);
                match (e1, e2) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let p1 = origin_priority_of(h1, n1);
                        let p2 = origin_priority_of(h2, n2);
                        p1.cmp(&p2)
                            .then_with(|| n1.to_ascii_lowercase().cmp(&n2.to_ascii_lowercase()))
                    }
                }
            });
        }

        return out;    }

    // Case B: we have at least one full token → use per-origin index
    let idx = origin_index(origin);

    // Build postings for the "full" tokens
    let mut lists: Vec<&[usize]> = Vec::new();
    for &tok in &full_toks {
        if let Some(list) = idx.get(tok) {
            lists.push(list.as_ref());
        } else {
            // A required full token doesn't exist in the index
            if matches!(mode, TokenMode::All) {
                return Vec::new();
            }
            // In ANY mode, we simply skip missing tokens
        }
    }

    // If no postings were found (e.g., all full tokens missing) and we're in ANY mode,
    // fall back to substring over short tokens (or whole q if no shorts).
    if lists.is_empty() {
        // Fallback: substring using all tokens (behaves like "Any" if requested)
        let mut out: Vec<(&'static str, &'static str)> = if !short_toks.is_empty() {
            // use short tokens OR semantics for ANY, AND for ALL/Substring
            match mode {
                TokenMode::Any => {
                    slice.iter().copied().filter(|&(_h, n)| {
                        let nlc = n.to_lowercase();
                        short_toks.iter().any(|s| nlc.contains(s))
                    }).collect()
                }
                _ => {
                    slice.iter().copied().filter(|&(_h, n)| {
                        name_matches_all_short(n, &short_toks)
                    }).collect()
                }
            }
        } else {
            // No short tokens either: substring on the whole query (rare)
            let qlc = q.to_lowercase();
            slice.iter().copied()
                .filter(|&(_h, n)| n.to_lowercase().contains(&qlc))
                .collect()
        };
        if matches!(origin, Origin::All) {
            let qlc = q.to_lowercase();
            out.sort_by(|&(h1, n1), &(h2, n2)| {
                let e1 = is_exact_ci(n1, &qlc);
                let e2 = is_exact_ci(n2, &qlc);
                match (e1, e2) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let p1 = origin_priority_of(h1, n1);
                        let p2 = origin_priority_of(h2, n2);
                        p1.cmp(&p2)
                            .then_with(|| n1.to_ascii_lowercase().cmp(&n2.to_ascii_lowercase()))
                    }
                }
            });
        }
        return out;    }

    // Merge postings (prefer smallest first)
    lists.sort_by_key(|p| p.len());
    let mut current: Vec<usize> = lists[0].to_vec();
    for p in lists.iter().skip(1) {
        current = if matches!(mode, TokenMode::All) {
            intersect_sorted_slices(&current, p)
        } else {
            union_sorted_slices(&current, p)
        };
        if current.is_empty() {
            break;
        }
    }

    // If index produced nothing in ANY mode, try a substring fallback on q
    if current.is_empty() {
        let qlc = q.to_lowercase();
        return slice.iter().copied()
            .filter(|&(_h, n)| n.to_lowercase().contains(&qlc))
            .collect();
    }

    // Apply short-token substring filter to the candidate set (AND semantics for ALL/Substring; OR for ANY)
    let mut out: Vec<(&'static str, &'static str)> = Vec::with_capacity(current.len());
    match mode {
        TokenMode::Any => {
            if short_toks.is_empty() {
                // No short filters → return all candidates
                for i in current { out.push(slice[i]); }
            } else {
                for i in current {
                    let name = slice[i].1;
                    let nlc = name.to_lowercase();
                    if short_toks.iter().any(|s| nlc.contains(s)) {
                        out.push(slice[i]);
                    }
                }
            }
        }
        _ => {
            // ALL or Substring → require ALL short tokens to appear
            for i in current {
                let name = slice[i].1;
                if name_matches_all_short(name, &short_toks) {
                    out.push(slice[i]);
                }
            }
        }
    }

    if matches!(origin, Origin::All) {
        let qlc = q.to_lowercase();
        out.sort_by(|&(h1, n1), &(h2, n2)| {
            let e1 = is_exact_ci(n1, &qlc);
            let e2 = is_exact_ci(n2, &qlc);
            match (e1, e2) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => {
                    let p1 = origin_priority_of(h1, n1);
                    let p2 = origin_priority_of(h2, n2);
                    p1.cmp(&p2)
                        .then_with(|| n1.to_ascii_lowercase().cmp(&n2.to_ascii_lowercase()))
                }
            }
        });
    }
    return out;
}

use std::collections::HashMap;
use std::sync::LazyLock;

// Build once over the full combined list.
pub static COLORS_BY_NAME: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    COMBINED_COLORS.iter().map(|(hex, name)| (*name, *hex)).collect()
});

pub static COLORS_BY_NAME_LC: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (hex, name) in COMBINED_COLORS.as_slice() {
        m.insert(name.to_lowercase(), *hex);
    }
    m
});

pub struct ColorEntry {
    pub hex: &'static str,
    pub name: &'static str,
    pub name_lc: String,
}

pub static COLORS_LC: LazyLock<Vec<ColorEntry>> = LazyLock::new(|| {
    COMBINED_COLORS
        .iter()
        .map(|(hex, name)| ColorEntry {
            hex: *hex,
            name: *name,
            name_lc: name.to_lowercase(),
        })
        .collect()
});

fn tokenize_lc(s: &str) -> impl Iterator<Item = String> + '_ {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
}

// Token -> sorted, deduped posting list of indices into COMBINED_COLORS
pub static NAME_TOKEN_INDEX: LazyLock<HashMap<String, Box<[usize]>>> = LazyLock::new(|| {
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, (_, name)) in COMBINED_COLORS.iter().enumerate() {
        for tok in tokenize_lc(name) {
            idx.entry(tok).or_default().push(i);
        }
    }

    idx.into_iter()
        .map(|(tok, mut v)| {
            v.sort_unstable();
            v.dedup();
            v.shrink_to_fit();
            (tok, v.into_boxed_slice())
        })
        .collect()
});

// Lookups
pub fn lookup_by_name(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME.get(name).copied()
}
pub fn lookup_by_name_ci(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME_LC.get(&name.to_lowercase()).copied()
}
pub fn search_substring(query: &str) -> Vec<(&'static str, &'static str)> {
    let qlc = query.to_lowercase();
    let mut out: Vec<(&'static str, &'static str)> = COLORS_LC
        .iter()
        .filter(|e| e.name_lc.contains(&qlc))
        .map(|e| (e.hex, e.name))
        .collect();

    // Prefer exact name matches, then CSS origin, then alphabetical
    sort_results_for_all(&qlc, &mut out);
    if out.len() > MAX_RESULTS { out.truncate(MAX_RESULTS); }
    out
}

// Merge helpers
fn intersect_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len().min(b.len()));
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => { out.push(a[i]); i += 1; j += 1; }
        }
    }
    out
}
fn union_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => { out.push(a[i]); i += 1; }
            std::cmp::Ordering::Greater => { out.push(b[j]); j += 1; }
            std::cmp::Ordering::Equal => { out.push(a[i]); i += 1; j += 1; }
        }
    }
    if i < a.len() { out.extend_from_slice(&a[i..]); }
    if j < b.len() { out.extend_from_slice(&b[j..]); }
    out
}
fn sort_results_for_all(query_lc: &str, out: &mut Vec<(&'static str, &'static str)>) {
    if out.len() <= 1 { return; }
    // Precompute keys once per item; avoids per-compare allocations
    let mut keyed: Vec<((u8, u8, String), (&'static str, &'static str))> = out
        .iter()
        .map(|&(h, n)| {
            let name_lc = n.to_ascii_lowercase();
            let exact = if name_lc == query_lc { 0 } else { 1 };
            let pri = origin_priority_of(h, n);
            ((exact, pri, name_lc), (h, n))
        })
        .collect();

    keyed.sort_by(|a, b| a.0.cmp(&b.0));

    out.clear();
    out.extend(keyed.into_iter().map(|(_, pair)| pair));
}

fn css_exact_match(name_lc: &str) -> Option<(&'static str, &'static str)> {
    for &(h, n) in COLORS_CSS.iter() {
        if n.eq_ignore_ascii_case(name_lc) {
            return Some((h, n));
        }
    }
    None
}

// Global token searches over COMBINED_COLORS
pub fn search_tokens_any(query: &str) -> Vec<(&'static str, &'static str)> {
    let mut postings: Vec<&[usize]> = Vec::new();
    for tok in tokenize_lc(query) {
        if let Some(list) = NAME_TOKEN_INDEX.get(&tok) {
            postings.push(list);
        }
    }
    if postings.is_empty() { return Vec::new(); }
    postings.sort_by_key(|p| p.len());
    let mut result: Vec<usize> = postings[0].to_vec();
    for p in postings.iter().skip(1) {
        result = union_sorted_slices(&result, p);
    }
    let mut out: Vec<(&'static str, &'static str)> =
        result.into_iter().map(|i| COMBINED_COLORS[i]).collect();

    // Prefer exact name matches and CSS first for global (All) searches
    let qlc = query.to_lowercase();
    sort_results_for_all(&qlc, &mut out);
    if out.len() > MAX_RESULTS { out.truncate(MAX_RESULTS); }
    out
}
pub fn search_tokens_all(query: &str) -> Vec<(&'static str, &'static str)> {
    let mut lists: Vec<&[usize]> = {
        let mut tmp = Vec::new();
        for tok in tokenize_lc(query) {
            match NAME_TOKEN_INDEX.get(&tok) {
                Some(list) => tmp.push(list.as_ref()),
                None => return Vec::new(),
            }
        }
        if tmp.is_empty() { return Vec::new(); }
        tmp
    };
    lists.sort_by_key(|p| p.len());
    let mut current: Vec<usize> = lists[0].to_vec();
    for p in lists.iter().skip(1) {
        if current.is_empty() { break; }
        current = intersect_sorted_slices(&current, p);
    }
    let mut out: Vec<(&'static str, &'static str)> =
        current.into_iter().map(|i| COMBINED_COLORS[i]).collect();

    let qlc = query.to_lowercase();
    out.sort_by(|&(h1, n1), &(h2, n2)| {
        let e1 = is_exact_ci(n1, &qlc);
        let e2 = is_exact_ci(n2, &qlc);
        match (e1, e2) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let p1 = origin_priority_of(h1, n1);
                let p2 = origin_priority_of(h2, n2);
                p1.cmp(&p2)
                    .then_with(|| n1.to_ascii_lowercase().cmp(&n2.to_ascii_lowercase()))
            }
        }
    });
    out
}
impl ColorsFor {
    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [(&'static str, &'static str)] {
        match self {
            ColorsFor::Slice(s) => s,
            ColorsFor::Owned(v) => v.as_slice(),
        }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<(&'static str, &'static str)> {
        self.as_slice().to_vec()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, (&'static str, &'static str)> {
        self.as_slice().iter()
    }
}


pub static ORIGIN_NAMES: LazyLock<HashMap<Origin, Box<[&'static str]>>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for origin in [
        Origin::All,
        Origin::Css,
        Origin::XKCD, Origin::Pantone,
        Origin::Hindi, Origin::Persian, Origin::National,
    #[cfg(feature = "github-colors")]
    Origin::GitHub
    ] {
        let set = colors_for(origin);        // keep the enum alive
        let slice = set.as_slice();          // borrow from it
        let mut names: Vec<&'static str> = slice.iter().map(|&(_h, n)| n).collect();
        names.sort_unstable_by(|a,b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
        m.insert(origin, names.into_boxed_slice());
    }
    m
});

// Optional: ranking map to sort filtered hits in the same stable per-origin order
pub static ORIGIN_RANK: LazyLock<HashMap<Origin, HashMap<&'static str, usize>>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (&origin, names) in ORIGIN_NAMES.iter() {
        let mut rank = HashMap::with_capacity(names.len());
        for (i, &n) in names.iter().enumerate() {
            rank.insert(n, i);
        }
        m.insert(origin, rank);
    }
    m
});


// tune these
pub const HEAVY_MIN_QUERY: usize = 1;   // require 1+ chars for heavy sets (so 2-letter inputs show)
pub const MAX_RESULTS: usize = 200;     // cap the dropdown size
#[inline]
pub fn is_heavy_origin(origin: Origin) -> bool {
    match origin {
        Origin::All => true,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => true,
        _ => false,
    }
}


fn is_exact_ci(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

fn origin_priority_of(hex: &str, name: &str) -> u8 {
    // Lower is better: CSS first, then XKCD, Persian, Pantone, Hindi, National, GitHub.
    if COLORS_CSS.iter().any(|&(h, n)| h == hex && n == name) { return 0; }
    if COLORS_XKCD.iter().any(|&(h, n)| h == hex && n == name) { return 1; }
    if COLORS_PERSIAN.iter().any(|&(h, n)| h == hex && n == name) { return 2; }
    if COLORS_PANTONE.iter().any(|&(h, n)| h == hex && n == name) { return 3; }
    if COLORS_HINDI.iter().any(|&(h, n)| h == hex && n == name) { return 4; }
    if COLORS_NATIONAL.iter().any(|&(h, n)| h == hex && n == name) { return 5; }
    #[cfg(feature = "github-colors")]
    if COLORS_GITHUB.iter().any(|&(h, n)| h == hex && n == name) { return 6; }
    7
}