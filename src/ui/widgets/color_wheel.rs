use crate::ui::app_gui::App;
use crate::colors_helper::COMBINED_COLORS;
use crate::ui::messages::{Channel, Msg};
use crate::core::color_types::{HexCode, ColorName};
use iced::border::Radius;
use iced::widget::canvas::stroke;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::widget::text_input as ti;
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Point, Rectangle, alignment,
    mouse,
    widget::{column, container, text, text_input},
};

use crate::core::hex_to_rgb;

#[derive(Default)]
pub struct WheelState {
    pub(crate) dragging: Option<Channel>,
}

pub struct ColorWheel<F>
where
    F: Fn(Channel, u8) -> Msg + Clone + 'static,
{
    r: u8,
    g: u8,
    b: u8,
    on_change: F,
    cache: canvas::Cache,
}

impl<F> ColorWheel<F>
where
    F: Fn(Channel, u8) -> Msg + Clone + 'static,
{
    pub fn new(r: u8, g: u8, b: u8, on_change: F) -> Self {
        Self {
            r,
            g,
            b,
            on_change,
            cache: canvas::Cache::new(),
        }
    }

    /// Renders the canvas and overlays the 3 hex inputs **inside** the wheel.
    pub fn view(
        self,
        label: &'static str,
        r_hex: &str,
        g_hex: &str,
        b_hex: &str,
    ) -> Element<'static, Msg> {
        use iced::widget::stack;

        // Base wheel canvas
        let canvas = Canvas::new(self)
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(300.0));

        // Geometry parameters
        let size = 300.0_f32;
        let center = size / 2.0;
        let outer_radius = size.min(size) * 0.45;
        let ring_thickness = outer_radius * 0.18;
        let gap = ring_thickness * 0.08;

        let r_outer = outer_radius; // R
        let r_mid = r_outer - (ring_thickness + gap); // G
        let r_inner = r_mid - (ring_thickness + gap); // B

        let half_field_h = 18.0_f32;

        // Helper to place an input at the north of a ring
        let place_input =
            |value: &str, placeholder: &'static str, on_input: fn(String) -> Msg, radius: f32| {
                let v_adjust = ring_thickness * 0.12;
                let input_w = 33.0_f32;
                let top_px = (center - radius - half_field_h + v_adjust).max(0.0);
                let left_px = (center - (input_w / 2.0)).clamp(0.0, size - input_w);

                let field = text_input(placeholder, value)
                    .on_input(on_input)
                    .padding(6)
                    .size(14)
                    .width(Length::Fixed(input_w))
                    .style(|_: &iced::Theme, _status: ti::Status| ti::Style {
                        background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1)),
                        border: Border {
                            width: 0.5,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            radius: Radius::from(6.0),
                        },

                        icon: Color::from_rgb(0.35, 0.35, 0.35),
                        placeholder: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
                        value: Color::WHITE,
                        selection: Color::from_rgba(0.20, 0.55, 1.0, 0.35),
                    });

                container(field)
                    .width(Length::Fixed(size))
                    .height(Length::Fixed(size))
                    .padding(Padding {
                        top: top_px,
                        right: 0.0,
                        bottom: 0.0,
                        left: left_px,
                    })
                    .into()
            };

        let r_input_layer: Element<Msg> = place_input(r_hex, "RR", Msg::RChanged, r_outer);
        let g_input_layer: Element<Msg> = place_input(g_hex, "GG", Msg::GChanged, r_mid);
        let b_input_layer: Element<Msg> = place_input(b_hex, "BB", Msg::BChanged, r_inner);

        container(
            column![
                text(label).size(18),
                stack![canvas, r_input_layer, g_input_layer, b_input_layer,],
            ]
            .spacing(8)
            .width(Length::Fixed(300.0))
            .align_x(Alignment::Center),
        )
        .padding(6)
        .into()
    }

    /// Renders the wheel AND the old fast search + index-driven dropdown
    /// as a single widget you can drop into your layout. It uses the
    /// precomputed results in `App` (query, results_idx, sel_pos, base).
    pub fn view_with_search<'a>(self, title: &'static str, app: &'a App) -> Element<'a, Msg> {
        use iced::widget::{Space, column, container, mouse_area, scrollable, text};
        use iced::{Alignment, Background, Color, Length, Renderer, Theme, border};

        // Reuse the existing wheel view with current rr/gg/bb from App
        let wheel_core: Element<'static, Msg> = self.view(title, &app.rr, &app.gg, &app.bb);
        let show_dropdown = app.dropdown_open && !app.results_idx.is_empty();

        // Search box (emits QueryChanged / PressedEnter) — NO filtering here
        let search_box: iced::widget::TextInput<'a, Msg, Theme, Renderer> =
            iced::widget::text_input("Search color name…", &app.query)
                .on_input(Msg::QueryChanged)
                .on_submit(Msg::PressedEnter)
                .padding(8)
                .width(Length::Fill);

        // Index-driven dropdown (NO recompute here). Mirrors your old good code.
        fn view_dropdown<'a>(app: &'a App) -> Element<'a, Msg> {
            if app.results_idx.is_empty() {
                return Space::with_height(0).into();
            }

            let mut col = column![]
                .spacing(2)
                .padding(4)
                .align_x(Alignment::Start)
                .width(Length::Fill);

            for (row, &idx) in app.results_idx.iter().enumerate() {
                let (hex, name) = app.base[idx];
                let is_sel = app.sel_pos == Some(row);
                let label = if is_sel {
                    format!("▶ {}  {}", name.as_str(), hex.as_str())
                } else {
                    format!("{}  {}", name.as_str(), hex.as_str())
                };

                let row_body = container(text(label))
                    .padding([6, 8])
                    .width(Length::Fill)
                    .style(move |_theme: &Theme| {
                        if is_sel {
                            iced::widget::container::Style {
                                background: Some(Background::Color(Color {
                                    r: 0.20,
                                    g: 0.40,
                                    b: 0.80,
                                    a: 0.20,
                                })),
                                border: border::Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        } else {
                            iced::widget::container::Style::default()
                        }
                    });

                let click = mouse_area(row_body).on_press(Msg::DropdownClicked(row));
                col = col.push(click);
            }

            scrollable(col)
                .id(app.dropdown_scroll_id.clone())
                .height(Length::Fixed(220.0))
                .width(Length::Fill)
                .into()
        }

        let dropdown: Option<Element<'a, Msg>> = if show_dropdown {
            Some(view_dropdown(app))
        } else {
            None
        };

        let mut stack = column![search_box]
            .spacing(8)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        if let Some(dd) = dropdown {
            stack = stack.push(dd);
        }

        container(
            column![wheel_core, stack,]
                .spacing(12)
                .align_x(Alignment::Center),
        )
        .padding([8, 8])
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}

/// Props for embedding the search UI with the ColorWheel in any host app
pub struct WheelSearchProps<'a> {
    pub query: &'a str,
    pub results_idx: &'a [usize],
    pub sel_pos: Option<usize>,
    pub base: &'a [(HexCode, ColorName)],
    pub scroll_id: iced::widget::scrollable::Id,
    pub on_query: fn(String) -> crate::ui::messages::Msg,
    pub on_enter: fn() -> crate::ui::messages::Msg,
    pub on_click_row: fn(usize) -> crate::ui::messages::Msg,
}

fn greedy_wrap<'a>(text: &'a str, max_chars: usize, max_lines: usize) -> String {
    if max_chars == 0 || max_lines == 0 {
        return String::new();
    }

    let mut out: Vec<String> = Vec::with_capacity(max_lines);
    let mut line = String::new();

    for (i, word) in text.split_whitespace().enumerate() {
        let sep = if i == 0 || line.is_empty() { "" } else { " " };
        if line.len() + sep.len() + word.len() <= max_chars {
            line.push_str(sep);
            line.push_str(word);
        } else {
            if !line.is_empty() {
                out.push(std::mem::take(&mut line));
                if out.len() == max_lines {
                    break;
                }
            }
            // If single word longer than max, hard-truncate it
            if word.chars().count() > max_chars {
                let truncated: String = word.chars().take(max_chars.saturating_sub(1)).collect();
                out.push(format!("{}…", truncated));
            } else {
                line.push_str(word);
            }
        }
        if out.len() == max_lines {
            break;
        }
    }

    if !line.is_empty() && out.len() < max_lines {
        out.push(line);
    }

    // If we exceeded lines, add ellipsis to the last one
    if out.len() == max_lines
        && text.split_whitespace().count() > out.iter().flat_map(|l| l.split_whitespace()).count()
    {
        if let Some(last) = out.last_mut() {
            if !last.ends_with('…') {
                last.push('…');
            }
        }
    }

    out.join("\n")
}

fn compute_typography(inner_radius: f32) -> (f32, f32, usize, usize) {
    // Scale fonts with the wheel’s inner disc size.
    // You can tweak these multipliers for taste.
    let hex_size = (inner_radius * 0.12).clamp(12.0, 28.0);
    let name_size = (inner_radius * 0.10).clamp(11.0, 24.0);

    // Very rough average glyph width at ~16px is ~8px; scale to our size
    let avg_w_hex = (hex_size / 16.0) * 8.0;
    let avg_w_name = (name_size / 16.0) * 8.0;

    // Let usable width be ~70% of diameter (avoid overflow, no clip)
    let usable_w = inner_radius * 2.0 * 0.70;
    let max_chars_hex = (usable_w / avg_w_hex).floor().max(8.0) as usize;
    let max_chars_name = (usable_w / avg_w_name).floor().max(8.0) as usize;

    // Allow up to 2 lines for the name
    let _max_lines_name = 2usize;
    (hex_size, name_size, max_chars_hex, max_chars_name.min(40))
}
impl<F> ColorWheel<F>
where
    F: Fn(crate::ui::messages::Channel, u8) -> crate::ui::messages::Msg + Clone + 'static,
{
    /// Render wheel **plus** search + dropdown, using the host app's state/callbacks.
    pub fn view_with_search_props<'a>(
        self,
        title: &'static str,
        rr: &'a str,
        gg: &'a str,
        bb: &'a str,
        props: WheelSearchProps<'a>,
    ) -> iced::Element<'a, crate::ui::messages::Msg> {
        use crate::ui::messages::Msg;
        use iced::widget::container as container_widget;
        use iced::widget::{Space, column, container, mouse_area, scrollable, text, text_input};
        use iced::{Alignment, Background, Color, Length, Renderer, Theme};

        // Core wheel (this consumes `self`, same as `view`)
        let wheel_core: iced::Element<'a, Msg> = self.view(title, rr, gg, bb);

        // Search box wired to your app callbacks
        let search_box: iced::widget::TextInput<'a, Msg, Theme, Renderer> =
            text_input("Search color name…", props.query)
                .on_input(props.on_query)
                .on_submit((props.on_enter)())
                .padding(4)
                .size(14)
                .width(Length::Fill);

        // Dropdown built from *indices* (no recompute here)
        fn dropdown<'a>(props: &WheelSearchProps<'a>) -> iced::Element<'a, Msg> {
            if props.results_idx.is_empty() {
                return Space::with_height(0).into();
            }

            let mut col = column![]
                .spacing(1)
                .padding(4)
                .align_x(Alignment::Start)
                .width(Length::Fill);

            for (row, &idx) in props.results_idx.iter().enumerate() {
                let (hex, name) = props.base[idx];
                let is_sel = props.sel_pos == Some(row);
                let label = if is_sel {
                    format!("▶ {}  {}", name.as_str(), hex.as_str())
                } else {
                    format!("{}  {}", name.as_str(), hex.as_str())
                };

                let row_body = container(text(label))
                    .padding([4, 6])
                    .width(Length::Fill)
                    .style(move |_theme: &Theme| {
                        if is_sel {
                            container_widget::Style {
                                background: Some(Background::Color(Color {
                                    r: 0.20,
                                    g: 0.40,
                                    b: 0.80,
                                    a: 0.20,
                                })),
                                border: iced::border::Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        } else {
                            container_widget::Style::default()
                        }
                    });

                let click = mouse_area(row_body).on_press((props.on_click_row)(row));
                col = col.push(click);
            }

            scrollable(col)
                .id(props.scroll_id.clone())
                .height(Length::Fixed(180.0))
                .width(Length::Fill)
                .into()
        }

        // Stack: search box + (optional) dropdown
        let mut stack = column![search_box]
            .spacing(4)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        if !props.results_idx.is_empty() {
            stack = stack.push(dropdown(&props));
        }

        container(
            column![wheel_core, stack]
                .spacing(8)
                .align_x(Alignment::Center),
        )
        .padding([6, 6])
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}

impl<F> Program<Msg> for ColorWheel<F>
where
    F: Fn(Channel, u8) -> Msg + Clone + 'static,
{
    type State = WheelState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        // Cached rings
        let rings = self
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
                let outer_radius = bounds.width.min(bounds.height) * 0.45;

                let ring_thickness = outer_radius * 0.18;
                let gap = ring_thickness * 0.08;

                let r_outer = outer_radius;
                let r_mid = r_outer - (ring_thickness + gap);
                let r_inner = r_mid - (ring_thickness + gap);

                let paint_ring = |frame: &mut Frame, radius: f32, tint: Color| {
                    for i in 0..360 {
                        let t = i as f32 / 360.0;
                        let val = (t * 255.0).round() as u8;

                        let col = Color {
                            r: tint.r * (val as f32 / 255.0),
                            g: tint.g * (val as f32 / 255.0),
                            b: tint.b * (val as f32 / 255.0),
                            a: 1.0,
                        };

                        let a0 = (i as f32).to_radians();
                        let a1 = ((i + 1) as f32).to_radians();
                        let r_out = radius + ring_thickness / 2.0;
                        let r_inn = radius - ring_thickness / 2.0;

                        let p0 = polar(center, r_inn, a0);
                        let p1 = polar(center, r_out, a0);
                        let p2 = polar(center, r_out, a1);
                        let p3 = polar(center, r_inn, a1);

                        let path = Path::new(|b| {
                            b.move_to(p0);
                            b.line_to(p1);
                            b.line_to(p2);
                            b.line_to(p3);
                            b.close();
                        });
                        frame.fill(&path, col);
                    }
                };

                paint_ring(frame, r_outer, Color::from_rgb(1.0, 0.0, 0.0));
                paint_ring(frame, r_mid, Color::from_rgb(0.0, 1.0, 0.0));
                paint_ring(frame, r_inner, Color::from_rgb(0.0, 0.0, 1.0));
            });

        // Overlay
        let mut overlay = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let outer_radius = bounds.width.min(bounds.height) * 0.45;
        let ring_thickness = outer_radius * 0.18;
        let gap = ring_thickness * 0.08;

        let r_outer = outer_radius;
        let r_mid = r_outer - (ring_thickness + gap);
        let r_inner = r_mid - (ring_thickness + gap);

        // Make the inner circle a bit bigger
        let inner_radius = (r_inner - ring_thickness * 0.40).max(24.0);
        let circle = Path::circle(center, inner_radius);
        let combined = Color::from_rgb8(self.r, self.g, self.b);
        overlay.fill(&circle, combined);

        // Thumbs
        let thumb = |frame: &mut Frame, radius: f32, value: u8| {
            let angle = (value as f32 / 255.0) * std::f32::consts::TAU;
            let pos = polar(center, radius, angle);
            let circ = Path::circle(pos, 6.0);
            frame.fill(&circ, Color::BLACK);
            frame.stroke(
                &circ,
                Stroke {
                    width: 2.0,
                    style: stroke::Style::Solid(Color::WHITE),
                    ..Default::default()
                },
            );
        };
        thumb(&mut overlay, r_outer, self.r);
        thumb(&mut overlay, r_mid, self.g);
        thumb(&mut overlay, r_inner, self.b);

        // Compute nearest color
        let combined_hex = format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b);
        let dist2 = |a: (u8, u8, u8), b: (u8, u8, u8)| -> u32 {
            let dr = a.0 as i32 - b.0 as i32;
            let dg = a.1 as i32 - b.1 as i32;
            let db = a.2 as i32 - b.2 as i32;
            (dr * dr + dg * dg + db * db) as u32
        };

        let mut best_name: Option<&'static str> = None;
        let mut best_hex: Option<&'static str> = None;
        let mut best_d: Option<u32> = None;

        for (h, name) in COMBINED_COLORS.iter() {
            if let Some(rgb) = hex_to_rgb(h.as_str()) {
                let d = dist2((self.r, self.g, self.b), (rgb.r, rgb.g, rgb.b));
                if best_d.map_or(true, |bd| d < bd) {
                    best_d = Some(d);
                    best_hex = Some(h.as_str());
                    best_name = Some(name.as_str());
                }
            }
        }

        let mut label = combined_hex.clone();
        if let (Some(nm), Some(hx)) = (best_name, best_hex) {
            let exact = hx.eq_ignore_ascii_case(&combined_hex);
            if exact {
                label.push_str(&format!("\n{} *", nm));
            } else {
                label.push_str(&format!("\n{}", nm));
            }
        }

        let luminance = 0.2126 * (self.r as f32 / 255.0)
            + 0.7152 * (self.g as f32 / 255.0)
            + 0.0722 * (self.b as f32 / 255.0);
        let text_color = if luminance > 0.6 {
            Color::from_rgb(0.1, 0.1, 0.1)
        } else {
            Color::WHITE
        };

        // --- Wrapped center label (no clip, just width budget) ---
        // Split the composed label into hex and name lines
        let hex_str = label.lines().next().unwrap_or(&combined_hex);
        let name_str = label.split_once('\n').map(|(_, n)| n).unwrap_or("");

        // Scale typography to inner disc and wrap name to fit
        let (hex_size, name_size, _max_hex, max_name) = compute_typography(inner_radius);
        let wrapped_name = greedy_wrap(name_str, max_name, 2);

        overlay.with_save(|frame| {
            // No `frame.clip` available in this Iced version. We keep text within the
            // inner disc by sizing/wrapping conservatively in `compute_typography`.

            // HEX line (centered, slightly above)
            frame.fill_text(canvas::Text {
                content: hex_str.to_string(),
                position: center + iced::Vector::new(0.0, -name_size * 0.6),
                color: text_color,
                size: iced::Pixels(hex_size),
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: alignment::Vertical::Center,
                ..Default::default()
            });

            // Wrapped name (centered, below)
            frame.fill_text(canvas::Text {
                content: wrapped_name,
                position: center + iced::Vector::new(0.0, name_size * 0.25),
                color: text_color,
                size: iced::Pixels(name_size),
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: alignment::Vertical::Top,
                ..Default::default()
            });
        });

        vec![rings, overlay.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Msg>) {
        use canvas::event::Status::{Captured, Ignored};

        let Some(pos) = cursor.position_in(bounds) else {
            return (Ignored, None);
        };

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let outer_radius = bounds.width.min(bounds.height) * 0.45;
        let ring_thickness = outer_radius * 0.18;
        let gap = ring_thickness * 0.08;

        let r_outer = outer_radius;
        let r_mid = r_outer - (ring_thickness + gap);
        let r_inner = r_mid - (ring_thickness + gap);

        // Same inner radius as in `draw` (keep in sync)
        let inner_radius = (r_inner - ring_thickness * 0.40).max(24.0);

        let in_band = |dist: f32, radius: f32| {
            dist >= (radius - ring_thickness / 2.0) && dist <= (radius + ring_thickness / 2.0)
        };

        let which_ring = |dist: f32| -> Option<Channel> {
            if in_band(dist, r_outer) {
                Some(Channel::R)
            } else if in_band(dist, r_mid) {
                Some(Channel::G)
            } else if in_band(dist, r_inner) {
                Some(Channel::B)
            } else {
                None
            }
        };

        let v = iced::Vector::new(pos.x - center.x, pos.y - center.y);
        let dist = (v.x * v.x + v.y * v.y).sqrt();

        let compute_val = || {
            let mut angle = v.y.atan2(v.x);
            if angle < 0.0 {
                angle += std::f32::consts::TAU;
            }
            let t = angle / std::f32::consts::TAU;
            (t * 255.0).round().clamp(0.0, 255.0) as u8
        };

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // Click on center: copy combined hex
                if dist <= inner_radius {
                    let hex = format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b);
                    return (Captured, Some(Msg::CopyHex(hex)));
                }
                if let Some(ch) = which_ring(dist) {
                    state.dragging = Some(ch);
                    let val = compute_val();
                    return (Captured, Some((self.on_change)(ch, val)));
                }
                (Ignored, None)
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(ch) = state.dragging {
                    let val = compute_val();
                    return (Captured, Some((self.on_change)(ch, val)));
                }
                (Ignored, None)
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(ch) = state.dragging.take() {
                    if which_ring(dist).is_some() {
                        let val = compute_val();
                        return (Captured, Some((self.on_change)(ch, val)));
                    }
                    return (Captured, None);
                }
                (Ignored, None)
            }
            _ => (Ignored, None),
        }
    }
}

/* -------- helpers -------- */

fn polar(center: Point, r: f32, angle: f32) -> Point {
    Point::new(center.x + r * angle.cos(), center.y + r * angle.sin())
}
