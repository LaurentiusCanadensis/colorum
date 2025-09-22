// src/colors_helper/catalog.rs
use super::*;
use std::collections::HashMap;
use crate::colors::kelvin_colors::KELVIN_COLORS;

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
        Origin::ItalianBrands => COLORS_ITALIANBRANDS,
        Origin::MetalFlames => COLORS_METALS_FLAME,
        Origin::KelvinColors => KELVIN_COLORS,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => COLORS_GITHUB,
    }
}

pub static REGISTRY_MAP: LazyLock<
    HashMap<Origin, fn() -> &'static [(&'static str, &'static str)]>,
> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(REGISTRY.len());
    for c in REGISTRY {
        m.insert(c.origin, c.data); // store the function pointer
    }
    m
});

pub fn colors_for(origin: Origin) -> &'static [(&'static str, &'static str)] {
    if let Origin::All = origin {
        return COMBINED_COLORS.as_slice();
    }
    REGISTRY_MAP.get(&origin).map(|f| f()).unwrap_or(&[])
}

pub static COMBINED_COLORS: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    let mut v = Vec::new();
    for c in REGISTRY {
        v.extend_from_slice((c.data)());
    }
    v
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

pub static ORIGIN_NAMES_ITALIANBRANDS: LazyLock<Box<[&'static str]>> =
    LazyLock::new(|| build_sorted_names(Origin::ItalianBrands));
// Origin -> &'static [&'static str]
static NAMES_CACHE: LazyLock<std::sync::Mutex<HashMap<Origin, &'static [&'static str]>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

pub fn origin_names(origin: Origin) -> &'static [&'static str] {
    if let Origin::All = origin {
        // One combined cache for All
        static ALL_NAMES: LazyLock<&'static [&'static str]> = LazyLock::new(|| {
            let names: Vec<&'static str> = COMBINED_COLORS.iter().map(|&(_, n)| n).collect();
            Box::leak(names.into_boxed_slice())
        });
        return *ALL_NAMES;
    }

    let mut guard = NAMES_CACHE.lock().expect("poisoned NAMES_CACHE");
    if let Some(&cached) = guard.get(&origin) {
        return cached;
    }
    let names_vec: Vec<&'static str> = colors_for(origin).iter().map(|&(_, n)| n).collect();
    let leaked: &'static [&'static str] = Box::leak(names_vec.into_boxed_slice());
    guard.insert(origin, leaked);
    leaked
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
#[cfg(feature = "github-colors")]
pub static ORIGIN_RANK_GITHUB: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_GITHUB));
pub static ORIGIN_RANK_BRANDS: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_BRANDS));

pub static ORIGIN_RANK_ITALIANBRANDS: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| build_rank_map(&ORIGIN_NAMES_ITALIANBRANDS));

// Origin -> &'static HashMap<&'static str, usize>
static RANK_CACHE: LazyLock<
    std::sync::Mutex<HashMap<Origin, &'static HashMap<&'static str, usize>>>,
> = LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

pub fn origin_rank(origin: Origin) -> &'static HashMap<&'static str, usize> {
    let mut guard = RANK_CACHE.lock().expect("poisoned RANK_CACHE");
    if let Some(&cached) = guard.get(&origin) {
        return cached;
    }

    let mut map: HashMap<&'static str, usize> = HashMap::new();
    for (i, &(_, name)) in colors_for(origin).iter().enumerate() {
        map.insert(name, i);
    }
    let leaked: &'static HashMap<&'static str, usize> = Box::leak(Box::new(map));
    guard.insert(origin, leaked);
    leaked
}
