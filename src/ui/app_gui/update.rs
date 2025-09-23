use crate::ui::app_gui::App;
use crate::colors_helper::{
    HEAVY_MIN_QUERY, MAX_RESULTS, Origin, TokenMode, is_heavy_origin, origin_rank, sanitize_hex2,
    search_in_origin,
};
use crate::core::hex::combine_hex;
use crate::ui::messages::Msg;
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::{Event, Task, clipboard};

impl App {
    pub fn title(&self) -> String {
        "Colorum".to_string()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        // Check splash timer on every update
        if self.show_splash {
            if let Some(start_time) = self.splash_start_time {
                if start_time.elapsed() >= std::time::Duration::from_secs(5) {
                    self.show_splash = false;
                    self.splash_start_time = None;
                }
            }
        }

        match msg {
            Msg::RawEvent(Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                match key.as_ref() {
                    Key::Named(Named::ArrowUp)
                    | Key::Named(Named::ArrowDown)
                    | Key::Named(Named::ArrowLeft)
                    | Key::Named(Named::ArrowRight)
                    | Key::Named(Named::Enter) => {
                        // Re-use your existing key handler:
                        return self.update(Msg::KeyPressed(key));
                    }
                    _ => {}
                }
                Task::none()
            }

            Msg::MoveSelection(d) => {
                self.move_selection(d);
                Task::none()
            }
            Msg::ActivateSelection => {
                self.activate_selected();
                Task::none()
            }

            Msg::OpenDropdown => {
                self.dropdown_open = true;
                Task::none()
            }
            Msg::CloseDropdown => {
                self.dropdown_open = false;
                Task::none()
            }
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
                    crate::ui::messages::Channel::R => self.rr = hh,
                    crate::ui::messages::Channel::G => self.gg = hh,
                    crate::ui::messages::Channel::B => self.bb = hh,
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

            Msg::PickedName(name) => {
                self.apply_selected_name(name);
                if let Some(hex) = self.hex_for_name_in_origin(&name) {
                    self.set_from_hex(hex);
                }
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

                self.query.clear();
                self.results_idx.clear();
                self.last_query.clear();
                self.last_results_idx.clear();

                self.sel_pos = None;
                self.dropdown_open = false;

                self.selected_name = None;
                self.status.clear();

                // Keep the current origin, or reset if you prefer:
                // self.selected_origin = Origin::All;

                Task::none()
            }

            Msg::QueryChanged(s) => {
                self.query = s;
                let q = self.query.trim();
                #[cfg(feature = "github-colors")]
                // Treat these origins as "heavy"
                let is_heavy = matches!(self.selected_origin, Origin::All | Origin::GitHub);

                let is_heavy = matches!(self.selected_origin, Origin::All);

                const HEAVY_MIN: usize = 2;
                const MAX_RESULTS: usize = 200;

                // If it *looks* like a hex query, run HEX reverse search and bypass gating
                if let Some(hexq) = normalize_hex_query(q) {
                    self.results_idx.clear();

                    // prefix match (fast) over cached hex without '#'
                    for (i, &hex_no_pound) in self.base_hex_nopound.iter().enumerate() {
                        if hex_no_pound.starts_with(&hexq) {
                            self.results_idx.push(i);
                            if self.results_idx.len() >= MAX_RESULTS {
                                break;
                            }
                        }
                    }

                    self.sel_pos = if self.results_idx.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.dropdown_open = !self.results_idx.is_empty();

                    // Auto-select first hit to update center color immediately
                    if let Some(&i0) = self.results_idx.first() {
                        let (hex, name) = self.base[i0];
                        self.selected_name = Some(name.as_str().to_string());
                        self.set_from_hex(hex.as_str());
                    }

                    // record cache keys for your “seed” optimization path
                    self.last_query = hexq;
                    self.last_results_idx = self.results_idx.clone();

                    return Task::none();
                }

                // Non-hex query path (name search)
                if q.is_empty() || (is_heavy && q.len() < HEAVY_MIN) {
                    // Heavy origin + too-short query → keep list closed (or show top-N if you prefer)
                    self.results_idx.clear();
                    self.sel_pos = None;
                    self.dropdown_open = false;
                    self.last_query.clear();
                    self.last_results_idx.clear();

                    // When query is empty, show all colors for the current origin
                    if q.is_empty() {
                        self.selected_name = None;
                        self.repopulate_full_results_capped();
                        self.dropdown_open = true;
                        // Optional: clear the color or keep the last selected color
                        // self.rr.clear();
                        // self.gg.clear();
                        // self.bb.clear();
                    }

                    return Task::none();
                }

                // Use Entity-aware search from search_in_origin
                let mode = if q.contains(' ') {
                    crate::colors_helper::TokenMode::All
                } else {
                    crate::colors_helper::TokenMode::Any
                };

                let hits = crate::colors_helper::search_in_origin(self.selected_origin, q, mode.clone());

                self.results_idx.clear();
                self.results_idx.reserve(hits.len().min(MAX_RESULTS));

                // Map search results back to indices in self.base
                for (_hex, name) in hits.iter().take(MAX_RESULTS) {
                    if let Some(&i) = self.base_index_by_name.get(name) {
                        self.results_idx.push(i);
                    }
                }

                self.last_query = q.to_string();
                self.last_results_idx = self.results_idx.clone();

                self.sel_pos = if self.results_idx.is_empty() {
                    None
                } else {
                    Some(0)
                };
                self.dropdown_open = !self.results_idx.is_empty();

                // Auto-select first hit to update wheel
                if let Some(&i0) = self.results_idx.first() {
                    let (hex, name) = self.base[i0];
                    self.selected_name = Some(name.as_str().to_string());
                    self.set_from_hex(hex.as_str());
                }

                Task::none()
            }

            Msg::OriginPicked(o) => {
                #[cfg(feature = "profile")]
                let __t0 = std::time::Instant::now();

                self.selected_origin = o;

                let slice = crate::colors_helper::origin_slice(o);
                self.base = slice.to_vec();
                // (re)build caches
                self.base_index_by_name.clear();
                for (i, (_h, n)) in self.base.iter().enumerate() {
                    self.base_index_by_name.insert(*n, i);
                }
                self.base_names_lc = self
                    .base
                    .iter()
                    .map(|(_, n)| n.as_str().to_ascii_lowercase())
                    .collect();


                // rebuild lowercase cache for the new base
                self.base_names_lc.clear();
                self.base_names_lc.reserve(self.base.len());
                for (_h, n) in &self.base {
                    self.base_names_lc.push(n.as_str().to_ascii_lowercase());
                }

                // reset incremental caches
                self.last_query.clear();
                self.last_results_idx.clear();

                self.base_index_by_name.clear();
                for (i, (_h, n)) in self.base.iter().enumerate() {
                    self.base_index_by_name.insert(*n, i);
                }

                self.base_names_lc = self
                    .base
                    .iter()
                    .map(|(_h, n)| n.as_str().to_ascii_lowercase())
                    .collect();
                self.base_hex_nopound = self
                    .base
                    .iter()
                    .map(|(h, _)| {
                        if let Some(stripped) = h.as_str().strip_prefix('#') {
                            stripped
                        } else {
                            h.as_str()
                        }
                    })
                    .collect();

                // OWN the trimmed query (avoid borrowing self)
                let q: String = self.query.trim().to_owned();

                if q.is_empty() {
                    self.repopulate_full_results_capped();
                } else {
                    // Use Entity-aware search for origin changes too
                    let mode = if q.contains(' ') {
                        crate::colors_helper::TokenMode::All
                    } else {
                        crate::colors_helper::TokenMode::Any
                    };

                    let hits = crate::colors_helper::search_in_origin(self.selected_origin, &q, mode);

                    self.results_idx.clear();
                    self.results_idx.reserve(hits.len().min(MAX_RESULTS));

                    // Map search results back to indices in self.base
                    for (_hex, name) in hits.iter().take(MAX_RESULTS) {
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

                if let Some(&i0) = self.results_idx.first() {
                    let (hex, name) = self.base[i0];
                    self.selected_name = Some(name.as_str().to_string());
                    self.set_from_hex(hex.as_str());
                } else {
                    self.selected_name = None;
                }
                self.base_names_lc = self
                    .base
                    .iter()
                    .map(|(_h, n)| n.as_str().to_ascii_lowercase())
                    .collect();
                self.base_hex_nopound = self
                    .base
                    .iter()
                    .map(|(h, _)| {
                        if let Some(stripped) = h.as_str().strip_prefix('#') {
                            stripped
                        } else {
                            h.as_str()
                        }
                    })
                    .collect();

                #[cfg(feature = "profile")]
                eprintln!(
                    "[profile] OriginPicked({:?}): base_len={} hits={} took={:?}",
                    self.selected_origin,
                    self.base.len(),
                    self.results_idx.len(),
                    __t0.elapsed()
                );

                Task::none()
            }

            Msg::KeyPressed(key) => {
                use iced::keyboard::{Key, key::Named};
                let mut moved = false;
                match key {
                    Key::Named(Named::ArrowDown) => {
                        self.move_selection(1);
                        moved = true;
                    }
                    Key::Named(Named::ArrowUp) => {
                        self.move_selection(-1);
                        moved = true;
                    }
                    Key::Named(Named::ArrowRight) => {
                        self.move_selection(10);
                        moved = true;
                    }
                    Key::Named(Named::ArrowLeft) => {
                        self.move_selection(-10);
                        moved = true;
                    }
                    Key::Named(Named::Enter) => {
                        self.activate_selected();
                        return Task::none();
                    }
                    _ => {}
                }
                if moved {
                    self.activate_selected(); // wheel updates immediately
                    return self.scroll_to_selected(); // pin in view
                }
                Task::none()
            }
            Msg::PressedEnter => {
                self.activate_selected();
                iced::Task::none()
            }
            Msg::DropdownClicked(row) => {
                self.select_row(row);
                Task::none()
            }

            Msg::Tick => {
                if self.show_splash {
                    if let Some(start_time) = self.splash_start_time {
                        if start_time.elapsed() >= std::time::Duration::from_millis(2000) {
                            self.show_splash = false;
                            self.splash_start_time = None;
                        }
                    }
                }
                Task::none()
            }

            _ => Task::none(),
        }
    }
}

#[inline]
fn normalize_hex_query(q: &str) -> Option<String> {
    // Only treat as hex if it starts with '#' or has at least 2 hex digits
    // This prevents single letters like "a" from being treated as hex queries
    let has_hash = q.trim().starts_with('#');

    // Trim, strip leading '#', remove spaces, upper-case
    let mut s = q
        .trim()
        .trim_start_matches('#')
        .replace(char::is_whitespace, "");
    if s.is_empty() {
        return None;
    }

    // HEX only, max 6 chars
    if s.len() > 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    // Require either '#' prefix or at least 2 characters to avoid false positives
    if !has_hash && s.len() < 2 {
        return None;
    }

    s.make_ascii_uppercase();
    Some(s)
}
