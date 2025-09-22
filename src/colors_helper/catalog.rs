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
        // New palette system - direct access to ensure data is available
        Origin::Seasons => crate::colors::seasons::DATA,
        Origin::CanadianProvinces => crate::colors::canadian_provinces::DATA,
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
// Consolidated origin name caching
pub static ORIGIN_NAMES_CACHE: LazyLock<HashMap<Origin, Box<[&'static str]>>> = LazyLock::new(|| {
    let mut cache = HashMap::new();

    // Build for all origins
    for &origin in &[
        Origin::All, Origin::Css, Origin::XKCD, Origin::Pantone,
        Origin::Hindi, Origin::Persian, Origin::National, Origin::Brands,
        Origin::ItalianBrands, Origin::MetalFlames, Origin::KelvinColors,
        #[cfg(feature = "github-colors")]
        Origin::GitHub,
    ] {
        cache.insert(origin, build_sorted_names(origin));
    }

    cache
});
// Consolidated origin rank caching
pub static ORIGIN_RANK_CACHE: LazyLock<HashMap<Origin, HashMap<&'static str, usize>>> = LazyLock::new(|| {
    let mut cache = HashMap::new();

    // Build ranks for all origins using the name cache
    for (&origin, names) in ORIGIN_NAMES_CACHE.iter() {
        cache.insert(origin, build_rank_map(names));
    }

    cache
});

pub fn origin_names(origin: Origin) -> &'static [&'static str] {
    ORIGIN_NAMES_CACHE.get(&origin)
        .map(|names| names.as_ref())
        .unwrap_or(&[])
}

pub fn origin_rank(origin: Origin) -> &'static HashMap<&'static str, usize> {
    ORIGIN_RANK_CACHE.get(&origin)
        .unwrap_or(&EMPTY_RANK_MAP)
}

// Empty fallback to avoid Option handling
static EMPTY_RANK_MAP: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| HashMap::new());
