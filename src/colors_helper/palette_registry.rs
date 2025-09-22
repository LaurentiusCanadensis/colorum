// Simplified palette registration system
use super::Origin;
use crate::color_types::{HexCode, ColorName};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Trait for color palettes - implement this for your palette
pub trait ColorPalette {
    /// The data for this palette as (hex, name) tuples
    const DATA: &'static [(HexCode, ColorName)];
    /// Display name for this palette
    const NAME: &'static str;
    /// The origin enum variant for this palette
    const ORIGIN: Origin;
}

/// Macro to easily define and register a new color palette
///
/// Usage:
/// ```rust
/// define_palette!(
///     MyColors,           // Struct name
///     Origin::MyColors,   // Origin enum variant (add to Origin enum first)
///     "My Colors",        // Display name
///     [
///         ("#FF0000", "red"),
///         ("#00FF00", "green"),
///         ("#0000FF", "blue"),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! define_palette {
    ($struct_name:ident, $origin:expr, $display_name:literal, [$(($hex:literal, $name:literal)),* $(,)?]) => {
        pub struct $struct_name;

        impl $crate::colors_helper::palette_registry::ColorPalette for $struct_name {
            const DATA: &'static [(rust_colors::color_types::HexCode, rust_colors::color_types::ColorName)] = &[
                $((
                    rust_colors::color_types::HexCode::new($hex),
                    rust_colors::color_types::ColorName::new($name)
                )),*
            ];
            const NAME: &'static str = $display_name;
            const ORIGIN: $crate::colors_helper::Origin = $origin;
        }

        // Export the data directly for easy access
        pub const DATA: &'static [(rust_colors::color_types::HexCode, rust_colors::color_types::ColorName)] = &[
            $((
                rust_colors::color_types::HexCode::new($hex),
                rust_colors::color_types::ColorName::new($name)
            )),*
        ];

        // Auto-register the palette
        inventory::submit! {
            $crate::colors_helper::palette_registry::PaletteRegistration {
                origin: $origin,
                name: $display_name,
                data: || <$struct_name as $crate::colors_helper::palette_registry::ColorPalette>::DATA,
            }
        }
    };
}

/// Registration entry for automatic palette discovery
pub struct PaletteRegistration {
    pub origin: Origin,
    pub name: &'static str,
    pub data: fn() -> &'static [(HexCode, ColorName)],
}

inventory::collect!(PaletteRegistration);

/// Get all registered palettes
pub fn registered_palettes() -> &'static [&'static PaletteRegistration] {
    inventory::iter::<PaletteRegistration>().collect::<Vec<_>>().leak()
}

/// Get palette data by origin (legacy function - returns empty for compatibility)
pub fn palette_data(origin: Origin) -> &'static [(&'static str, &'static str)] {
    // This function is deprecated - use the new type system instead
    &[]
}

/// Get all combined colors from all registered palettes (legacy - returns empty for compatibility)
pub static COMBINED_COLORS_AUTO: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    // This is deprecated - use the new type system instead
    Vec::new()
});

/// Get palette name by origin
pub fn palette_name(origin: Origin) -> &'static str {
    for palette in inventory::iter::<PaletteRegistration>() {
        if palette.origin == origin {
            return palette.name;
        }
    }
    "Unknown"
}

/// Build registry map for all registered palettes (legacy - returns empty for compatibility)
pub static REGISTRY_MAP_AUTO: LazyLock<HashMap<Origin, fn() -> &'static [(&'static str, &'static str)]>> = LazyLock::new(|| {
    // This is deprecated - use the new type system instead
    HashMap::new()
});