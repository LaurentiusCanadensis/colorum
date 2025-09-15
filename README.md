# rust_colors

A tiny Rust library **and** Iced widget for working with hex colors made from three 2‑digit hex channels (RR, GG, BB).  
It includes UI‑free helpers (normalize/split/combine sanitization, RGB struct + conversions, nearest named color) and a **concentric RGB wheel** widget that you can drop into any Iced app.

---


## Features

- **Hex utilities**  
  - `normalize_hex`: supports `#RGB`, `#RRGGBB`, `#RRGGBBAA` → `#RRGGBB`  
  - `split_hex` / `combine_hex` (3× two‑hex into one)  
  - `sanitize_hex2`: keep **at most** two uppercase hex digits
- **RGB utilities**  
  - `Rgb { r, g, b }` struct  
  - `hex_to_rgb` / `rgb_to_hex`  
  - `dist2` (Euclidean distance squared)
- **Color names**  
  - `COMBINED_COLORS: &[(&'static str /*#RRGGBB*/, &'static str /*name*/)]`  
  - `nearest_name_r_eq_00("#RRGGBB") -> (&'static str, &'static str, u32)`
  - `hex_for_name` / `name_for_hex`
- **Iced widget: concentric RGB wheel**  
  - Three rings (R outer, G middle, B inner) + center swatch  
  - Drag while mouse is pressed; text inputs stay in sync  
  - Center shows `#RRGGBB`, **nearest color name** (star **★** if exact)  
  - Click center to **copy** hex to clipboard  
  - Optional search/dropdown for names

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust_colors = { git = "https://github.com/LaurentiusCanadensis/rust_colors" }
# If you're using the widget, add iced too:
iced = { version = "0.13", features = ["wgpu", "clipboard"] }
```

> The crate exposes pure helpers even without Iced; the widget lives under `rust_colors::widgets`.

---

## Quick start (library)

```rust
use rust_colors::{
    normalize_hex, split_hex, combine_hex,
    Rgb, hex_to_rgb, rgb_to_hex,
    hex_for_name, name_for_hex,
};

fn main() {
    // Normalize variants
    let norm = normalize_hex("#3af").unwrap(); // -> "#33AAFF"
    let (rr, gg, bb) = split_hex(&norm).unwrap(); // ("33","AA","FF")
    let combined = combine_hex(&rr, &gg, &bb);    // "#33AAFF"

    // Conversions
    let rgb = hex_to_rgb(&combined).unwrap();     // Rgb { r: 0x33, g: 0xAA, b: 0xFF }
    let back = rgb_to_hex(rgb);                   // "#33AAFF"

    // Names
    if let Some(name) = name_for_hex("#33AAFF") {
        println!("Exact name: {name}");
    }
    if let Some(hex) = hex_for_name("tomato") {
        println!("Tomato hex is {hex}");
    }
}
```

---

## Quick start (Iced widget)

Minimal embedding of the concentric RGB wheel:

```rust
use iced::{application, Element, Theme, Length, Task, widget::{column, container, text}};
use rust_colors::widgets::color_wheel::ColorWheel;

#[derive(Debug, Clone, Copy)]
enum Msg {
    WheelChanged(rust_colors::Channel, u8),
}

#[derive(Default)]
struct App { r: u8, g: u8, b: u8 }

impl App {
    fn title(&self) -> String { "RGB Wheel".into() }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        if let Msg::WheelChanged(ch, v) = msg {
            match ch { rust_colors::Channel::R => self.r = v, rust_colors::Channel::G => self.g = v, rust_colors::Channel::B => self.b = v }
        }
        Task::none()
    }

    fn view(&self) -> Element<Msg> {
        let wheel = ColorWheel::new(self.r, self.g, self.b, Msg::WheelChanged)
            .view("RGB Wheel", &format!("{:02X}", self.r), &format!("{:02X}", self.g), &format!("{:02X}", self.b));
        container(column![text(""), wheel]).width(Length::Fill).height(Length::Fill).into()
    }
}

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}
```

> The widget also supports click‑to‑copy of the center hex if you handle `Msg::CopyHex(String)` in your `update` and return `iced::clipboard::write(hex)`.

---

## API surface (selected)

- `hex` module  
  - `normalize_hex`, `split_hex`, `combine_hex`, `sanitize_hex2`  
  - `hex_for_name`, `name_for_hex`
- `rgb` module  
  - `Rgb`, `hex_to_rgb`, `rgb_to_hex`, `dist2`
- `colors` module  
  - `COMBINED_COLORS`, `nearest_name_r_eq_00`
- `widgets::color_wheel`  
  - `ColorWheel::new(r, g, b, on_change) -> Self`  
  - `.view(title, r_hex, g_hex, b_hex) -> Element<_>`

---

## Screenshots

![img.png](img.png)

![img_1.png](img_1.png)

![img_2.png](img_2.png)

---

## Development

```bash
cargo build
cargo test
```

If you use the widget:
- Enable `iced` with `wgpu` + `clipboard` features.
- macOS users may want to run with `RUST_BACKTRACE=1` for better error output.

---

## License

MIT © 2025 LaurentiusCanadensis
