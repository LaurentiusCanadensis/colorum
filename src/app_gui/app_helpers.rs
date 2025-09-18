
use std::collections::HashMap;
use std::sync::LazyLock;
// src/app_gui.rs
use crate::colors_helper::{self, COLORS_GITHUB, COLORS_HINDI, COLORS_NATIONAL, COLORS_PANTONE, COLORS_PERSIAN, COLORS_XKCD, COMBINED_COLORS, Origin};
use crate::hex::{combine_hex, sanitize_hex2};
use crate::messages::Msg;
use crate::rgb::hex_to_rgb;
use iced::widget::{PickList, button, column, container, pick_list, row, scrollable, text, text_input, Space, mouse_area};
use iced::{Alignment, Background, Color, Element, Length, Renderer, Task, Theme, clipboard, border};

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

    fn view_dropdown(&self) -> iced::Element<Msg> {
        use iced::widget::{column, container, scrollable, text, mouse_area, Space};
        use iced::{Alignment, Background, Color, Length};
        use iced::border;

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
                            border: border::Border { radius: 8.0.into(), ..Default::default() },
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


    pub(crate) fn filtered_names(&self) -> Vec<&'static str> {
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
    fn hex_for_name_in_origin(&self, name: &str) -> Option<&'static str> {
        let set = colors_helper::colors_for(self.selected_origin);
        set.as_slice()
            .iter()
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
    match origin {
        Origin::All => COMBINED_COLORS.as_slice(),
        Origin::XKCD => COLORS_XKCD,
        Origin::Pantone => COLORS_PANTONE,
        Origin::Hindi => COLORS_HINDI,
        Origin::Persian => COLORS_PERSIAN,
        #[cfg(feature = "github-colors")]
        Origin::GitHub => COLORS_GITHUB,
        Origin::Css => COLORS_XKCD,
        Origin::National => COLORS_NATIONAL.as_slice(),
    }
}

// ---------- A) Exact name maps & pre-lowercased cache ----------
pub static COLORS_BY_NAME: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    COMBINED_COLORS
        .iter()
        .map(|(hex, name)| (*name, *hex))
        .collect()
});

pub static COLORS_BY_NAME_LC: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (hex, name) in COMBINED_COLORS.as_slice() {
        m.insert(name.to_lowercase(), *hex);
    }
    m
});

pub struct ColorEntry {
    pub hex: &'static str,
    pub name: &'static str,
    pub name_lc: String, // built once
}

pub static COLORS_LC: LazyLock<Vec<ColorEntry>> = LazyLock::new(|| {
    COMBINED_COLORS
        .iter()
        .map(|(hex, name)| ColorEntry {
            hex: *hex,
            name: *name,
            name_lc: name.to_lowercase(),
        })
        .collect()
});

// ---------- Tokenization ----------
fn tokenize_lc(s: &str) -> impl Iterator<Item=String> + '_ {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
}

// ---------- B) Token index (token -> sorted, deduped posting list) ----------
// NOTE: store compact Box<[usize]> and pre-sort/dedup once.
pub static NAME_TOKEN_INDEX: LazyLock<HashMap<String, Box<[usize]>>> = LazyLock::new(|| {
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, (_, name)) in COMBINED_COLORS.iter().enumerate() {
        for tok in tokenize_lc(name) {
            idx.entry(tok).or_default().push(i);
        }
    }

    // Sort, dedup, shrink, then convert to Box<[usize]>
    idx.into_iter()
        .map(|(tok, mut v)| {
            v.sort_unstable();
            v.dedup();
            v.shrink_to_fit();
            (tok, v.into_boxed_slice())
        })
        .collect()
});

// ---------- Lookups ----------
pub fn lookup_by_name(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME.get(name).copied()
}

pub fn lookup_by_name_ci(name: &str) -> Option<&'static str> {
    COLORS_BY_NAME_LC.get(&name.to_lowercase()).copied()
}

// Fast substring search (case-insensitive, O(n) but no per-item lowercase)
pub fn search_substring(query: &str) -> Vec<(&'static str, &'static str)> {
    let q = query.to_lowercase();
    COLORS_LC
        .iter()
        .filter(|e| e.name_lc.contains(&q))
        .map(|e| (e.hex, e.name))
        .collect()
}

// ---------- Helpers for sorted postings ----------
#[inline]
fn intersect_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut out = Vec::with_capacity(a.len().min(b.len()));

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}

#[inline]
fn union_sorted_slices(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    // Upper bound is a.len() + b.len()
    let mut out = Vec::with_capacity(a.len() + b.len());

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                out.push(a[i]); // same value once
                i += 1;
                j += 1;
            }
        }
    }
    if i < a.len() {
        out.extend_from_slice(&a[i..]);
    }
    if j < b.len() {
        out.extend_from_slice(&b[j..]);
    }
    out
}

// ---------- Token search: ANY (OR) semantics, k-way union over sorted postings ----------
pub fn search_tokens_any(query: &str) -> Vec<(&'static str, &'static str)> {
    let mut postings: Vec<&[usize]> = Vec::new();

    for tok in tokenize_lc(query) {
        if let Some(list) = NAME_TOKEN_INDEX.get(&tok) {
            postings.push(list);
        }
    }

    if postings.is_empty() {
        return Vec::new();
    }

    // Sort by length to improve merge locality
    postings.sort_by_key(|p| p.len());

    // Iteratively union the sorted lists
    let mut result: Vec<usize> = postings[0].to_vec();
    for p in postings.iter().skip(1) {
        result = union_sorted_slices(&result, p);
    }

    result.into_iter().map(|i| COMBINED_COLORS[i]).collect()
}

// ---------- Token search: ALL (AND) semantics, intersect smallest lists first ----------
pub fn search_tokens_all(query: &str) -> Vec<(&'static str, &'static str)> {
    // Collect tokens and fetch postings
    let mut lists: Vec<&[usize]> = {
        let mut tmp = Vec::new();
        for tok in tokenize_lc(query) {
            match NAME_TOKEN_INDEX.get(&tok) {
                Some(list) => tmp.push(list.as_ref()),
                None => return Vec::new(), // missing token => no matches
            }
        }
        if tmp.is_empty() {
            return Vec::new();
        }
        tmp
    };

    // Intersect smallest first to keep intermediate sets tiny
    lists.sort_by_key(|p| p.len());

    let mut current: Vec<usize> = lists[0].to_vec();
    for p in lists.iter().skip(1) {
        if current.is_empty() {
            break;
        }
        current = intersect_sorted_slices(&current, p);
    }

    current.into_iter().map(|i| COMBINED_COLORS[i]).collect()
}

use iced::advanced::subscription;
use iced::keyboard::{self, Event as KEvent, Key, key::Named};
use iced::{Event, Subscription};
use crate::app_gui::App;

impl App {
    // add near your other imports in app_gui.rs

    // then in Application::subscription():
    pub fn subscription(&self) -> iced::Subscription<Msg> {
        iced::keyboard::on_key_press(|key, _mods| match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => Some(Msg::KeyPressed(key)),
            _ => None,
        })
    }
}

impl App {


    pub fn scroll_to_selected(&self) -> Task<Msg> {
        // Must match your dropdown widget:
        const VIEWPORT_H: f32 = 220.0; // .height(Length::Fixed(220.0))
        const ROW_H: f32     = 30.0;   // ~ text + padding; tweak 28–32 if needed

        let len = self.results_idx.len();
        if len == 0 {
            return Task::none();
        }

        // Keep 1 row of margin so the selection never hides under the top edge
        let sel = self.sel_pos.unwrap_or(0).min(len - 1);
        let target_row = sel.saturating_sub(1);

        let content_h  = (len as f32) * ROW_H;
        let max_scroll = (content_h - VIEWPORT_H).max(0.0);
        let desired_y  = (target_row as f32) * ROW_H;

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

        self.sel_pos = if self.results_idx.is_empty() { None } else { Some(0) };
    }

    pub fn move_selection(&mut self, delta: i32) {
        let len = self.results_idx.len() as i32;
        if len == 0 { self.sel_pos = None; return; }
        let cur  = self.sel_pos.unwrap_or(0) as i32;
        let next = (cur + delta).rem_euclid(len) as usize;
        self.sel_pos = Some(next);
    }

    /// Apply the currently selected row to the UI (wheel, fields, selected name)
    pub(crate) fn activate_selected(&mut self) {
        let Some(row) = self.sel_pos else { return; };
        if row >= self.results_idx.len() { return; }
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
}
