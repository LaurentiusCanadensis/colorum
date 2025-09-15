use iced::{
    Alignment, Background, Color, Element, Length, Task,
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
};
use rust_colors::sanitize_hex2;
use crate::messages::{Msg, Channel};
use crate::util::{ hex_to_rgb, combine_hex, hex_for_name, name_for_hex};
use crate::widgets::combined_wheel::combined_wheel_card;

#[derive(Default)]
pub struct App {
    pub rr: String,
    pub gg: String,
    pub bb: String,

    pub search: String,
    pub selected_name: Option<String>,

    pub status: String,
}

impl App {
    pub fn title(&self) -> String {
        "rust_colors • concentric RGB wheel + name search".into()
    }

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
                self.search = name.clone(); // reflect into text box
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

        if self.rr.len() == 2 && self.gg.len() == 2 && self.bb.len() == 2 {
            let hex = combine_hex(&self.rr, &self.gg, &self.bb);
            if let Some(name) = name_for_hex(&hex) {
                self.status = format!("Combined: {hex} • exact name: {name}");
                self.selected_name = Some(name.to_string());
            } else {
                self.status = format!("Combined: {hex}");
            }
        } else {
            self.status = "Type two hex digits per channel (0–9, A–F) or use the name picker.".into();
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Msg> {
        let inputs = column![
            text("R").size(16),
            text_input("RR", &self.rr).on_input(Msg::RChanged).width(Length::Fixed(100.0)),
            spacer(8.0),
            text("G").size(16),
            text_input("GG", &self.gg).on_input(Msg::GChanged).width(Length::Fixed(100.0)),
            spacer(8.0),
            text("B").size(16),
            text_input("BB", &self.bb).on_input(Msg::BChanged).width(Length::Fixed(100.0)),
        ]
            .spacing(6);

        let r_val = u8_from_hex2(&self.rr);
        let g_val = u8_from_hex2(&self.gg);
        let b_val = u8_from_hex2(&self.bb);

        let wheel = combined_wheel_card(r_val, g_val, b_val);

        // combined swatch (as a card for clarity)
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

        let top = row![inputs, spacer(12.0), wheel, spacer(12.0), swatch]
            .spacing(16)
            .align_y(Alignment::Center);

        // Search + dropdown
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

        let dropdown = iced::widget::pick_list(
            names.clone(),
            selected,
            |picked: &str| Msg::PickedName(picked.to_string()),
        )
            .placeholder("Select a color")
            .width(Length::Fill);

        let info = column![
            text("• Concentric wheel: R outer, G middle, B inner; drag while mouse is pressed."),
            text("• Type two hex digits per channel — inputs, wheel and dropdown stay in sync."),
            text(&self.status),
        ]
            .spacing(4);

        let content = column![
            text("rust_colors • concentric RGB wheel + name search").size(22),
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
            crate::colors::COMBINED_COLORS.iter().map(|&(_hex, name)| name).collect()
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
fn u8_from_hex2(s: &str) -> u8 {
    if s.len() == 2 { u8::from_str_radix(s, 16).unwrap_or(0) } else { 0 }
}