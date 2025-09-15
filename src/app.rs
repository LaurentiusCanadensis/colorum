use iced::{Alignment, Background, Border, Color, Element, Length, Task, widget::{button, column, container, pick_list, row, scrollable, text, text_input}};

use crate::messages::{Channel, Msg};
use crate::util::{combine_hex, hex_for_name, hex_to_rgb, name_for_hex};
use crate::widgets::color_wheel::ColorWheel;
use rust_colors::sanitize_hex2;

use iced::border::Radius;


#[derive(Default)]
pub struct App {
    pub rr: String,
    pub gg: String,
    pub bb: String,

    pub search: String,
    pub selected_name: Option<String>,

    pub status: String, // kept for internal messages if you want
}

impl App {
    pub fn title(&self) -> String { String::new() } // not used anymore (title set in main)

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
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
                let names = self.filtered_names();
                if let Some(first) = names.first().copied() {
                    self.selected_name = Some(first.to_string());
                    if let Some(hex) = hex_for_name(first) {
                        if let Some(rgb) = hex_to_rgb(hex) {
                            self.rr = format!("{:02X}", rgb.r);
                            self.gg = format!("{:02X}", rgb.g);
                            self.bb = format!("{:02X}", rgb.b);
                        }
                    }
                } else {
                    self.selected_name = None;
                }
            }

            Msg::PickedName(name) => {
                self.selected_name = Some(name.clone());
                self.search = name.clone();
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

        // Optional status text (not rendered below)
        if self.rr.len() == 2 && self.gg.len() == 2 && self.bb.len() == 2 {
            let hex = combine_hex(&self.rr, &self.gg, &self.bb);
            if let Some(name) = name_for_hex(&hex) {
                self.status = format!("Combined: {hex} • exact name: {name}");
                self.selected_name = Some(name.to_string());
            } else {
                self.status = format!("Combined: {hex}");
            }
        } else {
            self.status.clear();
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Msg> {
        // Wheel values
        let r_val = u8_from_hex2(&self.rr);
        let g_val = u8_from_hex2(&self.gg);
        let b_val = u8_from_hex2(&self.bb);

        // The concentric wheel (has its own RR/GG/BB overlays)
        let wheel = ColorWheel::new(r_val, g_val, b_val, |ch, val| Msg::WheelChanged(ch, val))
            .view("RGB Wheel", &self.rr, &self.gg, &self.bb);

        // Search + dropdown, sized to 300px and bordered
        let search_box = text_input("Search color name…", &self.search)
            .on_input(Msg::SearchChanged)
            .padding(8)
            .size(16)
            .width(Length::Fill);

        let names = self.filtered_names();
        let selected: Option<&'static str> = self
            .selected_name
            .as_deref()
            .and_then(|cur| names.iter().copied().find(|n| n.eq_ignore_ascii_case(cur)));

        let dropdown = pick_list(
            names.clone(),
            selected,
            |picked: &str| Msg::PickedName(picked.to_string()),
        )
            .placeholder("Select a color")
            .width(Length::Fill);

        let search_panel = container(
            column![search_box, dropdown].spacing(8)
        )
            .width(Length::Fixed(300.0))
            .padding(8)
            .style(|_| iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgb(0.97, 0.97, 0.97))),
                border: Border {
                    width: 1.0,
                    color: Color::from_rgb(0.80, 0.80, 0.80),
                    radius: Radius::from(12.0),
                },
                ..Default::default()
            });

        // Final centered column: Wheel + Search panel + Clear
        let content = column![
                wheel,
                search_panel,
                row![button("Clear").on_press(Msg::Clear)]
                    .padding(4)
                    .align_y(Alignment::Center)
            ]
            .spacing(12)
            .padding(12)
            .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

   }



// helpers
fn u8_from_hex2(s: &str) -> u8 {
    if s.len() == 2 { u8::from_str_radix(s, 16).unwrap_or(0) } else { 0 }
}

impl App {
    fn rgb_dist2(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
        let dr = a.0 as i32 - b.0 as i32;
        let dg = a.1 as i32 - b.1 as i32;
        let db = a.2 as i32 - b.2 as i32;
        (dr * dr + dg * dg + db * db) as u32
    }

    fn nearest_named_for_hex(hex: &str) -> Option<(&'static str, &'static str)> {
        let target = hex_to_rgb(hex)?;
        let mut best: Option<(&'static str, &'static str, u32)> = None;
        for (h, name) in crate::colors::COMBINED_COLORS {
            if let Some(rgb) = hex_to_rgb(h) {
                let d = Self::rgb_dist2((target.r, target.g, target.b), (rgb.r, rgb.g, rgb.b));
                match best {
                    None => best = Some((name, h, d)),
                    Some((_bn, _bh, bd)) if d < bd => best = Some((name, h, d)),
                    _ => {}
                }
            }
        }
        best.map(|(n, h, _)| (n, h))
    }
}
impl App{

    fn filtered_names(&self) -> Vec<&'static str> {
        let q = self.search.trim().to_lowercase();
        if q.is_empty() {
            crate::colors::COMBINED_COLORS
                .iter()
                .map(|&(_hex, name)| name)
                .collect()
        } else {
            crate::colors::COMBINED_COLORS
                .iter()
                .map(|&(_hex, name)| name)
                .filter(|name| name.to_lowercase().contains(&q))
                .collect()
        }
    }
}

fn spacer(px: f32) -> Element<'static, Msg> {
    container(text("")).height(Length::Fixed(px)).into()
}