// src/colors_helper/mod.rs
use core::fmt::{self, Display};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub use once_cell::sync::Lazy;
pub use std::sync::LazyLock;

pub use dropdown_results_for_ui;

pub use best_first_for_ui;

// external color tables

#[path = "../colors/css_colors.rs"]
mod css_colors;
pub use css_colors::COLORS_CSS;

#[path = "../colors/metals_flame_colors.rs"]
mod metals_flame_colors;
pub use metals_flame_colors::COLORS_METALS_FLAME;


#[path = "../colors/kelvin_colors.rs"]
mod kelvin_colors;
pub use kelvin_colors::KELVIN_COLORS;
#[path = "../colors/xkcd_colors.rs"]
mod xkcd_colors;
pub use xkcd_colors::COLORS_XKCD;

#[path = "../colors/pantone_colors.rs"]
mod pantone_colors;
pub use pantone_colors::COLORS_PANTONE;

#[path = "../colors/hindi_colors.rs"]
mod hindi_colors;
pub use hindi_colors::COLORS_HINDI;

#[path = "../colors/national_colors.rs"]
mod national_colors;
pub use national_colors::COLORS_NATIONAL;

#[path = "../colors/persian_colors.rs"]
pub mod persian_colors;
pub use persian_colors::COLORS_PERSIAN;

#[path = "../colors/italianbrand_colors.rs"]
pub mod italianbrand_colors;
pub use italianbrand_colors::COLORS_ITALIANBRANDS;
#[path = "../colors/brand_colors.rs"]
pub mod brand_colors;
pub use brand_colors::COLORS_BRANDS;

#[cfg(feature = "github-colors")]
#[path = "../colors/github_colors.rs"]
mod github_colors;
#[cfg(feature = "github-colors")]
pub use github_colors::COLORS_GITHUB;

// share hex/rgb utils
pub use crate::hex::*;
pub use crate::rgb::*;

// ==== submodules ====
mod catalog;
pub use catalog::*;
mod sort;
pub use sort::*;
mod search;
pub use search::*;
mod ui;
pub use ui::*;

// ===== origin facade (kept public) =====
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin {
    #[default]
    All,
    Css,
    Hindi,
    Persian,
    Pantone,
    XKCD,
    ItalianBrands,
    National,
    Brands,
    MetalFlames,
    KelvinColors,
    #[cfg(feature = "github-colors")]
    GitHub,
}
impl Hash for Origin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
impl Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Origin::All => "all",
            Origin::Css => "Css",
            Origin::Hindi => "Hindi",
            Origin::Persian => "Persian",
            Origin::Pantone => "Pantone",
            Origin::XKCD => "Xkcd",
            Origin::National => "National",
            Origin::Brands => "Brands",
            Origin::ItalianBrands => "Italian Brands",
            Origin::MetalFlames => "Metal Flames",
            Origin::KelvinColors => "Kelvin Colors",

            #[cfg(feature = "github-colors")]
            Origin::GitHub => "github",
        };
        f.write_str(s)
    }
}

pub fn colors_for(origin: Origin) -> &'static [(&'static str, &'static str)] {
    if let Origin::All = origin {
        return COMBINED_COLORS.as_slice();
    }
    REGISTRY_MAP
        .get(&origin)
        .map(|f| f()) // call the fn pointer
        .unwrap_or(&[])
}

pub struct ColorCatalog {
    pub name: &'static str,
    pub origin: Origin,
    pub data: fn() -> &'static [(&'static str, &'static str)],
}

pub static REGISTRY: &[ColorCatalog] = &[
    ColorCatalog {
        name: "CSS",
        origin: Origin::Css,
        data: data_css,
    },
    ColorCatalog {
        name: "XKCD",
        origin: Origin::XKCD,
        data: data_xkcd,
    },
    ColorCatalog {
        name: "Pantone",
        origin: Origin::Pantone,
        data: data_pantone,
    },
    ColorCatalog {
        name: "Hindi",
        origin: Origin::Hindi,
        data: data_hindi,
    },
    ColorCatalog {
        name: "Persian",
        origin: Origin::Persian,
        data: data_persian,
    },
    ColorCatalog {
        name: "National",
        origin: Origin::National,
        data: data_national,
    }, // <- fixed
    ColorCatalog {
        name: "Brands",
        origin: Origin::Brands,
        data: data_brands,
    },
    ColorCatalog {
        name: "Italian Brands",
        origin: Origin::ItalianBrands,
        data: data_italian_brands,
    },
    ColorCatalog {
        name: "Metal Flames",
        origin: Origin::MetalFlames,
        data: data_metal_flames,
    },
    ColorCatalog {
        name: "Kelvin Colors",
        origin: Origin::KelvinColors,
        data: data_kelvin_colors,
    },
    #[cfg(feature = "github-colors")]
    ColorCatalog {
        name: "GitHub",
        origin: Origin::GitHub,
        data: data_github,
    },
];
fn data_national() -> &'static [(&'static str, &'static str)] {
    COLORS_NATIONAL.as_slice() // this runs at runtime, not in a const context
}
fn data_css() -> &'static [(&'static str, &'static str)] {
    COLORS_CSS
}
fn data_xkcd() -> &'static [(&'static str, &'static str)] {
    COLORS_XKCD
}
fn data_pantone() -> &'static [(&'static str, &'static str)] {
    COLORS_PANTONE
}
fn data_hindi() -> &'static [(&'static str, &'static str)] {
    COLORS_HINDI
}
fn data_persian() -> &'static [(&'static str, &'static str)] {
    COLORS_PERSIAN
}
fn data_brands() -> &'static [(&'static str, &'static str)] {
    COLORS_BRANDS
}
fn data_italian_brands() -> &'static [(&'static str, &'static str)] {
    COLORS_ITALIANBRANDS
}
fn data_kelvin_colors() -> &'static [(&'static str, &'static str)] {
    KELVIN_COLORS
}fn data_metal_flames() -> &'static [(&'static str, &'static str)] {
    COLORS_METALS_FLAME
}
#[cfg(feature = "github-colors")]
fn data_github() -> &'static [(&'static str, &'static str)] {
    COLORS_GITHUB
}

pub static COMBINED_COLORS: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    let mut v = Vec::new();
    for c in REGISTRY {
        v.extend_from_slice((c.data)()); // <- call the function
    }
    v
});

// BEFORE (wrong value type)
// pub static REGISTRY_MAP: LazyLock<HashMap<Origin, &'static [(&'static str, &'static str)]>> = ...

// AFTER (store fn pointers)
pub static REGISTRY_MAP: LazyLock<
    HashMap<Origin, fn() -> &'static [(&'static str, &'static str)]>,
> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(REGISTRY.len());
    for c in REGISTRY {
        m.insert(c.origin, c.data); // store the function pointer
    }
    m
});
