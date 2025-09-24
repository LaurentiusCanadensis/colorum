use crate::colors_helper::{MAX_RESULTS, Origin};
use crate::ui::app_gui::App;
use crate::ui::messages::Msg;
use crate::core::rgb::{hex_to_rgb, rgb_to_hsl, Rgb, rgb_to_hex};
use iced::widget::{column, container, mouse_area, text, row, button, text_input, pick_list};
use iced::{Alignment, Background, Element, Length, Task, border, Color};
use palette::{Srgb, Lab, IntoColor, FromColor};

impl App {
    pub(crate) fn apply_selected_name(&mut self, name: &str) {
        self.selected_name = Some(name.to_string());
        if let Some(hex) = self.hex_for_name_in_origin(name) {
            if let Some(rgb) = hex_to_rgb(hex) {
                self.rr = format!("{:02X}", rgb.r);
                self.gg = format!("{:02X}", rgb.g);
                self.bb = format!("{:02X}", rgb.b);
            }
        }
    }

    pub(crate) fn view_dropdown(&self) -> iced::Element<'_, Msg> {
        use iced::border;
        use iced::widget::{Space, column, container, mouse_area, scrollable, text};
        use iced::{Alignment, Background, Color, Length};

        if self.results_idx.is_empty() {
            return Space::with_height(0).into();
        }

        let mut col = column![]
            .spacing(1)
            .padding(3)
            .align_x(Alignment::Start)
            .width(Length::Shrink);

        for (row, &idx) in self.results_idx.iter().enumerate() {
            let (hex, name) = self.base[idx];
            let is_sel = self.sel_pos == Some(row);
            let label = if is_sel {
                format!("▶ {}  {}", name.as_str(), hex.as_str())
            } else {
                format!("{}  {}", name.as_str(), hex.as_str())
            };

            let row_body = container(text(label))
                .padding([4, 6])
                .width(Length::Fill)
                .style(move |_theme: &iced::Theme| {
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
            .id(self.dropdown_scroll_id.clone())
            .height(Length::Fixed(150.0))
            .width(Length::Fill)
            .into()
    }

    pub fn filtered_names(&self) -> Vec<&'static str> {
        use crate::colors_helper::{
            HEAVY_MIN_QUERY, MAX_RESULTS, TokenMode, is_heavy_origin, origin_names, origin_rank,
            search_in_origin,
        };

        let q = self.query.trim();

        if q.is_empty() {
            // show full list when no query
            return origin_names(self.selected_origin).to_vec();
        }

        if is_heavy_origin(self.selected_origin) && q.len() < HEAVY_MIN_QUERY {
            // fallback: still show everything if user hasn't typed enough
            return origin_names(self.selected_origin).to_vec();
        }

        // normal search flow
        let mode = if q.contains(' ') {
            TokenMode::All
        } else {
            TokenMode::Any
        };
        let hits = search_in_origin(self.selected_origin, q, mode);

        let mut names: Vec<&'static str> = hits.into_iter().map(|(_hex, name)| name.as_str()).collect();
        let rank = origin_rank(self.selected_origin);
        names.sort_unstable_by_key(|n| rank.get(n).copied().unwrap_or(usize::MAX));

        if names.len() > MAX_RESULTS {
            names.truncate(MAX_RESULTS);
        }
        names
    }


    /// Filter names by origin *and* search, then sort alphabetically

    /// Get HEX for a name, *restricted to the active origin*.
    pub(crate) fn hex_for_name_in_origin(&self, name: &str) -> Option<&'static str> {
        let set = crate::colors_helper::origin_slice(self.selected_origin);
        set.iter()
            .find(|(_hex, nm)| nm.as_str().eq_ignore_ascii_case(name))
            .map(|(hex, _)| hex.as_str())
    }
}
#[allow(dead_code)]
fn u8_from_hex2(s: &str) -> u8 {
    if s.len() == 2 {
        u8::from_str_radix(s, 16).unwrap_or(0)
    } else {
        0
    }
}



impl App {
    /// Fill `results_idx` with all rows from current `base` and select the first row.
    pub fn repopulate_full_results(&mut self) {
        self.results_idx.clear();
        self.results_idx.extend(0..self.base.len());
        self.sel_pos = if self.base.is_empty() { None } else { Some(0) };
        self.dropdown_open = true;
    }
    pub fn scroll_to_selected(&self) -> Task<Msg> {
        // Must match your dropdown widget:
        const VIEWPORT_H: f32 = 220.0; // .height(Length::Fixed(220.0))
        const ROW_H: f32 = 30.0; // ~ text + padding; tweak 28–32 if needed

        let len = self.results_idx.len();
        if len == 0 {
            return Task::none();
        }

        // Keep 1 row of margin so the selection never hides under the top edge
        let sel = self.sel_pos.unwrap_or(0).min(len - 1);
        let target_row = sel.saturating_sub(1);

        let content_h = (len as f32) * ROW_H;
        let max_scroll = (content_h - VIEWPORT_H).max(0.0);
        let desired_y = (target_row as f32) * ROW_H;

        let rel_y = if max_scroll > 0.0 {
            (desired_y / max_scroll).clamp(0.0, 1.0)
        } else {
            0.0
        };

        iced::widget::scrollable::snap_to(
            self.dropdown_scroll_id.clone(),
            iced::widget::scrollable::RelativeOffset { x: 0.0, y: rel_y },
        )
    }


    pub fn move_selection(&mut self, delta: i32) {
        let len = self.results_idx.len() as i32;
        if len == 0 {
            self.sel_pos = None;
            return;
        }
        let cur = self.sel_pos.unwrap_or(0) as i32;
        let next = (cur + delta).rem_euclid(len) as usize;
        self.sel_pos = Some(next);
    }

    /// Apply the currently selected row to the UI (wheel, fields, selected name)
    pub(crate) fn activate_selected(&mut self) {
        let Some(row) = self.sel_pos else {
            return;
        };
        if row >= self.results_idx.len() {
            return;
        }
        eprintln!("Enter pressed {:?}-> Self at this point is ", self.query);

        let idx = self.results_idx[row];
        let (hex, name) = self.base[idx];

        if let Some(rgb) = crate::core::rgb::hex_to_rgb(hex.as_str()) {
            self.rr = format!("{:02X}", rgb.r);
            self.gg = format!("{:02X}", rgb.g);
            self.bb = format!("{:02X}", rgb.b);
        }
        self.selected_name = Some(name.to_string());
    }

    /// Set rr/gg/bb from a hex string like "#61B3E4" or "61B3E4".
    pub(crate) fn set_from_hex(&mut self, hex: &str) {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() == 6 {
            if let Ok(r) = u8::from_str_radix(&clean[0..2], 16) {
                if let Ok(g) = u8::from_str_radix(&clean[2..4], 16) {
                    if let Ok(b) = u8::from_str_radix(&clean[4..6], 16) {
                        self.rr = format!("{r:02X}");
                        self.gg = format!("{g:02X}");
                        self.bb = format!("{b:02X}");
                    }
                }
            }
        }
    }

    /// Apply the color at `results_idx[row]` to the wheel + selection.
    pub(crate) fn select_row(&mut self, row: usize) {
        if row < self.results_idx.len() {
            self.sel_pos = Some(row);
            let idx = self.results_idx[row];
            let (hex, name) = self.base[idx];
            self.selected_name = Some(name.to_string());
            self.set_from_hex(hex.as_str());
        }
    }
    pub(crate) fn repopulate_full_results_capped(&mut self) {
        let n = self.base.len().min(MAX_RESULTS);
        self.results_idx.clear();
        self.results_idx.reserve(n);
        self.results_idx.extend(0..n);
        self.sel_pos = if n > 0 { Some(0) } else { None };
    }

    /// Check if the current RGB values match the selected color name
    pub(crate) fn current_color_matches_selected_name(&self) -> bool {
        let Some(ref selected_name) = self.selected_name else {
            return false;
        };

        let Some(hex) = self.hex_for_name_in_origin(selected_name) else {
            return false;
        };

        // Convert current RGB to hex and compare
        let current_hex = crate::core::hex::combine_hex(&self.rr, &self.gg, &self.bb);
        let normalized_current = current_hex.trim_start_matches('#').to_uppercase();
        let normalized_selected = hex.trim_start_matches('#').to_uppercase();

        normalized_current == normalized_selected
    }

    /// Clear the selected name if the current color doesn't match it
    pub(crate) fn clear_name_if_color_mismatch(&mut self) {
        if !self.current_color_matches_selected_name() {
            self.selected_name = None;
        }
    }

    /// Add a color to the recently used history
    /// - Removes duplicates if the color already exists
    /// - Adds to front of queue
    /// - Maintains max 10 colors (FIFO)
    pub(crate) fn add_to_color_history(&mut self, hex: &str) {
        // Normalize the hex color (ensure it starts with #)
        let normalized_hex = if hex.starts_with('#') {
            hex.to_string()
        } else {
            format!("#{}", hex)
        };

        // Remove existing instance if present
        if let Some(pos) = self.color_history.iter().position(|h| h == &normalized_hex) {
            self.color_history.remove(pos);
        }

        // Add to front
        self.color_history.push_front(normalized_hex);

        // Maintain max 10 colors
        while self.color_history.len() > 10 {
            self.color_history.pop_back();
        }
    }

    /// Create the recently used colors panel view
    pub(crate) fn view_recently_used_colors(&self) -> Option<Element<'_, Msg>> {
        // Only show if there are colors in history
        if self.color_history.is_empty() {
            return None;
        }

        let mut row = iced::widget::Row::new()
            .spacing(2)
            .align_y(iced::Alignment::Center);

        // Add label
        row = row.push(
            iced::widget::text("Recent:")
                .size(16)
                .color(iced::Color::from_rgb(0.5, 0.5, 0.5))
        );

        // Add color swatches
        for hex in self.color_history.iter() {
            let swatch = self.create_color_swatch(hex);
            row = row.push(swatch);
        }

        Some(
            container(row)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding([2, 0])
                .into()
        )
    }

    /// Create a single color swatch for the history panel
    fn create_color_swatch(&self, hex: &str) -> Element<'_, Msg> {
        // Parse hex to RGB for background color
        let background_color = if let Some(rgb) = hex_to_rgb(hex) {
            iced::Color::from_rgb8(rgb.r, rgb.g, rgb.b)
        } else {
            iced::Color::BLACK
        };

        // Create a clickable color square
        let swatch_size = 30.0;
        let swatch_content = iced::widget::container(
            iced::widget::Space::with_width(Length::Fixed(swatch_size))
        )
        .width(Length::Fixed(swatch_size))
        .height(Length::Fixed(swatch_size))
        .style(move |_theme: &iced::Theme| {
            iced::widget::container::Style {
                background: Some(Background::Color(background_color)),
                border: border::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                },
                ..Default::default()
            }
        });

        // Make it clickable
        mouse_area(swatch_content)
            .on_press(Msg::SelectFromHistory(hex.to_string()))
            .into()
    }

    pub(crate) fn view_color_analytics(&self) -> Element<'_, Msg> {
        self.view_color_analytics_with_width(300.0)
    }

    pub(crate) fn view_color_analytics_with_width(&self, width: f32) -> Element<'_, Msg> {
        fn u8_from_hex2(s: &str) -> u8 {
            if s.len() == 2 {
                u8::from_str_radix(s, 16).unwrap_or(0)
            } else {
                0
            }
        }

        let r = u8_from_hex2(&self.rr);
        let g = u8_from_hex2(&self.gg);
        let b = u8_from_hex2(&self.bb);

        let _hsl = rgb_to_hsl(Rgb { r, g, b });

        // Try to get exact name first, then find closest name from COMBINED_NAMES
        let exact_name = crate::core::hex::name_for_hex(rgb_to_hex(Rgb { r, g, b }));

        let closest_name = if exact_name.is_none() {
            // Find closest color name from COMBINED_NAMES
            crate::colors_helper::find_closest_color_name(Rgb { r, g, b })
        } else {
            exact_name
        };

        let color_name = self.selected_name.as_deref()
            .or(closest_name)
            .unwrap_or("Unnamed Color");

        // Calculate color distance using CIEDE2000 in Lab color space
        let current_rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let current_lab: Lab = current_rgb.into_color();

        // Calculate distance from white (as a reference point)
        let white_rgb = Srgb::new(1.0, 1.0, 1.0);
        let white_lab: Lab = white_rgb.into_color();

        // Simple Euclidean distance in Lab space
        let distance = ((current_lab.l - white_lab.l).powi(2) +
                       (current_lab.a - white_lab.a).powi(2) +
                       (current_lab.b - white_lab.b).powi(2)).sqrt();

        let analytics_content = column![]
            .spacing(6)
            .padding([10, 12])
            .push(
                text("Color Analytics")
                    .size(18)
                    .color(Color::from_rgb(0.1, 0.1, 0.1))
            )
            .push(
                // Color name row
                row![]
                    .spacing(3)
                    .push(
                        button(
                            text(color_name)
                                .size(16)
                                .color(Color::from_rgb(0.2, 0.2, 0.2))
                        )
                        .padding([4, 8])
                        .on_press(Msg::CopyHex(color_name.to_string()))
                        .style(|_theme: &iced::Theme, _status| {
                            iced::widget::button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.85, 0.85, 0.85))),
                                border: border::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                                },
                                text_color: Color::from_rgb(0.2, 0.2, 0.2),
                                ..Default::default()
                            }
                        })
                    )
            )
            .push(
                // Hex color row
                row![]
                    .spacing(3)
                    .push(
                        button(
                            text(format!("#{:02X}{:02X}{:02X}", r, g, b))
                                .size(16)
                                .color(Color::from_rgb(0.2, 0.2, 0.2))
                        )
                        .padding([4, 8])
                        .on_press(Msg::CopyHex(format!("#{:02X}{:02X}{:02X}", r, g, b)))
                        .style(|_theme: &iced::Theme, _status| {
                            iced::widget::button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.85, 0.85, 0.85))),
                                border: border::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                                },
                                text_color: Color::from_rgb(0.2, 0.2, 0.2),
                                ..Default::default()
                            }
                        })
                    )
            )
            .push(
                // Color distance row
                row![]
                    .spacing(3)
                    .push(
                        button(
                            text(format!("Distance: {:.1}", distance))
                                .size(16)
                                .color(Color::from_rgb(0.2, 0.2, 0.2))
                        )
                        .padding([4, 8])
                        .on_press(Msg::CopyHex(format!("{:.1}", distance)))
                        .style(|_theme: &iced::Theme, _status| {
                            iced::widget::button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.85, 0.85, 0.85))),
                                border: border::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                                },
                                text_color: Color::from_rgb(0.2, 0.2, 0.2),
                                ..Default::default()
                            }
                        })
                    )
            );

        container(analytics_content)
            .style(|_theme: &iced::Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                    border: border::rounded(8),
                    ..Default::default()
                }
            })
            .width(Length::Fixed(width))
            .into()
    }

    pub(crate) fn view_search_block(&self) -> Element<'_, Msg> {
        self.view_search_block_with_width(300.0)
    }

    pub(crate) fn view_search_block_with_width(&self, width: f32) -> Element<'_, Msg> {
        let origins_list = origins_vec();

        // Create the search UI elements
        let create_search_elements = || {
            let origin_dd = pick_list(
                origins_list.clone(),
                Some(self.selected_origin),
                Msg::OriginPicked,
            )
            .placeholder("Origin")
            .width(Length::Fill);

            let clear_btn = button("Clear")
                .on_press(Msg::Clear)
                .padding([2, 4]);

            let search_box = text_input("Search color name…", &self.query)
                .id(self.search_input_id.clone())
                .on_input(Msg::QueryChanged)
                .on_submit(Msg::PressedEnter)
                .padding(3)
                .width(Length::Fill);

            let search_row = row![]
                .push(search_box)
                .push(clear_btn)
                .spacing(3)
                .align_y(Alignment::Center);

            (origin_dd, search_row)
        };

        // Build the final content based on whether dropdown is open
        let final_content = if self.dropdown_open && !self.results_idx.is_empty() {
            let (origin_dd, search_row) = create_search_elements();
            let dropdown = self.view_dropdown();

            column![]
                .spacing(4)
                .padding([8, 10])
                .push(
                    text("Search")
                        .size(18)
                        //.color(Color::from_rgb(0.1, 0.1, 0.1))
                )
                .push(
                    container(
                        column![]
                            .spacing(4)
                            .push(origin_dd)
                            .push(search_row)
                            .push(
                                container(dropdown)
                                    .style(|_theme: &iced::Theme| {
                                        iced::widget::container::Style {
                                           // background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                                            border: border::Border {
                                                radius: 4.0.into(),
                                                width: 1.0,
                                                color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                                            },
                                            ..Default::default()
                                        }
                                    })
                                    .padding([2, 2])
                            )
                    )
                    .style(|_theme: &iced::Theme| {
                        iced::widget::container::Style {
                            //background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                            border: border::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                            },
                            ..Default::default()
                        }
                    })
                    .padding([4, 6])
                )
        } else {
            let (origin_dd, search_row) = create_search_elements();

            column![]
                .spacing(4)
                .padding([8, 10])
                .push(
                    text("Search")
                        .size(18)
                        .color(Color::from_rgb(0.1, 0.1, 0.1))
                )
                .push(
                    container(
                        column![]
                            .spacing(4)
                            .push(origin_dd)
                            .push(search_row)
                    )
                    .style(|_theme: &iced::Theme| {
                        iced::widget::container::Style {
                            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                            border: border::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                            },
                            ..Default::default()
                        }
                    })
                    .padding([4, 6])
                )
        };

        container(final_content)
            .style(|_theme: &iced::Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                    border: border::rounded(8),
                    ..Default::default()
                }
            })
            .width(Length::Fixed(width))
            .into()
    }

}

pub fn origins_vec() -> Vec<Origin> {
    vec![
        Origin::All,
        Origin::Css,
        Origin::XKCD,
        Origin::Pantone,
        Origin::Hindi,
        Origin::Persian,
        Origin::National,
        Origin::Brands,
        Origin::ItalianBrands,
        Origin::MetalFlames,
        Origin::KelvinColors,
        #[cfg(feature = "github-colors")]
        Origin::GitHub,
        Origin::Seasons,
        Origin::CanadianProvinces,
    ]
}