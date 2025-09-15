// src/main.rs
use iced::{
    application, Alignment, Background, Color, Element, Length, Task, Theme,
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::widget::canvas::stroke;
use iced::{Point, Rectangle, mouse};
use rust_colors::names::COMBINED_COLORS;
/* ---------------- Combined array here (hex, name) ---------------- */
// Replace this sample with your full list:


/* ---------------- App ---------------- */

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}

#[derive(Debug, Clone)]
enum Msg {
    // text inputs
    RChanged(String),
    GChanged(String),
    BChanged(String),
    // wheels
    WheelChanged(Channel, u8),
    // search/dropdown
    SearchChanged(String),
    PickedName(String),
    // misc
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channel { R, G, B }

#[derive(Default)]
struct App {
    rr: String, // exactly 2 hex chars when valid
    gg: String,
    bb: String,

    search: String,
    selected_name: Option<String>,

    status: String,
}

impl App {
    fn title(&self) -> String {
        "rust_colors • 3×2-hex wheels + name search".into()
    }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::RChanged(s) => self.rr = sanitize_hex2(&s),
            Msg::GChanged(s) => self.gg = sanitize_hex2(&s),
            Msg::BChanged(s) => self.bb = sanitize_hex2(&s),

            Msg::WheelChanged(ch, v) => {
                let hh = format!("{v:02X}");
                match ch {
                    Channel::R => self.rr = hh,
                    Channel::G => self.gg = hh,
                    Channel::B => self.bb = hh,
                }
            }

            Msg::SearchChanged(s) => {
                self.search = s;

                // Build filtered list (borrowed &'static str options)
                let names = self.filtered_names();

                if let Some(first) = names.first().copied() {
                    // Always select the first match
                    self.selected_name = Some(first.to_string());

                    // And immediately sync RR/GG/BB to that color
                    if let Some(hex) = hex_for_name(first) {
                        if let Some(rgb) = hex_to_rgb(hex) {
                            self.rr = format!("{:02X}", rgb.r);
                            self.gg = format!("{:02X}", rgb.g);
                            self.bb = format!("{:02X}", rgb.b);
                        }
                    }
                } else {
                    // No matches -> clear selection (leave RR/GG/BB as-is)
                    self.selected_name = None;
                }
            }

            Msg::PickedName(name) => {
                // reflect selection
                self.selected_name = Some(name.clone());

                // ⬅️ also update the text box so it shows the chosen option
                self.search = name.clone();

                // sync RR/GG/BB & wheels
                if let Some(hex) = hex_for_name(&name) {
                    if let Some(rgb) = hex_to_rgb(hex) {
                        self.rr = format!("{:02X}", rgb.r);
                        self.gg = format!("{:02X}", rgb.g);
                        self.bb = format!("{:02X}", rgb.b);
                    }
                }
            }

            Msg::Clear => {
                self.rr.clear();
                self.gg.clear();
                self.bb.clear();
                self.search.clear();
                self.selected_name = None;
                self.status.clear();
            }
        }

        // status text
        if self.rr.len() == 2 && self.gg.len() == 2 && self.bb.len() == 2 {
            let hex = combine_hex(&self.rr, &self.gg, &self.bb);
            if let Some(name) = name_for_hex(&hex) {
                self.status = format!("Combined: {hex} • exact name: {name}");
                self.selected_name = Some(name.to_string());
            } else {
                self.status = format!("Combined: {hex}");
            }
        } else {
            self.status = "Type two hex digits (0–9, A–F) for each of R, G, B — or pick a name.".into();
        }

        Task::none()
    }

    fn view(&self) -> Element<Msg> {
        // inputs (2-hex per channel)
        let inputs = column![
            text("Color 1 (R)").size(16),
            text_input("RR", &self.rr).on_input(Msg::RChanged).width(Length::Fixed(100.0)),
            spacer(8.0),
            text("Color 2 (G)").size(16),
            text_input("GG", &self.gg).on_input(Msg::GChanged).width(Length::Fixed(100.0)),
            spacer(8.0),
            text("Color 3 (B)").size(16),
            text_input("BB", &self.bb).on_input(Msg::BChanged).width(Length::Fixed(100.0)),
        ]
            .spacing(6);

        // wheels (ring per channel, angle -> 0..255)
        let r_val = u8_from_hex2(&self.rr);
        let g_val = u8_from_hex2(&self.gg);
        let b_val = u8_from_hex2(&self.bb);

        let wheels = row![
            wheel_card("R", Channel::R, r_val, Color::from_rgb(1.0, 0.0, 0.0)),
            wheel_card("G", Channel::G, g_val, Color::from_rgb(0.0, 1.0, 0.0)),
            wheel_card("B", Channel::B, b_val, Color::from_rgb(0.0, 0.0, 1.0)),
        ]
            .spacing(16)
            .align_y(Alignment::Center);

        // combined swatch
        let (combined_hex, swatch_color) = if self.rr.len() == 2 && self.gg.len() == 2 && self.bb.len() == 2 {
            let hex = combine_hex(&self.rr, &self.gg, &self.bb);
            let rgb = hex_to_rgb(&hex).unwrap();
            (hex, Color::from_rgb8(rgb.r, rgb.g, rgb.b))
        } else {
            ("#------".to_string(), Color::from_rgb8(240, 240, 240))
        };

        let swatch = container(
            column![
                text("Combined").size(16),
                container(text(combined_hex.clone())).padding(6),
            ]
                .spacing(6)
        )
            .padding(10)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(swatch_color)),
                ..Default::default()
            })
            .width(Length::Fixed(160.0))
            .height(Length::Fixed(120.0));

        let top = row![inputs, spacer(12.0), wheels, spacer(12.0), swatch]
            .spacing(16)
            .align_y(Alignment::Center);

        // --- Search + dropdown ---
        // --- Search + dropdown ---
        let search_box = text_input("Search color name…", &self.search)
            .on_input(Msg::SearchChanged)
            .padding(8)
            .size(16)
            .width(Length::Fill);

        // Borrow names directly from your combined array as `&'static str`
        let names: Vec<&'static str> = self.filtered_names();

        // Ensure `selected` is exactly one of the items in `names`
        let selected: Option<&'static str> = self
            .selected_name
            .as_deref()
            .and_then(|cur| names.iter().copied().find(|n| n.eq_ignore_ascii_case(cur)));

        let dropdown = pick_list(
            names.clone(),
            selected,
            |picked: &'static str| Msg::PickedName(picked.to_string()) // convert here
        )
            .placeholder("Select a color")
            .width(Length::Fill);




        let info = column![
            text("• Type two hex digits per channel, or use the search + dropdown."),
            text("• Drag a wheel to set 0–255 for that channel."),
            text("• Inputs, wheels, and dropdown stay in sync."),
            text(&self.status),
        ]
            .spacing(4);

        let content = column![
            text("rust_colors • RGB wheels + 3×2-hex + name search").size(22),
            spacer(8.0),
            top,
            spacer(10.0),
            search_box,
            dropdown,
            spacer(10.0),
            scrollable(container(info).padding(8)).height(Length::Fill),
            spacer(6.0),
            row![button("Clear").on_press(Msg::Clear)],
        ]
            .spacing(12)
            .padding(12);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn filtered_names(&self) -> Vec<&'static str> {
        let q = self.search.trim().to_lowercase();

        if q.is_empty() {
            COMBINED_COLORS.iter().map(|&(_hex, name)| name).collect()
        } else {
            COMBINED_COLORS
                .iter()
                .map(|&(_hex, name)| name)
                .filter(|name| name.to_lowercase().contains(&q))
                .collect()
        }
    }
}

/* ------------ Wheel widget ------------ */

fn wheel_card(
    label: &'static str,
    channel: Channel,
    value: u8,
    tint: Color,
) -> Element<'static, Msg> {
    let wheel = RingWheel::new(channel, value, tint);

    container(
        column![
            text(label).size(18),
            Canvas::new(wheel)
                .width(Length::Fixed(180.0))
                .height(Length::Fixed(180.0)),
            text(format!("{value:02X} ({value:>3})")),
        ]
            .spacing(6)
            .width(Length::Fixed(180.0))
    )
        .width(Length::Fixed(200.0))
        .padding(6)
        .into()
}

/// Simple ring: angle -> value 0..255. Cached colored ring; live thumb.
struct RingWheel {
    channel: Channel,
    value: u8,
    tint: Color,
    ring_cache: canvas::Cache,
}

impl RingWheel {
    fn new(channel: Channel, value: u8, tint: Color) -> Self {
        Self { channel, value, tint, ring_cache: canvas::Cache::new() }
    }
}

impl Program<Msg> for RingWheel {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    )-> Vec<Geometry> {
        // 1) cached ring
        let ring = self.ring_cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = bounds.width.min(bounds.height) * 0.42;
            let thickness = radius * 0.22;

            for i in 0..360 {
                let t = i as f32 / 360.0;
                let val = (t * 255.0).round() as u8;
                let col = Color {
                    r: self.tint.r * (val as f32 / 255.0),
                    g: self.tint.g * (val as f32 / 255.0),
                    b: self.tint.b * (val as f32 / 255.0),
                    a: 1.0,
                };

                let a0 = (i as f32).to_radians();
                let a1 = ((i + 1) as f32).to_radians();
                let r_outer = radius + thickness / 2.0;
                let r_inner = radius - thickness / 2.0;

                let p0 = polar(center, r_inner, a0);
                let p1 = polar(center, r_outer, a0);
                let p2 = polar(center, r_outer, a1);
                let p3 = polar(center, r_inner, a1);

                let path = Path::new(|b| {
                    b.move_to(p0);
                    b.line_to(p1);
                    b.line_to(p2);
                    b.line_to(p3);
                    b.close();
                });
                frame.fill(&path, col);
            }
        });

        // 2) live thumb
        let mut thumb_frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = bounds.width.min(bounds.height) * 0.42;
        let angle = (self.value as f32 / 255.0) * std::f32::consts::TAU;
        let thumb_pos = polar(center, radius, angle);
        let thumb = Path::circle(thumb_pos, 6.5);

        thumb_frame.fill(&thumb, Color::BLACK);
        thumb_frame.stroke(
            &thumb,
            Stroke {
                width: 2.0,
                style: stroke::Style::Solid(Color::WHITE),
                ..Default::default()
            },
        );
        let thumb = thumb_frame.into_geometry();

        vec![ring, thumb]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) { mouse::Interaction::Pointer } else { mouse::Interaction::default() }
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Msg>) {
        let Some(pos) = cursor.position_in(bounds) else {
            return (canvas::event::Status::Ignored, None);
        };

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let v = iced::Vector::new(pos.x - center.x, pos.y - center.y);
        let dist = (v.x * v.x + v.y * v.y).sqrt();

        let radius = bounds.width.min(bounds.height) * 0.42;
        let thickness = radius * 0.22;
        let in_ring = dist >= (radius - thickness / 2.0) && dist <= (radius + thickness / 2.0);

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | canvas::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if !in_ring {
                    return (canvas::event::Status::Ignored, None);
                }
                let mut angle = v.y.atan2(v.x); // -PI..PI (0 at +X)
                if angle < 0.0 { angle += std::f32::consts::TAU; }
                let t = angle / std::f32::consts::TAU; // 0..1
                let val = (t * 255.0).round().clamp(0.0, 255.0) as u8;
                (canvas::event::Status::Captured, Some(Msg::WheelChanged(self.channel, val)))
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }
}

/* ------------- helpers ------------- */

fn spacer(px: f32) -> Element<'static, Msg> {
    container(text("")).height(Length::Fixed(px)).into()
}
fn polar(center: Point, r: f32, angle: f32) -> Point {
    Point::new(center.x + r * angle.cos(), center.y + r * angle.sin())
}
fn u8_from_hex2(s: &str) -> u8 {
    if s.len() == 2 {
        u8::from_str_radix(s, 16).unwrap_or(0)
    } else { 0 }
}

/* ------------- color utils (local) ------------- */

fn sanitize_hex2(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .take(2)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

struct Rgb { r: u8, g: u8, b: u8 }

fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    if s.len() != 6 { return None; }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Rgb { r, g, b })
}

fn combine_hex(rr: &str, gg: &str, bb: &str) -> String {
    format!("#{}{}{}", rr, gg, bb)
}

/* ------------- lookup helpers over COMBINED_COLORS ------------- */

fn hex_for_name(name: &str) -> Option<&'static str> {
    let n = name.trim();
    COMBINED_COLORS
        .iter()
        .find(|(hex, nm)| {
            let _ = hex;
            nm.eq_ignore_ascii_case(n)
        })
        .map(|(hex, _nm)| *hex)
}

fn name_for_hex(hex: &str) -> Option<&'static str> {
    let h = hex.trim();
    COMBINED_COLORS
        .iter()
        .find(|(hx, _nm)| hx.eq_ignore_ascii_case(h))
        .map(|(_hx, nm)| *nm)
}