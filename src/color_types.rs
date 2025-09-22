/// Strongly typed color name that supports sorting and comparison
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ColorName(&'static str);

impl ColorName {
    /// Create a new ColorName from a static string
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Get the underlying string
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for ColorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for ColorName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_ascii_lowercase().cmp(&other.0.to_ascii_lowercase())
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