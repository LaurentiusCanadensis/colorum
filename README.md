
# Colorum — Thesaurus Colorum

<img  height="336" alt="logo"  src="https://github.com/user-attachments/assets/cb2d3b59-693b-4d2d-8641-4cc6a1d5c837" />

A comprehensive Rust color management application with an interactive GUI built on the Iced framework. Features multiple color palette sources, real-time search, and an intuitive color wheel interface for exploring and working with colors.

## Features

<img width="744" height="542" alt="image" src="https://github.com/user-attachments/assets/7fd6dac7-da4f-4ef7-ae72-486b190fd2f4" />


### 🎨 **Multiple Color Palettes**
- **CSS Colors**: Standard web colors (140+ colors)
- **XKCD Colors**: Community-sourced color names (900+ colors)
- **Pantone Colors**: Professional color standards
- **Brand Colors**: Popular brand color palettes
- **Cultural Palettes**: Hindi, Persian, Italian brand colors
- **Specialized Sets**: National colors, Kelvin temperature colors, metal flame colors
- **GitHub Colors**: Community-contributed color palettes (feature-gated)

### 🔍 **Powerful Search Engine**
- Real-time substring search across all color names
- Token-based indexing for fast queries
- Multi-word search support
- Origin-based filtering (search within specific palettes)
- Keyboard navigation (Up/Down arrows, Enter to select)

### 🎡 **Interactive Color Wheel**
- Concentric RGB rings for intuitive color selection
- Real-time hex input synchronization
- Visual feedback with color preview
- Click-to-copy hex values
- Smooth canvas-based rendering
- Responsive design that adapts to window size
- Smart panel layout (side-by-side or stacked)

### 📊 **Advanced Color Analytics**
- **Color Distance**: Perceptual distance calculations using Lab color space
- **Text Readability**: WCAG contrast ratios for accessibility (against black/white)
- **CMYK Preview**: Print-ready color values for professional workflows
- **Closest Color Names**: Intelligent color name matching
- **Interactive Display**: All analytics are clickable for easy copying

### 🛠 **Developer-Friendly Library**
- **Hex utilities**: Normalize, validate, and convert hex colors
- **RGB utilities**: RGB struct with distance calculations
- **Color matching**: Find nearest named colors with perceptual accuracy
- **Search functions**: Powerful color search across all palettes
- **Color analytics**: WCAG contrast, CMYK conversion, Lab distance
- **Modular design**: Use as library or standalone application

## Installation & Quick Start

### Running the Application

Clone and run the standalone color picker application:

```bash
git clone https://github.com/LaurentiusCanadensis/colorum
cd colorum
cargo run
```

### Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
colorum = { git = "https://github.com/LaurentiusCanadensis/colorum" }
palette = "0.7"  # For advanced color analytics
iced = { version = "0.13", features = ["advanced", "canvas"] }  # For GUI widgets
```

### Feature Flags

Control which color palettes are included:

```toml
[dependencies]
colorum = {
    git = "https://github.com/LaurentiusCanadensis/colorum",
    features = ["github-colors"]  # Enable GitHub community colors
}
```

Available features:
- `github-colors`: Adds thousands of community-contributed colors (⚠️ increases binary size)
- `profile`: Enables tracing/debugging output

## Usage Examples

### Library Usage - Color Utilities

```rust
use colorum::{
    hex_to_rgb, rgb_to_hex, normalize_hex, Rgb, COLORS_CSS,
    get_closest_color_name_from_rgb, get_closest_color_name_from_hex
};

fn main() {
    // Hex normalization and validation
    let normalized = normalize_hex("#3af").unwrap(); // -> "#33AAFF"

    // RGB conversions
    let rgb = hex_to_rgb(&normalized).unwrap(); // Rgb { r: 51, g: 170, b: 255 }
    let hex_back = rgb_to_hex(rgb);            // "#33AAFF"

    // Color distance calculation
    let color1 = Rgb { r: 255, g: 0, b: 0 };   // Red
    let color2 = Rgb { r: 0, g: 255, b: 0 };   // Green
    let distance = colorum::dist2(color1, color2); // Euclidean distance squared

    // Find closest color names - NEW!
    if let Some(name) = get_closest_color_name_from_rgb(rgb) {
        println!("Closest color to {:?}: {}", rgb, name);
    }

    if let Some(name) = get_closest_color_name_from_hex("#FF6347") {
        println!("Closest color to tomato hex: {}", name);
    }

    // Access color palettes
    println!("CSS colors available: {}", COLORS_CSS.len());
    if let Some((_hex, name)) = COLORS_CSS.first() {
        println!("First CSS color: {}", name);
    }
}
```

### Library Usage - Color Search

```rust
use colorum::{
    search_colors, search_in_origin, origin_slice,
    Origin, TokenMode, COMBINED_COLORS
};

fn main() {
    // Search across all color palettes - NEW!
    let results = search_colors("red", TokenMode::Any);
    for (hex, name) in results.iter().take(5) {
        println!("{}: {}", name.as_str(), hex.as_str());
    }

    // Search within specific palette
    let css_reds = search_in_origin(Origin::Css, "red", TokenMode::Any);
    println!("Found {} CSS red variants", css_reds.len());

    // Get all colors from a palette
    let css_colors = origin_slice(Origin::Css);
    println!("CSS palette has {} colors", css_colors.len());

    // Access all colors
    println!("Total colors available: {}", COMBINED_COLORS.len());
}
```

### Library Usage - Color Analytics (NEW!)

```rust
use colorum::{Rgb, hex_to_rgb};
use palette::{Srgb, Lab, IntoColor};

fn main() {
    let rgb = Rgb { r: 255, g: 99, b: 71 }; // Tomato color

    // WCAG Contrast Analysis
    fn relative_luminance(r: u8, g: u8, b: u8) -> f32 {
        let to_linear = |c: u8| {
            let c = c as f32 / 255.0;
            if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b)
    }

    let luminance = relative_luminance(rgb.r, rgb.g, rgb.b);
    let white_contrast = (1.05) / (luminance + 0.05);
    let black_contrast = (luminance + 0.05) / (0.05);

    println!("Text contrast - White: {:.1}:1, Black: {:.1}:1", white_contrast, black_contrast);

    // CMYK Conversion
    let r_norm = rgb.r as f32 / 255.0;
    let g_norm = rgb.g as f32 / 255.0;
    let b_norm = rgb.b as f32 / 255.0;
    let k = 1.0 - r_norm.max(g_norm).max(b_norm);
    let c = if k == 1.0 { 0.0 } else { (1.0 - r_norm - k) / (1.0 - k) };
    let m = if k == 1.0 { 0.0 } else { (1.0 - g_norm - k) / (1.0 - k) };
    let y = if k == 1.0 { 0.0 } else { (1.0 - b_norm - k) / (1.0 - k) };

    println!("CMYK: C{}% M{}% Y{}% K{}%",
             (c * 100.0) as u8, (m * 100.0) as u8,
             (y * 100.0) as u8, (k * 100.0) as u8);

    // Lab Distance Calculation
    let current_srgb = Srgb::new(r_norm, g_norm, b_norm);
    let current_lab: Lab = current_srgb.into_color();
    let white_lab: Lab = Srgb::new(1.0, 1.0, 1.0).into_color();

    let distance = ((current_lab.l - white_lab.l).powi(2) +
                   (current_lab.a - white_lab.a).powi(2) +
                   (current_lab.b - white_lab.b).powi(2)).sqrt();
    println!("Lab distance from white: {:.1}", distance);
}
```

### Widget Integration

```rust
use iced::{application, Element, Theme, widget::container};
use colorum::{widgets::color_wheel::ColorWheel, messages::{Msg, Channel}};

#[derive(Default)]
struct ColorApp {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorApp {
    fn view(&self) -> Element<Msg> {
        let wheel = ColorWheel::new(self.r, self.g, self.b, Msg::WheelChanged);

        let wheel_view = wheel.view(
            "Color Picker",
            &format!("{:02X}", self.r),
            &format!("{:02X}", self.g),
            &format!("{:02X}", self.b)
        );

        container(wheel_view).into()
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::WheelChanged(channel, value) => {
                match channel {
                    Channel::R => self.r = value,
                    Channel::G => self.g = value,
                    Channel::B => self.b = value,
                }
            }
            _ => {}
        }
    }
}
```

---

## API Reference

### Core Modules

#### `hex` - Hex Color Utilities
```rust
// Normalize various hex formats to #RRGGBB
normalize_hex("#3af") -> "#33AAFF"
normalize_hex("#33AAFF") -> "#33AAFF"
normalize_hex("#33AAFFCC") -> "#33AAFF"  // strips alpha

// Split and combine hex components
split_hex("#33AAFF") -> ("33", "AA", "FF")
combine_hex("33", "AA", "FF") -> "#33AAFF"

// Name lookups
hex_for_name("tomato") -> Some("#FF6347")
name_for_hex("#FF6347") -> Some("tomato")
```

#### `rgb` - RGB Color Utilities
```rust
// RGB struct and conversions
Rgb { r: u8, g: u8, b: u8 }
hex_to_rgb("#FF6347") -> Rgb { r: 255, g: 99, b: 71 }
rgb_to_hex(rgb) -> "#FF6347"

// Color distance (Euclidean squared)
dist2(color1, color2) -> u32
```

#### `colors_helper` - Search and Catalogs
```rust
// Origin-based color filtering
Origin::All | Origin::Css | Origin::XKCD | Origin::Pantone | ...

// Search functions (NEW!)
search_colors(query: &str, mode: TokenMode) -> Vec<(HexCode, ColorName)>
search_in_origin(origin: Origin, query: &str, mode: TokenMode) -> Vec<(HexCode, ColorName)>
origin_slice(origin: Origin) -> &'static [(HexCode, ColorName)]

// Convenience functions (NEW!)
get_closest_color_name_from_rgb(rgb: Rgb) -> Option<&'static str>
get_closest_color_name_from_hex(hex: &str) -> Option<&'static str>
lookup_by_name(name: &str) -> Option<&'static str>
lookup_by_name_ci(name: &str) -> Option<&'static str>  // Case-insensitive

// Color collections
COMBINED_COLORS  // All colors from all palettes
COLORS_CSS       // Web-standard colors
COLORS_XKCD      // Community colors
// ... and more
```

#### `widgets::color_wheel` - Interactive GUI Component
```rust
ColorWheel::new(r: u8, g: u8, b: u8, on_change: F) -> Self
.view(title: &str, r_hex: &str, g_hex: &str, b_hex: &str) -> Element<Msg>
.view_with_search_props(...) -> Element<Msg>  // Includes search dropdown
```

## Color Palettes Included

| Palette | Count | Description |
|---------|-------|-------------|
| CSS | 140+ | Standard web colors |
| XKCD | 900+ | Community-sourced color names |
| Pantone | 200+ | Professional color standards |
| Hindi | 100+ | Traditional Hindi color names |
| Persian | 100+ | Traditional Persian color names |
| Brand Colors | 50+ | Popular brand palettes |
| Italian Brands | 30+ | Italian brand colors |
| National Colors | 200+ | Flag and national colors |
| Metal Flames | 20+ | Metal flame temperature colors |
| Kelvin Colors | 100+ | Temperature-based colors |
| GitHub Colors* | 1000+ | Community contributions |

*GitHub colors require the `github-colors` feature flag.



## Development

### Building and Testing

```bash
# Build the application
cargo build

# Run tests
cargo test

# Run with all features
cargo run --features github-colors

# Build optimized release
cargo build --release
```

### Responsive Design

Colorum features a fully responsive interface that adapts to different window sizes:

- **Large Windows** (>450px wide): Side-by-side layout with color wheel on the left, analytics and search panels on the right
- **Medium Windows** (350-450px): Vertical stacking with color wheel on top, panels below
- **Small Windows** (<350px): Shows only the color wheel for optimal mobile experience
- **Dynamic Sizing**: Color wheel and panels scale proportionally to window size
- **Smart Panel Visibility**: Panels automatically hide/show based on available screen real estate

### Performance Notes

- Color search uses pre-built token indices for fast substring matching
- Lazy loading of color palettes reduces startup time
- The `github-colors` feature significantly increases binary size due to large datasets
- Memory usage scales with enabled color palettes
- Responsive calculations are optimized for 60fps rendering

### Architecture

The project follows a modular design:

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library interface and re-exports
├── colors/           # Color palette modules
├── colors_helper/    # Search, indexing, and utilities
├── app_gui/          # Iced application logic
├── widgets/          # Custom UI components
├── rgb.rs            # RGB utilities
├── hex.rs            # Hex color utilities
└── messages.rs       # Application message types
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT © 2025 LaurentiusCanadensis
