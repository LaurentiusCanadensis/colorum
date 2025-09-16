// src/app.rs
use crate::colors::{self, COLORS_XKCD, Origin};
use crate::hex::{combine_hex, sanitize_hex2};
use crate::messages::Msg;
use crate::rgb::hex_to_rgb;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Task, clipboard};

#[cfg(feature = "github-colors")]
use crate::github_colors::COLORS_GITHUB;
use crate::hindi_colors::COLORS_HINDI;
use crate::pantone_colors::COLORS_PANTONE;
use persian_colors::COLORS_PERSIAN;
use rust_colors::{COMBINED_COLORS, persian_colors};

#[derive(Default)]
pub struct App {
    pub rr: String,
    pub gg: String,
    pub bb: String,

    pub search: String,
    pub selected_name: Option<String>,

    pub selected_origin: Origin,
    pub status: String,
}

impl App {
    pub fn title(&self) -> String {
        "rondel".into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::RChanged(s) => {
                self.rr = sanitize_hex2(&s);
                Task::none()
            }
            Msg::GChanged(s) => {
                self.gg = sanitize_hex2(&s);
                Task::none()
            }
            Msg::BChanged(s) => {
                self.bb = sanitize_hex2(&s);
                Task::none()
            }

            Msg::OriginPicked(o) => {
                self.selected_origin = o;
                let names = self.filtered_names();
                self.selected_name = names.first().map(|s| (*s).to_string());
                Task::none()
            }

            Msg::WheelChanged(ch, v) => {
                let hh = format!("{v:02X}");
                match ch {
                    crate::messages::Channel::R => self.rr = hh,
                    crate::messages::Channel::G => self.gg = hh,
                    crate::messages::Channel::B => self.bb = hh,
                }
                Task::none()
            }

            // Use one variant; keep this if your TextInput sends SearchChanged
            Msg::SearchChanged(s) => {
                self.search = s;
                let names = self.filtered_names();
                self.selected_name = names.first().map(|s| (*s).to_string());
                Task::none()
            }

            // If you still emit these, keep them; otherwise delete the variants.
            Msg::QueryChanged(s) => {
                self.search = s;
                let names = self.filtered_names();
                self.selected_name = names.first().map(|s| (*s).to_string());
                Task::none()
            }

            Msg::PickedName(name) | Msg::PickChanged(name) => {
                self.selected_name = Some(name.clone());
                if let Some(hex) = self.hex_for_name_in_origin(&name) {
                    if let Some(rgb) = hex_to_rgb(hex) {
                        self.rr = format!("{:02X}", rgb.r);
                        self.gg = format!("{:02X}", rgb.g);
                        self.bb = format!("{:02X}", rgb.b);
                    }
                }
                Task::none()
            }

            Msg::CenterClicked => {
                let text = combine_hex(&self.rr, &self.gg, &self.bb);
                clipboard::write(text)
            }

            Msg::CopyHex(s) => clipboard::write(s),

            Msg::Clear => {
                self.rr.clear();
                self.gg.clear();
                self.bb.clear();
                self.search.clear();
                self.selected_name = None;
                self.status.clear();
                self.selected_origin = Origin::All;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Msg> {
        // --- helpers ---
        fn u8_from_hex2(s: &str) -> u8 {
            if s.len() == 2 {
                u8::from_str_radix(s, 16).unwrap_or(0)
            } else {
                0
            }
        }

        // --- wheel state ---
        let r = u8_from_hex2(&self.rr);
        let g = u8_from_hex2(&self.gg);
        let b = u8_from_hex2(&self.bb);

        let wheel = crate::widgets::color_wheel::ColorWheel::new(r, g, b, Msg::WheelChanged);
        let wheel_view = wheel.view("RGB Wheel", &self.rr, &self.gg, &self.bb);

        // --- choose slice ---
        let slice = colors_for_origin(self.selected_origin);

        // --- filtered names (Vec<String>) ---
        let mut filtered: Vec<String> = if self.search.trim().is_empty() {
            slice.iter().map(|&(_hex, name)| name.to_string()).collect()
        } else {
            let q = self.search.to_lowercase();
            slice
                .iter()
                .map(|&(_hex, name)| name.to_string())
                .filter(|n| n.to_lowercase().contains(&q))
                .collect()
        };
        filtered.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        let selected_opt: Option<String> = self.selected_name.as_ref().and_then(|cur| {
            filtered
                .iter()
                .find(|s| s.eq_ignore_ascii_case(cur))
                .cloned()
        });

        // --- Origins dropdown (kept compact) ---
        let origins_list = {
            let mut v = vec![
                Origin::All,
                Origin::XKCD,
                Origin::Pantone,
                Origin::Hindi,
                Origin::Persian,
            ];
            #[cfg(feature = "github-colors")]
            {
                v.push(Origin::GitHub);
            }
            v
        };
        let origin_dd = pick_list(origins_list, Some(self.selected_origin), Msg::OriginPicked)
            .placeholder("Origin")
            .width(Length::Shrink);

        // --- Stacked controls under the wheel, responsive widths ---
        // Use Fill within a narrow container so they shrink on small windows.
        let search_box = text_input("Search color name…", &self.search)
            .on_input(Msg::SearchChanged)
            .padding(6)
            .size(16)
            .width(Length::Fill);

        let name_dd = pick_list(
            filtered.clone(), // Vec<String>
            selected_opt,     // Option<String>
            Msg::PickedName,
        )
        .placeholder("Select a color")
        .width(Length::Fill);

        // Constrain controls to a nice max width but allow shrinking.
        let stacked_controls = container(
            column![search_box, name_dd]
                .spacing(8)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .padding([4, 8])
        .width(Length::Fill) // fill available
        .align_x(Alignment::Center); // center the column inside the container

        let clear_btn = iced::widget::button("Clear")
            .on_press(Msg::Clear)
            .padding([8, 12]);

        // Top/bottom bars keep things compact and centered.
        let content = column![
            text("RGB Wheel").size(24),
            // Center the wheel without forcing width; it will scale to the window.
            container(wheel_view)
                .width(Length::Fill)
                .align_x(Alignment::Center) // center the column inside the container
                .padding([4, 0]),
            // Input above dropdown, centered, shrink-friendly
            stacked_controls,
            // Bottom row with origin selector + clear button
            row![origin_dd, clear_btn]
                .spacing(10)
                .align_y(Alignment::Center)
                .width(Length::Shrink),
        ]
        .align_x(Alignment::Center)
        .spacing(12)
        .padding([8, 8]);

        // Wrap everything in a scrollable so short heights don’t clip.
        scrollable(
            container(content)
                .width(Length::Fill)
                .align_x(Alignment::Center), // center the column inside the container
        )
        .into()
    }
}

impl App {
    /// Filter names by origin *and* search, then sort alphabetically
    fn filtered_names(&self) -> Vec<&'static str> {
        let set = colors::colors_for(self.selected_origin);

        let mut names: Vec<&'static str> =
            set.as_slice().iter().map(|&(_hex, name)| name).collect();

        let q = self.search.trim().to_lowercase();
        if !q.is_empty() {
            names.retain(|nm| nm.to_lowercase().contains(&q));
        }

        names.sort_unstable_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
        names
    }

    /// Get HEX for a name, *restricted to the active origin*.
    fn hex_for_name_in_origin(&self, name: &str) -> Option<&'static str> {
        let set = colors::colors_for(self.selected_origin);
        set.as_slice()
            .iter()
            .find(|&&(_hex, nm)| nm.eq_ignore_ascii_case(name))
            .map(|&(hex, _)| hex)
    }

    #[allow(dead_code)]
    fn u8_from_hex2(s: &str) -> u8 {
        if s.len() == 2 {
            u8::from_str_radix(s, 16).unwrap_or(0)
        } else {
            0
        }
    }
}

pub fn colors_for_origin(origin: Origin) -> &'static [(&'static str, &'static str)] {
    match origin {
        Origin::All => COMBINED_COLORS.as_slice(),
        Origin::XKCD => COLORS_XKCD,
        Origin::Pantone => COLORS_PANTONE,
        Origin::Hindi => COLORS_HINDI,
        Origin::Persian => COLORS_PERSIAN,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => COLORS_GITHUB,
        Origin::Css => COLORS_XKCD,
    }
}
