use crate::colors_helper::{MAX_RESULTS, Origin};
use crate::ui::app_gui::App;
use crate::ui::messages::Msg;
use crate::core::rgb::hex_to_rgb;
use iced::widget::{column, container, mouse_area, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Task, border};

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

    pub(crate) fn view_dropdown(&self) -> iced::Element<Msg> {
        use iced::border;
        use iced::widget::{Space, column, container, mouse_area, scrollable, text};
        use iced::{Alignment, Background, Color, Length};

        if self.results_idx.is_empty() {
            return Space::with_height(0).into();
        }

        let mut col = column![]
            .spacing(2)
            .padding(4)
            .align_x(Alignment::Start)
            .width(Length::Fill);

        for (row, &idx) in self.results_idx.iter().enumerate() {
            let (hex, name) = self.base[idx];
            let is_sel = self.sel_pos == Some(row);
            let label = if is_sel {
                format!("▶ {}  {}", name.as_str(), hex.as_str())
            } else {
                format!("{}  {}", name.as_str(), hex.as_str())
            };

            let row_body = container(text(label))
                .padding([6, 8])
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
            .height(Length::Fixed(320.0))  // Increased from 220 to 320 for more results
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
        let sel = self.sel_pos.unwrap_or(0).min(len.saturating_sub(1));
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

    /// Create a color analytics block showing closest colors, contrast ratios, and color information
    pub(crate) fn view_color_analytics_with_width(&self, width: Length) -> Element<Msg> {
        use iced::widget::{button, column, container, text};
        use iced::{Color, Length};

        // Get current RGB values
        let r = u8::from_str_radix(&self.rr, 16).unwrap_or(0);
        let g = u8::from_str_radix(&self.gg, 16).unwrap_or(0);
        let b = u8::from_str_radix(&self.bb, 16).unwrap_or(0);

        let current_hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        // Convert to other color spaces
        let hsl = rgb_to_hsl(r, g, b);
        let cmyk = rgb_to_cmyk(r, g, b);

        // Find closest color
        let closest_color = self.get_closest_color_name();

        // Calculate contrast ratios
        let white_contrast = contrast_ratio((r, g, b), (255, 255, 255));
        let black_contrast = contrast_ratio((r, g, b), (0, 0, 0));

        let mut analytics_column = column![]
            .spacing(2)
            .padding(6)
            .width(width);

        // Color info section
        analytics_column = analytics_column
            .push(text("Color Analytics").size(15))
            .push(iced::widget::Space::with_height(Length::Fixed(2.0)))
            .push(text(format!("Window: {}×{}", self.window_width as u32, self.window_height as u32)).size(10).color(iced::Color::from_rgb(0.5, 0.5, 0.5)));

        // Closest color section FIRST (clickable)
        if let Some(closest) = closest_color {
            let closest_button = button(text(format!("{}", closest)).size(11))
                .on_press(Msg::CopyHex(closest.to_string()))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: None,
                    text_color: Color::from_rgb(0.2, 0.4, 0.8),
                    border: iced::border::Border::default(),
                    shadow: iced::Shadow::default(),
                })
                .padding([1, 3]);

            analytics_column = analytics_column
                .push(iced::widget::Space::with_height(Length::Fixed(4.0)))
                .push(text("Closest Color").size(13).font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }))
                .push(closest_button);
        }

        // HEX section (clickable)
        let hex_button = button(text(format!("{}", current_hex)).size(11))
            .on_press(Msg::CopyHex(current_hex.clone()))
            .style(|_theme, _status| iced::widget::button::Style {
                background: None,
                text_color: Color::from_rgb(0.2, 0.4, 0.8),
                border: iced::border::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .padding([1, 3]);

        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(3.0)))
            .push(text("HEX").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(hex_button);

        // RGB section
        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(3.0)))
            .push(text("RGB").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(text(format!("{}, {}, {}", r, g, b)).size(11));

        // HSL section
        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(3.0)))
            .push(text("HSL").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(text(format!("{}°, {}%, {}%", hsl.0, hsl.1, hsl.2)).size(11));

        // CMYK section
        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(3.0)))
            .push(text("CMYK").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(text(format!("{}%, {}%, {}%, {}%", cmyk.0, cmyk.1, cmyk.2, cmyk.3)).size(11));

        // Contrast ratios section - more compact
        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(4.0)))
            .push(text("Contrast").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(text(format!("White: {:.1} | Black: {:.1}", white_contrast, black_contrast)).size(11));

        // WCAG compliance section - more compact
        let wcag_aa = white_contrast >= 4.5 || black_contrast >= 4.5;
        let wcag_aaa = white_contrast >= 7.0 || black_contrast >= 7.0;

        analytics_column = analytics_column
            .push(iced::widget::Space::with_height(Length::Fixed(2.0)))
            .push(text("WCAG").size(13).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }))
            .push(text(format!("AA: {} | AAA: {}",
                if wcag_aa { "✓" } else { "✗" },
                if wcag_aaa { "✓" } else { "✗" }
            )).size(11));

        container(analytics_column)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(Color {
                    r: 0.95,
                    g: 0.95,
                    b: 0.95,
                    a: 0.8,
                })),
                border: iced::border::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: Color {
                        r: 0.8,
                        g: 0.8,
                        b: 0.8,
                        a: 1.0,
                    },
                },
                ..Default::default()
            })
            .into()
    }

    /// Get the closest color name from the current color palette
    fn get_closest_color_name(&self) -> Option<&'static str> {
        let r = u8::from_str_radix(&self.rr, 16).unwrap_or(0);
        let g = u8::from_str_radix(&self.gg, 16).unwrap_or(0);
        let b = u8::from_str_radix(&self.bb, 16).unwrap_or(0);

        let colors = crate::colors_helper::origin_slice(self.selected_origin);
        let mut closest_name = None;
        let mut min_distance = f64::MAX;

        for (hex, name) in colors {
            if let Some(rgb) = crate::core::rgb::hex_to_rgb(hex.as_str()) {
                let distance = color_distance((r, g, b), (rgb.r, rgb.g, rgb.b));
                if distance < min_distance {
                    min_distance = distance;
                    closest_name = Some(name.as_str());
                }
            }
        }

        closest_name
    }


}

// Helper functions for color analytics
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    if delta == 0.0 {
        return (0, 0, (lightness * 100.0) as u8);
    }

    let saturation = if lightness < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let hue = if max == r {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    (
        ((hue * 60.0) as u16) % 360,
        (saturation * 100.0) as u8,
        (lightness * 100.0) as u8,
    )
}

fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let k = 1.0 - r.max(g.max(b));
    if k == 1.0 {
        return (0, 0, 0, 100);
    }

    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    (
        (c * 100.0) as u8,
        (m * 100.0) as u8,
        (y * 100.0) as u8,
        (k * 100.0) as u8,
    )
}

fn contrast_ratio(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
    let l1 = relative_luminance(color1);
    let l2 = relative_luminance(color2);

    let lighter = l1.max(l2);
    let darker = l1.min(l2);

    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: (u8, u8, u8)) -> f64 {
    let r = srgb_to_linear(color.0 as f64 / 255.0);
    let g = srgb_to_linear(color.1 as f64 / 255.0);
    let b = srgb_to_linear(color.2 as f64 / 255.0);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn color_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
    let dr = color1.0 as f64 - color2.0 as f64;
    let dg = color1.1 as f64 - color2.1 as f64;
    let db = color1.2 as f64 - color2.2 as f64;

    (dr * dr + dg * dg + db * db).sqrt()
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