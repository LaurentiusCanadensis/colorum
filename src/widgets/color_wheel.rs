use crate::colors_helper::COMBINED_COLORS;
use crate::messages::{Channel, Msg};
use iced::border::Radius;
use iced::widget::canvas::stroke;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::widget::text_input as ti;
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Point, Rectangle, alignment,
    mouse,
    widget::{column, container, text, text_input},
};

use crate::hex_to_rgb;

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
            if let Some(rgb) = hex_to_rgb(h) {
                let d = dist2((self.r, self.g, self.b), (rgb.r, rgb.g, rgb.b));
                if best_d.map_or(true, |bd| d < bd) {
                    best_d = Some(d);
                    best_hex = Some(h);
                    best_name = Some(name);
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

        overlay.fill_text(canvas::Text {
            content: label,
            position: center,
            color: text_color,
            size: iced::Pixels(15.0),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            ..Default::default()
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
