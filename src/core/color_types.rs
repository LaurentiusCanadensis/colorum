/// Entity type for color names - what kind of thing the color represents
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Entity {
    Color,        // Pure color name like "red", "blue"
    Object,       // Physical object like "tomato", "sky"
    Material,     // Material like "copper", "steel"
    Place,        // Geographic location like "persian", "canadian"
    Brand,        // Company or brand name
    Person,       // Person's name
    Abstract,     // Abstract concept
    Chemical,     // Chemical compound or element
    Temperature,  // Temperature-based like kelvin colors
    Other,        // Miscellaneous
}

/// Ordering type for color names - how they should be sorted
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Ordering {
    Name,         // Sort alphabetically by name
    Kelvin,       // Sort by temperature (for temperature-based colors)
    Integer,      // Sort by embedded integer value
    Brightness,   // Sort by color brightness
    Hue,          // Sort by hue value
    Custom(u16),  // Custom ordering with priority value
}

/// Strongly typed color name that supports sorting and comparison
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ColorName {
    name: &'static str,
    entity: Entity,
    origin: crate::colors_helper::Origin,
    ordering: Ordering,
}

impl ColorName {
    /// Create a new ColorName with all fields
    pub const fn new_full(
        name: &'static str,
        entity: Entity,
        origin: crate::colors_helper::Origin,
        ordering: Ordering,
    ) -> Self {
        Self { name, entity, origin, ordering }
    }

    /// Create a new ColorName with just name (for backward compatibility)
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            entity: Entity::Color,
            origin: crate::colors_helper::Origin::All,
            ordering: Ordering::Name,
        }
    }

    /// Get the underlying string
    pub fn as_str(&self) -> &'static str {
        self.name
    }

    /// Get the entity type
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Get the origin
    pub fn origin(&self) -> crate::colors_helper::Origin {
        self.origin
    }

    /// Get the ordering type
    pub fn ordering(&self) -> Ordering {
        self.ordering
    }
}

impl std::fmt::Display for ColorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialOrd for ColorName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary sort by ordering type, secondary by name
        match (&self.ordering, &other.ordering) {
            (Ordering::Name, Ordering::Name) => {
                self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase())
            }
            (Ordering::Custom(a), Ordering::Custom(b)) => {
                a.cmp(b).then_with(|| self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase()))
            }
            (Ordering::Kelvin, Ordering::Kelvin) => {
                // For Kelvin colors, try to extract temperature number from name
                let temp_a = extract_kelvin_temp(self.name).unwrap_or(0);
                let temp_b = extract_kelvin_temp(other.name).unwrap_or(0);
                temp_a.cmp(&temp_b).then_with(|| self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase()))
            }
            (Ordering::Integer, Ordering::Integer) => {
                // Extract integer from name for sorting
                let int_a = extract_integer(self.name).unwrap_or(0);
                let int_b = extract_integer(other.name).unwrap_or(0);
                int_a.cmp(&int_b).then_with(|| self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase()))
            }
            // Different ordering types - sort by enum order first
            _ => {
                self.ordering.cmp(&other.ordering)
                    .then_with(|| self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase()))
            }
        }
    }
}

/// Helper function to extract Kelvin temperature from color name
fn extract_kelvin_temp(name: &str) -> Option<u32> {
    // Look for pattern like "5500K" or "5500k"
    let name_lower = name.to_lowercase();
    if let Some(k_pos) = name_lower.find('k') {
        // Look backwards from 'k' to find the number
        let before_k = &name_lower[..k_pos];
        let num_end = k_pos;
        let mut num_start = k_pos;

        // Find start of number (work backwards from k)
        for (i, ch) in before_k.char_indices().rev() {
            if ch.is_ascii_digit() {
                num_start = i;
            } else if num_start < num_end {
                break;
            }
        }

        if num_start < num_end {
            name_lower[num_start..num_end].parse().ok()
        } else {
            None
        }
    } else {
        None
    }
}

/// Helper function to extract integer from color name
fn extract_integer(name: &str) -> Option<u32> {
    // Extract first number found in the name
    let mut num_chars = String::new();
    for ch in name.chars() {
        if ch.is_ascii_digit() {
            num_chars.push(ch);
        } else if !num_chars.is_empty() {
            break; // Stop at first non-digit after we found digits
        }
    }

    if num_chars.is_empty() {
        None
    } else {
        num_chars.parse().ok()
    }
}

/// Strongly typed hex color code that supports validation and sorting
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct HexCode(&'static str);

impl HexCode {
    /// Create a new HexCode from a static string (assumes valid format)
    pub const fn new(hex: &'static str) -> Self {
        Self(hex)
    }

    /// Get the underlying hex string
    pub fn as_str(&self) -> &'static str {
        self.0
    }

    /// Get RGB components as (r, g, b) tuple
    pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        let hex = self.0.trim_start_matches('#');
        if hex.len() == 6 {
            if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
                if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
                    if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                        return Some((r, g, b));
                    }
                }
            }
        }
        None
    }

    /// Calculate brightness for sorting (0.0 to 1.0)
    pub fn brightness(&self) -> f32 {
        if let Some((r, g, b)) = self.to_rgb() {
            // Standard luminance calculation
            (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0
        } else {
            0.0
        }
    }
}

impl std::fmt::Display for HexCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for HexCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HexCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by brightness first, then by hex string
        self.brightness().partial_cmp(&other.brightness())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.0.cmp(other.0))
    }
}

/// Convenience macro for creating color arrays with typed objects
#[macro_export]
macro_rules! color_array {
    [$(($hex:expr, $name:expr)),* $(,)?] => {
        &[$((HexCode::new($hex), ColorName::new($name))),*]
    };
}

/// Trait to convert between new typed format and legacy string tuple format
pub trait ToLegacyFormat {
    fn to_legacy(&self) -> (&'static str, &'static str);
}

impl ToLegacyFormat for (HexCode, ColorName) {
    fn to_legacy(&self) -> (&'static str, &'static str) {
        (self.0.as_str(), self.1.as_str())
    }
}

/// Helper function to convert a slice of (HexCode, ColorName) to legacy format
pub fn convert_to_legacy_format(colors: &[(HexCode, ColorName)]) -> Vec<(&'static str, &'static str)> {
    colors.iter().map(|color| color.to_legacy()).collect()
}