use crate::ui::messages::{Channel, Msg};
use iced::widget::canvas::stroke;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length, Point, Rectangle, mouse};

pub fn combined_wheel_card(r: u8, g: u8, b: u8) -> Element<'static, Msg> {
    let wheel = CombinedWheel::new(r, g, b);

    container(
        column![
            text("Combined RGB Wheel").size(18),
            Canvas::new(wheel)
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(300.0)),
            row![
                text(format!("R: {:02X}", r)),
                text("  "),
                text(format!("G: {:02X}", g)),
                text("  "),
                text(format!("B: {:02X}", b)),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .width(Length::Fixed(300.0)),
    )
    .padding(6)
    .into()
}

#[derive(Default)]
pub struct ComboState {
    dragging: Option<Channel>,
}

pub struct CombinedWheel {
    r: u8,
    g: u8,
    b: u8,
    ring_cache: canvas::Cache,
}

impl CombinedWheel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            ring_cache: canvas::Cache::new(),
        }
    }
}

impl Program<Msg> for CombinedWheel {
    type State = ComboState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geom = self
            .ring_cache
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

                // Rings
                paint_ring(frame, r_outer, Color::from_rgb(1.0, 0.0, 0.0)); // R
                paint_ring(frame, r_mid, Color::from_rgb(0.0, 1.0, 0.0)); // G
                paint_ring(frame, r_inner, Color::from_rgb(0.0, 0.0, 1.0)); // B

                // Center combined circle
                let radius = (r_inner - ring_thickness - gap).max(20.0);
                let circle = Path::circle(center, radius);
                let combined = Color::from_rgb8(self.r, self.g, self.b);
                frame.fill(&circle, combined);

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
                thumb(frame, r_outer, self.r);
                thumb(frame, r_mid, self.g);
                thumb(frame, r_inner, self.b);
            });

        vec![geom]
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
        let Some(pos) = cursor.position_in(bounds) else {
            return (canvas::event::Status::Ignored, None);
        };

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let outer_radius = bounds.width.min(bounds.height) * 0.45;
        let ring_thickness = outer_radius * 0.18;
        let gap = ring_thickness * 0.08;

        let r_outer = outer_radius;
        let r_mid = r_outer - (ring_thickness + gap);
        let r_inner = r_mid - (ring_thickness + gap);

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

        use iced::widget::canvas::event::Status::{Captured, Ignored};

        match event {
            canvas::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                // --- ⬇️ NEW: center click (copy) radius ---
                // This is the radius of the solid inner disk (inside the B ring).
                // Adjust the factor if your visual inner disk is a bit larger/smaller.
                let inner_copy_radius = r_inner - (ring_thickness / 2.0 + gap); // inside edge of the B ring
                // Optional: shrink slightly so a click near the edge doesn't count as center
                let inner_copy_radius = inner_copy_radius * 0.92;

                if dist <= inner_copy_radius {
                    // Clicked the center
                    return (Captured, Some(Msg::CenterClicked));
                }
                // --- ⬆️ NEW ---

                if let Some(ch) = which_ring(dist) {
                    state.dragging = Some(ch);
                    let val = compute_val();
                    return (Captured, Some(Msg::WheelChanged(ch, val)));
                }
                (Ignored, None)
            }
            canvas::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                if let Some(ch) = state.dragging {
                    let val = compute_val();
                    return (Captured, Some(Msg::WheelChanged(ch, val)));
                }
                (Ignored, None)
            }
            canvas::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                if let Some(ch) = state.dragging.take() {
                    if which_ring(dist).is_some() {
                        let val = compute_val();
                        return (Captured, Some(Msg::WheelChanged(ch, val)));
                    }
                    return (Captured, None);
                }
                (Ignored, None)
            }
            _ => (Ignored, None),
        }
    }
}

fn polar(center: Point, r: f32, angle: f32) -> Point {
    Point::new(center.x + r * angle.cos(), center.y + r * angle.sin())
}
