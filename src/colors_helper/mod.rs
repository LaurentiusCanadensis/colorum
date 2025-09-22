// src/colors_helper/mod.rs
use core::fmt::{self, Display};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::color_types::{HexCode, ColorName, convert_to_legacy_format};

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
pub mod palette_registry;
pub use palette_registry::*;

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
    Seasons,
    CanadianProvinces,
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
            Origin::Seasons => "Seasons",
            Origin::CanadianProvinces => "Canadian Provinces",
        };
        f.write_str(s)
    }
}

pub fn colors_for(origin: Origin) -> &'static [(HexCode, ColorName)] {
    // Use the implementation from catalog.rs
    catalog::colors_for(origin)
}

pub struct ColorCatalog {
    pub name: &'static str,
    pub origin: Origin,
    pub data: fn() -> &'static [(HexCode, ColorName)],
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
    // New simplified palette system
    ColorCatalog {
        name: "Seasons",
        origin: Origin::Seasons,
        data: data_seasons,
    },
    ColorCatalog {
        name: "Canadian Provinces",
        origin: Origin::CanadianProvinces,
        data: data_canadian_provinces,
    },
];
fn data_national() -> &'static [(HexCode, ColorName)] {
    COLORS_NATIONAL.as_slice() // this runs at runtime, not in a const context
}
fn data_css() -> &'static [(HexCode, ColorName)] {
    COLORS_CSS
}
fn data_xkcd() -> &'static [(HexCode, ColorName)] {
    COLORS_XKCD
}
fn data_pantone() -> &'static [(HexCode, ColorName)] {
    COLORS_PANTONE
}
fn data_hindi() -> &'static [(HexCode, ColorName)] {
    COLORS_HINDI
}
fn data_persian() -> &'static [(HexCode, ColorName)] {
    COLORS_PERSIAN
}
fn data_brands() -> &'static [(HexCode, ColorName)] {
    COLORS_BRANDS
}
fn data_italian_brands() -> &'static [(HexCode, ColorName)] {
    COLORS_ITALIANBRANDS.as_slice()
}
fn data_kelvin_colors() -> &'static [(HexCode, ColorName)] {
    KELVIN_COLORS.as_slice()
}
fn data_metal_flames() -> &'static [(HexCode, ColorName)] {
    COLORS_METALS_FLAME
}
#[cfg(feature = "github-colors")]
fn data_github() -> &'static [(HexCode, ColorName)] {
    COLORS_GITHUB
}

// New palette data functions
fn data_seasons() -> &'static [(HexCode, ColorName)] {
    crate::colors::seasons::DATA
}

fn data_canadian_provinces() -> &'static [(HexCode, ColorName)] {
    crate::colors::canadian_provinces::DATA
}

// COMBINED_COLORS moved to catalog.rs with new types

// BEFORE (wrong value type)
// pub static REGISTRY_MAP: LazyLock<HashMap<Origin, &'static [(&'static str, &'static str)]>> = ...

// AFTER (store fn pointers)
pub static REGISTRY_MAP: LazyLock<
    HashMap<Origin, fn() -> &'static [(HexCode, ColorName)]>,
> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(REGISTRY.len());
    for c in REGISTRY {
        m.insert(c.origin, c.data); // store the function pointer
    }
    m
});
