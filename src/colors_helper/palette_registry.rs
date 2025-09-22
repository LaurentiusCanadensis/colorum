// Simplified palette registration system
use super::Origin;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Trait for color palettes - implement this for your palette
pub trait ColorPalette {
    /// The data for this palette as (hex, name) tuples
    const DATA: &'static [(&'static str, &'static str)];
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
            const DATA: &'static [(&'static str, &'static str)] = &[
                $(($hex, $name)),*
            ];
            const NAME: &'static str = $display_name;
            const ORIGIN: $crate::colors_helper::Origin = $origin;
        }

        // Export the data directly for easy access
        pub const DATA: &'static [(&'static str, &'static str)] = &[
            $(($hex, $name)),*
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
    pub data: fn() -> &'static [(&'static str, &'static str)],
}

inventory::collect!(PaletteRegistration);

/// Get all registered palettes
pub fn registered_palettes() -> &'static [&'static PaletteRegistration] {
    inventory::iter::<PaletteRegistration>().collect::<Vec<_>>().leak()
}

/// Get palette data by origin
pub fn palette_data(origin: Origin) -> &'static [(&'static str, &'static str)] {
    for palette in inventory::iter::<PaletteRegistration>() {
        if palette.origin == origin {
            return (palette.data)();
        }
    }
    &[]
}

/// Get all combined colors from all registered palettes
pub static COMBINED_COLORS_AUTO: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    let mut colors = Vec::new();
    for palette in inventory::iter::<PaletteRegistration>() {
        colors.extend_from_slice((palette.data)());
    }
    colors.sort_by(|a, b| a.1.cmp(b.1)); // Sort by name
    colors
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

/// Build registry map for all registered palettes
pub static REGISTRY_MAP_AUTO: LazyLock<HashMap<Origin, fn() -> &'static [(&'static str, &'static str)]>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for palette in inventory::iter::<PaletteRegistration>() {
        map.insert(palette.origin, palette.data);
    }
    map
});