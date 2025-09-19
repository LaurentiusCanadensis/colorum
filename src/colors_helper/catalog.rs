// src/colors_helper/catalog.rs
use super::*;
use std::collections::HashMap;

pub enum ColorsFor {
    Slice(&'static [(&'static str, &'static str)]),
    Owned(Vec<(&'static str, &'static str)>),
}
impl ColorsFor {
    #[inline]
    pub fn as_slice(&self) -> &[(&'static str, &'static str)] {
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
    pub fn iter(&self) -> std::slice::Iter<'_, (&'static str, &'static str)> {
        self.as_slice().iter()
    }
}

// Per-origin slice
pub fn origin_slice(origin: Origin) -> &'static [(&'static str, &'static str)] {
    match origin {
        Origin::All => COMBINED_COLORS.as_slice(),
        Origin::Css => COLORS_CSS,
        Origin::XKCD => COLORS_XKCD,
        Origin::Pantone => COLORS_PANTONE,
        Origin::Hindi => COLORS_HINDI,
        Origin::Persian => COLORS_PERSIAN,
        Origin::National => COLORS_NATIONAL.as_slice(),
        Origin::Brands => COLORS_BRANDS,

        #[cfg(feature = "github-colors")]
        Origin::GitHub => COLORS_GITHUB,
    }
}

pub const COLORS_ALL_FALLBACK: &[(&str, &str)] = &[];

pub fn colors_for(origin: Origin) -> ColorsFor {
    match origin {
        Origin::Css => ColorsFor::Slice(COLORS_CSS),
        Origin::XKCD => ColorsFor::Slice(COLORS_XKCD),
        Origin::Pantone => ColorsFor::Slice(COLORS_PANTONE),
        Origin::Hindi => ColorsFor::Slice(COLORS_HINDI),
        Origin::Persian => ColorsFor::Slice(COLORS_PERSIAN),
        Origin::Brands => ColorsFor::Slice(COLORS_BRANDS),

        Origin::National => ColorsFor::Slice(COLORS_NATIONAL.as_slice()),
        #[cfg(feature = "github-colors")]
        Origin::GitHub => ColorsFor::Slice(COLORS_GITHUB),
        Origin::All => {
            if !COLORS_ALL_FALLBACK.is_empty() {
                ColorsFor::Slice(COLORS_ALL_FALLBACK)
            } else {
                let mut v = Vec::new();
                v.extend_from_slice(COLORS_CSS);
                v.extend_from_slice(COLORS_XKCD);
                v.extend_from_slice(COLORS_PERSIAN);
                v.extend_from_slice(COLORS_PANTONE);
                v.extend_from_slice(COLORS_HINDI);
                v.extend_from_slice(COLORS_BRANDS);

                v.extend_from_slice(COLORS_NATIONAL.as_slice());
                #[cfg(feature = "github-colors")]
                v.extend_from_slice(COLORS_GITHUB);
                ColorsFor::Owned(v)
            }
        }
    }
}

pub static COMBINED_COLORS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
    let base = COLORS_CSS
        .iter()
        .copied()
        .chain(COLORS_XKCD.iter().copied())
        .chain(COLORS_PERSIAN.iter().copied())
        .chain(COLORS_PANTONE.iter().copied())
        .chain(COLORS_HINDI.iter().copied())
        .chain(COLORS_BRANDS.iter().copied())

        .chain(COLORS_NATIONAL.iter().copied());
    #[cfg(feature = "github-colors")]
    let it = base.chain(COLORS_GITHUB.iter().copied());
    #[cfg(not(feature = "github-colors"))]
    let it = base;
    it.collect()
});

// Name lookups
pub static COLORS_BY_NAME: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    COMBINED_COLORS
        .iter()
        .map(|(hex, name)| (*name, *hex))
        .collect()
});
pub static COLORS_BY_NAME_LC: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (hex, name) in COMBINED_COLORS.as_slice() {
        m.insert(name.to_lowercase(), *hex);
    }
    m
});
pub fn lookup_by_name(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME.get(name).copied()
}
pub fn lookup_by_name_ci(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME_LC.get(&name.to_lowercase()).copied()
}

// Sorted names & ranks
fn build_sorted_names(origin: Origin) -> Box<[&'static str]> {
    let slice = colors_for(origin);
    let mut names: Vec<&'static str> = slice.iter().map(|&(_h, n)| n).collect();
    if matches!(origin, Origin::All) {
        names.sort_unstable_by(|a, b| {
            let ha = lookup_by_name(a).unwrap_or("#000000");
            let hb = lookup_by_name(b).unwrap_or("#000000");
            super::sort::origin_group_priority(ha, a)
                .cmp(&super::sort::origin_group_priority(hb, b))
                .then_with(|| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
        });
    } else {
        names.sort_unstable_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    }
    names.into_boxed_slice()
}
fn build_rank_map(names: &[&'static str]) -> HashMap<&'static str, usize> {
    let mut m = HashMap::with_capacity(names.len());
    for (i, &n) in names.iter().enumerate() {
        m.insert(n, i);
    }
    m
}
pub static ORIGIN_NAMES_ALL: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::All));
pub static ORIGIN_NAMES_CSS: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Css));
pub static ORIGIN_NAMES_XKCD: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::XKCD));
pub static ORIGIN_NAMES_PANTONE: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Pantone));
pub static ORIGIN_NAMES_HINDI: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Hindi));
pub static ORIGIN_NAMES_PERSIAN: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Persian));
pub static ORIGIN_NAMES_NATIONAL: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::National));
#[cfg(feature = "github-colors")]
pub static ORIGIN_NAMES_GITHUB: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::GitHub));
pub static ORIGIN_NAMES_BRANDS: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::Brands));


pub fn origin_names(origin: Origin) -> &'static [&'static str] {
    match origin {
        Origin::All => &ORIGIN_NAMES_ALL,
        Origin::Css => &ORIGIN_NAMES_CSS,
        Origin::XKCD => &ORIGIN_NAMES_XKCD,
        Origin::Pantone => &ORIGIN_NAMES_PANTONE,
        Origin::Hindi => &ORIGIN_NAMES_HINDI,
        Origin::Persian => &ORIGIN_NAMES_PERSIAN,
        Origin::National => &ORIGIN_NAMES_NATIONAL,
        Origin::Brands => &ORIGIN_NAMES_BRANDS,

        #[cfg(feature = "github-colors")]
        Origin::GitHub => &ORIGIN_NAMES_GITHUB,
    }
}

pub static ORIGIN_RANK_ALL: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_ALL));
pub static ORIGIN_RANK_CSS: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_CSS));
pub static ORIGIN_RANK_XKCD: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_XKCD));
pub static ORIGIN_RANK_PANTONE: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_PANTONE));
pub static ORIGIN_RANK_HINDI: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_HINDI));
pub static ORIGIN_RANK_PERSIAN: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_PERSIAN));
pub static ORIGIN_RANK_NATIONAL: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_NATIONAL));
pub static ORIGIN_RANK_GITHUB: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_GITHUB));
pub static ORIGIN_RANK_BRANDS: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_BRANDS));


pub fn origin_rank(origin: Origin) -> &'static HashMap<&'static str, usize> {
    match origin {
        Origin::All => &ORIGIN_RANK_ALL,
        Origin::Css => &ORIGIN_RANK_CSS,
        Origin::XKCD => &ORIGIN_RANK_XKCD,
        Origin::Pantone => &ORIGIN_RANK_PANTONE,
        Origin::Hindi => &ORIGIN_RANK_HINDI,
        Origin::Persian => &ORIGIN_RANK_PERSIAN,
        Origin::National => &ORIGIN_RANK_NATIONAL,
        Origin::Brands => &ORIGIN_RANK_BRANDS,

        #[cfg(feature = "github-colors")]
        Origin::GitHub => &ORIGIN_RANK_GITHUB,
    }
}
