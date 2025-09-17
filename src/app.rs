// src/app.rs
use crate::colors_helper::{self, COLORS_PANTONE, COMBINED_COLORS, Origin};
use crate::hex::{combine_hex, sanitize_hex2};
use crate::messages::Msg;
use crate::rgb::hex_to_rgb;
use iced::widget::{
    PickList, button, column, container, pick_list, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length, Renderer, Task, Theme, clipboard};

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
    fn apply_selected_name(&mut self, name: &str) {
        self.selected_name = Some(name.to_string());
        if let Some(hex) = self.hex_for_name_in_origin(name) {
            if let Some(rgb) = hex_to_rgb(hex) {
                self.rr = format!("{:02X}", rgb.r);
                self.gg = format!("{:02X}", rgb.g);
                self.bb = format!("{:02X}", rgb.b);
            }
        }
    }
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
            Msg::OriginPicked(o) => {
                self.selected_origin = o;
                let names = self.filtered_names();
                if let Some(first) = names.first() {
                    self.apply_selected_name(first);
                } else {
                    // nothing to select (e.g., heavy origin + short query)
                    self.selected_name = None;
                }
                Task::none()
            }

            Msg::SearchChanged(s) => {
                self.search = s;
                let names = self.filtered_names();
                if let Some(first) = names.first() {
                    self.apply_selected_name(first);
                } else {
                    // no matches for current query; keep previous RGB or clear selection:
                    self.selected_name = None; // <- or comment this out if you want to keep old selection
                }
                Task::none()
            }

            Msg::QueryChanged(s) => {
                self.search = s;
                let names = self.filtered_names();
                if let Some(first) = names.first() {
                    self.apply_selected_name(first);
                } else {
                    self.selected_name = None;
                }
                Task::none()
            }

            Msg::PickedName(name) => {
                self.apply_selected_name(name);
                Task::none()
            }

            Msg::PickChanged(name) => {
                self.apply_selected_name(&name);
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
        // --- filtered names (FAST; uses indexed search via self.filtered_names) ---
        // NOTE: make sure your `filtered_names()` calls `search_in_origin(...)`
        // with TokenMode::Any/All as we discussed, not a per-item substring scan.
        // --- filtered names (FAST; zero alloc on clear) ---
        let filtered_names: Vec<&'static str> = self.filtered_names();

        // Keep selected if still present
        let selected_opt: Option<&'static str> = self.selected_name.as_deref().and_then(|cur| {
            filtered_names
                .iter()
                .copied()
                .find(|s| s.eq_ignore_ascii_case(cur))
        });

        // --- Origins dropdown stays the same ---
        let origins_list = {
            let mut v = vec![
                Origin::All,
                Origin::Css,
                Origin::XKCD,
                Origin::Pantone,
                Origin::Hindi,
                Origin::Persian,
                Origin::National,
            ];
            #[cfg(feature = "github-colors")]
            {
                v.push(Origin::GitHub);
            }
            v
        };

        let on_origin: fn(Origin) -> Msg = Msg::OriginPicked;

        let origin_dd: PickList<Origin, Vec<Origin>, Origin, Msg, Theme, Renderer> = pick_list(
            origins_list,               // Vec<Origin>
            Some(self.selected_origin), // Option<Origin>
            on_origin,                  // fn(Origin) -> Msg
        )
        .placeholder("Origin")
        .width(Length::Shrink);

        // --- Controls ---
        let search_box: iced::widget::TextInput<'_, Msg, Theme, Renderer> =
            text_input("Search color name…", &self.search)
                .on_input(Msg::SearchChanged)
                .padding(6)
                .size(16)
                .width(Length::Fill);

        // IMPORTANT: make pick_list generic over &str and message is Msg::PickedName(&'static str)
        // filtered_names: Vec<&'static str>
        let filtered_names: Vec<&'static str> = self.filtered_names();

        // selected_opt: Option<&'static str>
        let selected_opt: Option<&'static str> = self.selected_name.as_deref().and_then(|cur| {
            filtered_names
                .iter()
                .copied()
                .find(|s| s.eq_ignore_ascii_case(cur))
        });

        // Tell the compiler exactly what the on_select fn is:
        let on_select: fn(&'static str) -> Msg = Msg::PickedName;

        // --- filtered names (FAST; lazy for heavy origins) ---
        let filtered_names: Vec<&'static str> = self.filtered_names();

        // Keep selected if still present
        let selected_opt: Option<&'static str> = self.selected_name.as_deref().and_then(|cur| {
            filtered_names
                .iter()
                .copied()
                .find(|s| s.eq_ignore_ascii_case(cur))
        });

        // --- Origins dropdown unchanged ---
        let origins_list = {
            let mut v = vec![
                Origin::All,
                Origin::Css,
                Origin::XKCD,
                Origin::Pantone,
                Origin::Hindi,
                Origin::Persian,
                Origin::National,
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

        // --- Search box unchanged ---
        let search_box = text_input("Search color name…", &self.search)
            .on_input(Msg::SearchChanged)
            .padding(6)
            .size(16)
            .width(Length::Fill);

        // --- Name dropdown ---
        // Tell the compiler the callback type explicitly if needed:
        let on_select: fn(&'static str) -> Msg = Msg::PickedName;

        let name_dd = pick_list(
            filtered_names.clone(), // Vec<&'static str>
            selected_opt,           // Option<&'static str>
            on_select,              // fn(&'static str) -> Msg
        )
        .placeholder({
            // Nice hint for heavy sets
            use crate::colors_helper::{HEAVY_MIN_QUERY, is_heavy_origin};
            if is_heavy_origin(self.selected_origin) && self.search.trim().len() < HEAVY_MIN_QUERY {
                "Type at least 2 letters…"
            } else {
                "Select a color"
            }
        })
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
    fn filtered_names(&self) -> Vec<&'static str> {
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
}

impl App {
    /// Filter names by origin *and* search, then sort alphabetically

    /// Get HEX for a name, *restricted to the active origin*.
    fn hex_for_name_in_origin(&self, name: &str) -> Option<&'static str> {
        let set = colors_helper::colors_for(self.selected_origin);
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
        Origin::National => COLORS_NATIONAL.as_slice(),
    }
}

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use crate::colors::github_colors::COLORS_GITHUB;
use crate::colors::persian_colors::COLORS_PERSIAN;
use crate::colors::xkcd::COLORS_XKCD;

use crate::colors::national_colors::COLORS_NATIONAL;

use crate::colors::hindi_colors::COLORS_HINDI;
//{COLORS_GITHUB, COLORS_HINDI, COLORS_NATIONAL, COLORS_PANTONE, COLORS_PERSIAN, COMBINED_COLORS};

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
fn tokenize_lc(s: &str) -> impl Iterator<Item = String> + '_ {
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
