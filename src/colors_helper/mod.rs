// src/colors_helper/mod.rs
use core::fmt::{self, Display};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub use once_cell::sync::Lazy;
pub use std::sync::LazyLock;

pub use dropdown_results_for_ui;

pub use best_first_for_ui;

// external color tables

#[path = "../colors/css.rs"]
mod css_colors;
pub use css_colors::COLORS_CSS;

#[path = "../colors/xkcd.rs"]
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

            #[cfg(feature = "github-colors")]
            Origin::GitHub => "github",
        };
        f.write_str(s)
    }
}
