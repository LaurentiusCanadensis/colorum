/// Simple RGB struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Convert normalized "#RRGGBB" to `Rgb`.
pub fn hex_to_rgb(norm: &str) -> Option<Rgb> {
    if norm.len() != 7 || !norm.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&norm[1..3], 16).ok()?;
    let g = u8::from_str_radix(&norm[3..5], 16).ok()?;
    let b = u8::from_str_radix(&norm[5..7], 16).ok()?;
    Some(Rgb { r, g, b })
}

/// Convert `Rgb` to `#RRGGBB`.
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

/// Euclidean distance squared in RGB (no gamma/perc.).
pub fn dist2(a: Rgb, b: Rgb) -> u32 {
    let dr = a.r as i32 - b.r as i32;
    let dg = a.g as i32 - b.g as i32;
    let db = a.b as i32 - b.b as i32;
    (dr * dr + dg * dg + db * db) as u32
}

/// Copy format for cycling through different color representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyFormat {
    Hex,        // #FF5733
    Rgb,        // rgb(255, 87, 51)
    Hsl,        // hsl(14, 100%, 60%)
    RgbValues,  // 255, 87, 51
}

impl CopyFormat {
    /// Cycle to the next format
    pub fn next(self) -> Self {
        match self {
            CopyFormat::Hex => CopyFormat::Rgb,
            CopyFormat::Rgb => CopyFormat::Hsl,
            CopyFormat::Hsl => CopyFormat::RgbValues,
            CopyFormat::RgbValues => CopyFormat::Hex,
        }
    }

    /// Get a display name for the format
    pub fn display_name(self) -> &'static str {
        match self {
            CopyFormat::Hex => "HEX",
            CopyFormat::Rgb => "RGB",
            CopyFormat::Hsl => "HSL",
            CopyFormat::RgbValues => "RGB Values",
        }
    }
}

impl Default for CopyFormat {
    fn default() -> Self {
        CopyFormat::Hex
    }
}

/// HSL representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    pub h: f32, // hue 0-360
    pub s: f32, // saturation 0-100
    pub l: f32, // lightness 0-100
}

/// Convert RGB to HSL
pub fn rgb_to_hsl(rgb: Rgb) -> Hsl {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let l = (max + min) / 2.0;

    let (h, s) = if delta == 0.0 {
        (0.0, 0.0) // achromatic
    } else {
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let h = if max == r {
            (g - b) / delta + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        (h * 60.0, s)
    };

    Hsl {
        h: h % 360.0,
        s: s * 100.0,
        l: l * 100.0,
    }
}

/// Format RGB in various formats
pub fn format_rgb(rgb: Rgb, format: CopyFormat) -> String {
    match format {
        CopyFormat::Hex => rgb_to_hex(rgb),
        CopyFormat::Rgb => format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b),
        CopyFormat::Hsl => {
            let hsl = rgb_to_hsl(rgb);
            format!("hsl({:.0}, {:.0}%, {:.0}%)", hsl.h, hsl.s, hsl.l)
        },
        CopyFormat::RgbValues => format!("{}, {}, {}", rgb.r, rgb.g, rgb.b),
    }
}
