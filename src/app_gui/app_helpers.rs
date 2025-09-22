use crate::colors_helper::{
    self, MAX_RESULTS, Origin,
};
use crate::app_gui::App;
use iced::keyboard::{self, Event as KEvent, Key, key::Named};
use iced::{Event, Subscription};

use crate::hex::{combine_hex, sanitize_hex2};
use crate::messages::Msg;
use crate::rgb::hex_to_rgb;
use iced::widget::{
    PickList, Space, button, column, container, mouse_area, pick_list, row, scrollable, text,
    text_input,
};
use iced::{
    Alignment, Background, Color, Element, Length, Renderer, Task, Theme, border, clipboard,
};

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
                format!("▶ {}  {}", name, hex)
            } else {
                format!("{}  {}", name, hex)
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
            .height(Length::Fixed(220.0))
            .width(Length::Fill)
            .into()
    }

    pub fn filtered_names(&self) -> Vec<&'static str> {
        use crate::colors_helper::{
            HEAVY_MIN_QUERY, MAX_RESULTS, TokenMode, is_heavy_origin, origin_names, origin_rank,
            search_in_origin,
        };

        let q = self.search.trim();

        if q.is_empty() {
            // show full list when no query
            return origin_names(self.selected_origin).to_vec();
        }

        if is_heavy_origin(self.selected_origin) && q.len() < HEAVY_MIN_QUERY {
            // fallback: still show everything if user hasn’t typed enough
            return origin_names(self.selected_origin).to_vec();
        }

        // normal search flow
        let mode = if q.contains(' ') {
            TokenMode::All
        } else {
            TokenMode::Any
        };
        let hits = search_in_origin(self.selected_origin, q, mode);

        let mut names: Vec<&'static str> = hits.into_iter().map(|(_hex, name)| name).collect();
        let rank = origin_rank(self.selected_origin);
        names.sort_unstable_by_key(|n| rank.get(n).copied().unwrap_or(usize::MAX));

        if names.len() > MAX_RESULTS {
            names.truncate(MAX_RESULTS);
        }
        names
    }

    pub(crate) fn filtered_names_old(&self) -> Vec<&'static str> {
        use crate::colors_helper::{
            HEAVY_MIN_QUERY, MAX_RESULTS, is_heavy_origin, origin_names, origin_rank,
        };
        use crate::colors_helper::{TokenMode, search_in_origin};

        let q = self.search.trim();

        // HEAVY ORIGINS: when query is empty (or too short), return empty list to keep dropdown light
        if is_heavy_origin(self.selected_origin) {
            if q.len() < HEAVY_MIN_QUERY {
                return Vec::new();
            }
        } else {
            // LIGHT ORIGINS: empty query → precomputed, already sorted
            if q.is_empty() {
                return origin_names(self.selected_origin).to_vec(); // pointer copies only
            }
        }

        // Non-empty query → use token index
        let mode = if q.contains(' ') {
            TokenMode::All
        } else {
            TokenMode::Any
        };
        let hits = search_in_origin(self.selected_origin, q, mode);

        // Keep names only
        let mut names: Vec<&'static str> = hits.into_iter().map(|(_hex, name)| name).collect();

        // Order by precomputed rank (no per-keystroke lowercase cost)
        let rank = origin_rank(self.selected_origin);
        names.sort_unstable_by_key(|n| rank.get(n).copied().unwrap_or(usize::MAX));

        // Cap results so pick_list stays fast
        if names.len() > MAX_RESULTS {
            names.truncate(MAX_RESULTS);
        }

        names
    }

    /// Filter names by origin *and* search, then sort alphabetically

    /// Get HEX for a name, *restricted to the active origin*.
    pub(crate) fn hex_for_name_in_origin(&self, name: &str) -> Option<&'static str> {
        let set = colors_helper::colors_for(self.selected_origin);
        set.iter()
            .find(|&&(_hex, nm)| nm.eq_ignore_ascii_case(name))
            .map(|&(hex, _)| hex)
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

pub fn colors_for_origin(origin: Origin) -> &'static [(&'static str, &'static str)] {
    crate::colors_helper::colors_for(origin)
}

// Use centralized lookup functions from colors_helper

// Use centralized search functions from colors_helper

// Duplicate search functions removed - use colors_helper module instead


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

    /// Rebuild the `base` list & index for the current origin.
    pub fn reindex_origin(&mut self) {
        self.base = crate::colors_helper::colors_for(self.selected_origin).to_vec();

        self.base_index_by_name.clear();
        self.base_index_by_name.reserve(self.base.len());
        for (i, &(_h, n)) in self.base.iter().enumerate() {
            self.base_index_by_name.insert(n, i);
        }

        // Reset results & cursor (query will repopulate)
        self.results_idx.clear();
        self.sel_pos = None;
    }

    /// Refilter into `results_idx` using the current query (index only).
    pub fn rebuild_results(&mut self) {
        let q = self.query.trim();
        if q.is_empty() {
            self.results_idx.clear();
            self.sel_pos = None;
            return;
        }

        // Use your fast ranked search, then map names to indices in `base`.
        let hits = crate::colors_helper::search_in_origin(
            self.selected_origin,
            q,
            crate::colors_helper::TokenMode::Any,
        );

        self.results_idx.clear();
        self.results_idx.reserve(hits.len());

        for &(_hex, name) in hits.iter() {
            if let Some(&i) = self.base_index_by_name.get(name) {
                self.results_idx.push(i);
            }
        }

        self.sel_pos = if self.results_idx.is_empty() {
            None
        } else {
            Some(0)
        };
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

        if let Some(rgb) = crate::rgb::hex_to_rgb(hex) {
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
            self.set_from_hex(hex);
        }
    }
    pub(crate) fn repopulate_full_results_capped(&mut self) {
        let n = self.base.len().min(MAX_RESULTS);
        self.results_idx.clear();
        self.results_idx.reserve(n);
        self.results_idx.extend(0..n);
        self.sel_pos = if n > 0 { Some(0) } else { None };
    }



}

// Removed duplicate ColorCatalog - use the one from colors_helper


// Use centralized registry from colors_helper module

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
        // New simplified palette system
        Origin::Seasons,
        Origin::CanadianProvinces,
    ]
}

pub const HEAVY_MIN_QUERY: usize = 2;

pub fn is_heavy_origin(o: Origin) -> bool {
    match o {
        Origin::All => true,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => true,
        _ => false,
    }
}