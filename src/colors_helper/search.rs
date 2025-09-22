// src/colors_helper/search.rs
use super::*;
use std::collections::HashMap;
use crate::color_types::{HexCode, ColorName};
use super::sort;

pub struct ColorEntry {
    pub hex: &'static str,
    pub name: &'static str,
    pub name_lc: String,
}

fn tokenize_lc(s: &str) -> impl Iterator<Item = String> + '_ {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
}

fn build_token_index_for(slice: &[(HexCode, ColorName)]) -> HashMap<String, Box<[usize]>> {
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, (_, name)) in slice.iter().enumerate() {
        for tok in tokenize_lc(name.as_str()) {
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

pub static COLORS_LC: LazyLock<Vec<ColorEntry>> = LazyLock::new(|| {
    catalog::COMBINED_COLORS
        .iter()
        .map(|(hex, name)| ColorEntry {
            hex: hex.as_str(),
            name: name.as_str(),
            name_lc: name.as_str().to_lowercase(),
        })
        .collect()
});

static IDX_ALL: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::All)));
static IDX_CSS: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::Css)));
static IDX_XKCD: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::XKCD)));
static IDX_PANTONE: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::Pantone)));
static IDX_HINDI: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::Hindi)));
static IDX_PERSIAN: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::Persian)));
static IDX_NATIONAL: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::National)));
static IDX_BRANDS: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::Brands)));
static IDX_ITALIANBRANDS: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::ItalianBrands)));

static IDX_METALFLAMES: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::MetalFlames)));
static IDX_KELVINCOLORS: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::KelvinColors)));

#[cfg(feature = "github-colors")]
static IDX_GITHUB: LazyLock<HashMap<String, Box<[usize]>>> =
    LazyLock::new(|| build_token_index_for(catalog::origin_slice(Origin::GitHub)));

fn origin_index(origin: Origin) -> &'static HashMap<String, Box<[usize]>> {
    match origin {
        Origin::All => &IDX_ALL,
        Origin::Css => &IDX_CSS,
        Origin::XKCD => &IDX_XKCD,
        Origin::Pantone => &IDX_PANTONE,
        Origin::Hindi => &IDX_HINDI,
        Origin::Persian => &IDX_PERSIAN,
        Origin::National => &IDX_NATIONAL,
        Origin::Brands => &IDX_BRANDS,
        Origin::ItalianBrands => &IDX_ITALIANBRANDS,
        Origin::MetalFlames => &IDX_METALFLAMES,
        Origin::KelvinColors => &IDX_KELVINCOLORS,

        #[cfg(feature = "github-colors")]
        Origin::GitHub => &IDX_GITHUB,

        // New palette system - these use the auto-generated indices
        Origin::Seasons | Origin::CanadianProvinces => &IDX_ALL, // fallback for now
    }
}

pub fn search_substring(query: &str) -> Vec<(HexCode, ColorName)> {
    let qlc = query.to_lowercase();
    let mut out: Vec<(HexCode, ColorName)> = COLORS_LC
        .iter()
        .filter(|e| e.name_lc.contains(&qlc))
        .map(|e| (HexCode::new(e.hex), ColorName::new(e.name)))
        .collect();
    sort::sort_dropdown_by_origin(&qlc, &mut out);
    if out.len() > super::MAX_RESULTS {
        out.truncate(super::MAX_RESULTS);
    }
    out
}

// … include your existing search_in_origin, search_tokens_any, search_tokens_all,
//    intersect_sorted_slices, union_sorted_slices – unchanged except they call
//    `origin_slice`, `origin_index`, and `sort::sort_dropdown_by_origin` from here.

pub enum TokenMode {
    Any,
    All,
    Substring,
}
// short inputs use substring (prefix) instead of token index
pub const SUBSTRING_THRESHOLD: usize = 3;

pub fn search_in_origin(
    origin: Origin,
    query: &str,
    mode: TokenMode,
) -> Vec<(HexCode, ColorName)> {
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
        if shorts.is_empty() {
            return true;
        }
        let nlc = name.to_lowercase();
        shorts.iter().all(|s| nlc.contains(s))
    }

    // Case A: no full tokens (all tokens short) → pure substring path.
    if full_toks.is_empty() {
        // Fast path for All on short tokens: use cached substring search
        if matches!(origin, Origin::All) {
            return search_substring(q);
        }
        let mut out: Vec<(HexCode, ColorName)> = match mode {
            TokenMode::Any => {
                // ANY of the short tokens
                if short_toks.is_empty() {
                    // Shouldn't happen (q non-empty), but guard anyway
                    slice.to_vec()
                } else {
                    slice
                        .iter()
                        .copied()
                        .filter(|&(_h, n)| {
                            let nlc = n.as_str().to_lowercase();
                            short_toks.iter().any(|s| nlc.contains(s))
                        })
                        .collect()
                }
            }
            _ => {
                // ALL or Substring → require ALL short tokens
                slice
                    .iter()
                    .copied()
                    .filter(|&(_h, n)| name_matches_all_short(n.as_str(), &short_toks))
                    .collect()
            }
        };

        if matches!(origin, Origin::All) {
            sort_dropdown_by_origin(&q.to_lowercase(), &mut out);
        }
        if out.len() > MAX_RESULTS {
            out.truncate(MAX_RESULTS);
        }
        return out;
    }

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
        if matches!(origin, Origin::All) {
            return search_substring(q);
        }
        // Fallback: substring using all tokens (behaves like "Any" if requested)
        let mut out: Vec<(HexCode, ColorName)> = if !short_toks.is_empty() {
            // use short tokens OR semantics for ANY, AND for ALL/Substring
            match mode {
                TokenMode::Any => slice
                    .iter()
                    .copied()
                    .filter(|&(_h, n)| {
                        let nlc = n.as_str().to_lowercase();
                        short_toks.iter().any(|s| nlc.contains(s))
                    })
                    .collect(),
                _ => slice
                    .iter()
                    .copied()
                    .filter(|&(_h, n)| name_matches_all_short(n.as_str(), &short_toks))
                    .collect(),
            }
        } else {
            // No short tokens either: substring on the whole query (rare)
            let qlc = q.to_lowercase();
            slice
                .iter()
                .copied()
                .filter(|&(_h, n)| n.as_str().to_lowercase().contains(&qlc))
                .collect()
        };

        if matches!(origin, Origin::All) {
            sort_dropdown_by_origin(&q.to_lowercase(), &mut out);
        }
        if out.len() > MAX_RESULTS {
            out.truncate(MAX_RESULTS);
        }
        return out;
    }

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
        return slice
            .iter()
            .copied()
            .filter(|&(_h, n)| n.as_str().to_lowercase().contains(&qlc))
            .collect();
    }

    // Apply short-token substring filter to the candidate set (AND semantics for ALL/Substring; OR for ANY)
    let mut out: Vec<(HexCode, ColorName)> = Vec::with_capacity(current.len());
    match mode {
        TokenMode::Any => {
            if short_toks.is_empty() {
                // No short filters → return all candidates
                for i in current {
                    out.push(slice[i]);
                }
            } else {
                for i in current {
                    let name = slice[i].1;
                    let nlc = name.as_str().to_lowercase();
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
                if name_matches_all_short(name.as_str(), &short_toks) {
                    out.push(slice[i]);
                }
            }
        }
    }

    if matches!(origin, Origin::All) {
        sort_dropdown_by_origin(&q.to_lowercase(), &mut out);
    }
    if out.len() > MAX_RESULTS {
        out.truncate(MAX_RESULTS);
    }
    return out;
}

use std::sync::LazyLock;

// Build once over the full combined list.
pub static COLORS_BY_NAME: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    catalog::COMBINED_COLORS
        .iter()
        .map(|(hex, name)| (name.as_str(), hex.as_str()))
        .collect()
});

pub static COLORS_BY_NAME_LC: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (hex, name) in catalog::COMBINED_COLORS.as_slice() {
        m.insert(name.as_str().to_lowercase(), hex.as_str());
    }
    m
});

// Merge helpers
fn intersect_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len().min(b.len()));
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}
fn union_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    if i < a.len() {
        out.extend_from_slice(&a[i..]);
    }
    if j < b.len() {
        out.extend_from_slice(&b[j..]);
    }
    out
}

// tune these
pub const HEAVY_MIN_QUERY: usize = 1; // require 1+ chars for heavy sets (so 2-letter inputs show)
pub const MAX_RESULTS: usize = 4000; // cap the dropdown size
#[inline]
pub fn is_heavy_origin(origin: Origin) -> bool {
    match origin {
        Origin::All => true,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => true,
        _ => false,
    }
}

// Token -> sorted, deduped posting list of indices into COMBINED_COLORS
pub static NAME_TOKEN_INDEX: LazyLock<HashMap<String, Box<[usize]>>> = LazyLock::new(|| {
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, (_, name)) in catalog::COMBINED_COLORS.iter().enumerate() {
        for tok in tokenize_lc(name.as_str()) {
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

// Global token searches over COMBINED_COLORS
pub fn search_tokens_any(query: &str) -> Vec<(HexCode, ColorName)> {
    let mut postings: Vec<&[usize]> = Vec::new();
    for tok in tokenize_lc(query) {
        if let Some(list) = NAME_TOKEN_INDEX.get(&tok) {
            postings.push(list);
        }
    }
    if postings.is_empty() {
        return Vec::new();
    }
    postings.sort_by_key(|p| p.len());
    let mut result: Vec<usize> = postings[0].to_vec();
    for p in postings.iter().skip(1) {
        result = union_sorted_slices(&result, p);
    }
    let mut out: Vec<(HexCode, ColorName)> =
        result.into_iter().map(|i| catalog::COMBINED_COLORS[i]).collect();

    // Prefer exact name matches and CSS first for global (All) searches
    let qlc = query.to_lowercase();
    sort_dropdown_by_origin(&qlc, &mut out);
    if out.len() > MAX_RESULTS {
        out.truncate(MAX_RESULTS);
    }
    out
}
pub fn search_tokens_all(query: &str) -> Vec<(HexCode, ColorName)> {
    let mut lists: Vec<&[usize]> = {
        let mut tmp = Vec::new();
        for tok in tokenize_lc(query) {
            match NAME_TOKEN_INDEX.get(&tok) {
                Some(list) => tmp.push(list.as_ref()),
                None => return Vec::new(),
            }
        }
        if tmp.is_empty() {
            return Vec::new();
        }
        tmp
    };
    lists.sort_by_key(|p| p.len());
    let mut current: Vec<usize> = lists[0].to_vec();
    for p in lists.iter().skip(1) {
        if current.is_empty() {
            break;
        }
        current = intersect_sorted_slices(&current, p);
    }
    let mut out: Vec<(HexCode, ColorName)> =
        current.into_iter().map(|i| catalog::COMBINED_COLORS[i]).collect();

    let qlc = query.to_lowercase();
    sort_dropdown_by_origin(&qlc, &mut out);
    if out.len() > MAX_RESULTS {
        out.truncate(MAX_RESULTS);
    }
    out
}
