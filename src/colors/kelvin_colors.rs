use crate::color_types::{HexCode, ColorName, Entity, Ordering};
use crate::colors_helper::Origin;
use std::sync::LazyLock;

/// Kelvin color temperature palette with structured types, sorted by temperature.
/// Each entry is a tuple of (HexCode, ColorName) with sortable components.
const KELVIN_COLORS_UNORDERED: &[(HexCode, ColorName)] = &[
    //const KELVIN_PALETTE: &[(&str, &str)] = &[
    (HexCode::new("#FFFFFF"), ColorName::new_full("Skylight 20000K Very Pale Blue-White", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#050573"), ColorName::new_full("Skylight 19500K Deep Navy Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#0D0DD2"), ColorName::new_full("Skylight 18500K Royal Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#1313FF"), ColorName::new_full("Skylight 16500K Pure Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#4242FC"), ColorName::new_full("Shade 8000K Bright Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#6B6BFB"), ColorName::new_full("Shade→Monitor 7500K Cornflower Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#8989FC"), ColorName::new_full("Monitors 6500K Lavender Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#C4C4FD"), ColorName::new_full("Noon Sun 5600K Pale Periwinkle", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FEFDF0"), ColorName::new_full("Fluorescent 4300K Soft White", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FBF695"), ColorName::new_full("Tungsten Halogen 3200K Light Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F7EE35"), ColorName::new_full("Halogen Transition 3500K Lemon Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F8E213"), ColorName::new_full("Incandescent 2800K Golden Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FE5C0D"), ColorName::new_full("Candle 1800K Orange Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F8170E"), ColorName::new_full("Candle→Embers 1500K Bright Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#BA0709"), ColorName::new_full("Embers 800K Dark Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#600708"), ColorName::new_full("Deep Embers 700K Blackish Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    // Gradient fill-ins to keep smooth steps
    (HexCode::new("#1212FF"), ColorName::new_full("Skylight 16473K Bright Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#2527FF"), ColorName::new_full("Skylight 16081K Bright Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#3C3DFE"), ColorName::new_full("Skylight 15689K Bright Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#5F5FFA"), ColorName::new_full("Skylight 14906K Bright Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#6969FF"), ColorName::new_full("Skylight 14514K Light Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#7777FE"), ColorName::new_full("Skylight 14122K Light Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#8B8AF9"), ColorName::new_full("Skylight 13730K Light Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#9696FB"), ColorName::new_full("Skylight 13338K Light Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#A4A3FE"), ColorName::new_full("Skylight 12946K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#ACADFC"), ColorName::new_full("Skylight 12555K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#B7B7FD"), ColorName::new_full("Skylight 12163K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#C1C0FE"), ColorName::new_full("Skylight 11771K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#CFD1FD"), ColorName::new_full("Skylight 11379K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#E2E2FE"), ColorName::new_full("Skylight 10987K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F1F0FF"), ColorName::new_full("Skylight 10595K Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FCFCFE"), ColorName::new_full("Daylight 10204K Very Pale Blue", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FEFEEF"), ColorName::new_full("Daylight 9812K Very Pale Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FEFACB"), ColorName::new_full("Daylight 9420K Pale Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FCF6A0"), ColorName::new_full("Daylight 9028K Pale Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FAF26C"), ColorName::new_full("Daylight 8636K Light Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F9F04D"), ColorName::new_full("Daylight 8244K Light Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#F9D51E"), ColorName::new_full("Noon Sun 6285K Golden Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FDCB0F"), ColorName::new_full("Noon Sun 5893K Golden Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FDC608"), ColorName::new_full("Fluorescent 5502K Warm Yellow", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FEB707"), ColorName::new_full("Fluorescent 5110K Yellow-Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FE9C09"), ColorName::new_full("Fluorescent 4718K Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FD801A"), ColorName::new_full("Halogen 4326K Bright Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FD6B01"), ColorName::new_full("Halogen 3934K Orange-Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FF4200"), ColorName::new_full("Halogen 3542K Bright Red-Orange", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FF1C02"), ColorName::new_full("Incandescent 3151K Bright Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#FD0600"), ColorName::new_full("Incandescent 2759K Bright Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#EA0000"), ColorName::new_full("Candle 2367K Bright Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#CB0000"), ColorName::new_full("Candle 1975K Vivid Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#A10200"), ColorName::new_full("Candle 1583K Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#760001"), ColorName::new_full("Embers 1191K Deep Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
    (HexCode::new("#320100"), ColorName::new_full("Embers 800K Dark Red", Entity::Temperature, Origin::KelvinColors, Ordering::Kelvin)),
];

/// Kelvin colors automatically sorted by temperature (highest to lowest)
pub static KELVIN_COLORS: LazyLock<Vec<(HexCode, ColorName)>> = LazyLock::new(|| {
    let mut colors = KELVIN_COLORS_UNORDERED.to_vec();
    colors.sort_by(|a, b| {
        // Extract Kelvin temperatures for comparison
        let temp_a = extract_kelvin_temp(a.1.as_str());
        let temp_b = extract_kelvin_temp(b.1.as_str());

        match (temp_a, temp_b) {
            (Some(ta), Some(tb)) => {
                // Both have temperatures - sort by temperature descending (highest first)
                tb.cmp(&ta).then_with(|| a.1.as_str().cmp(b.1.as_str()))
            }
            (Some(_), None) => std::cmp::Ordering::Less, // Colors with temps come first
            (None, Some(_)) => std::cmp::Ordering::Greater, // Colors without temps come last
            (None, None) => a.1.as_str().cmp(b.1.as_str()), // No temps - sort alphabetically
        }
    });
    colors
});

/// Helper function to extract Kelvin temperature from color name
fn extract_kelvin_temp(name: &str) -> Option<u32> {
    // Look for pattern like "20000K" or "5500k" - find the largest number followed by 'k'

    let name_lower = name.to_lowercase();
    let mut max_temp: Option<u32> = None;

    // Find all 'k' positions and check each one
    for k_pos in name_lower.match_indices('k').map(|(i, _)| i) {
        let before_k = &name_lower[..k_pos];
        let mut num_start = k_pos;

        // Find start of number (work backwards from k)
        for (i, ch) in before_k.char_indices().rev() {
            if ch.is_ascii_digit() {
                num_start = i;
            } else if num_start < k_pos {
                break;
            }
        }

        if num_start < k_pos {
            if let Ok(temp) = name_lower[num_start..k_pos].parse::<u32>() {
                max_temp = Some(max_temp.unwrap_or(0).max(temp));
            }
        }
    }

    max_temp
}
