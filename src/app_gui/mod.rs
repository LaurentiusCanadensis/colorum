use iced::widget::{Space, column, container, mouse_area, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Renderer, Theme, border};
use std::collections::HashMap;

pub mod app_helpers;
pub mod subscription;
pub mod update;
pub mod view;

pub struct App {
    // lowercase cache for fast substring search (parallel to `base`)
    base_names_lc: Vec<String>,

    pub rr: String,
    pub gg: String,
    pub bb: String,

    pub search: String,
    pub selected_name: Option<String>,

    pub selected_origin: crate::colors_helper::Origin,
    pub status: String,

    pub query: String,

    pub dropdown_scroll_id: scrollable::Id,

    // Index-driven dropdown state
    pub base: Vec<(&'static str, &'static str)>,
    pub base_index_by_name: HashMap<&'static str, usize>,
    pub results_idx: Vec<usize>,
    pub sel_pos: Option<usize>,
    pub dropdown_open: bool,
    last_query: String,
    last_results_idx: Vec<usize>,
    pub base_hex_nopound: Vec<&'static str>, // NEW: "E53B3B" for "#E53B3B"
}

impl Default for App {
    fn default() -> Self {
        let selected_origin = crate::colors_helper::Origin::All;

        // materialize the current origin's list
        let base = crate::colors_helper::colors_for(selected_origin).to_vec();

        // build name -> index map
        let mut base_index_by_name = HashMap::with_capacity(base.len());
        for (i, &(_h, n)) in base.iter().enumerate() {
            base_index_by_name.insert(n, i);
        }

        let mut s = Self {
            rr: String::new(),
            gg: String::new(),
            bb: String::new(),
            search: String::new(),
            selected_name: None,
            selected_origin, // keep your existing value
            status: String::new(),
            query: String::new(),

            // these three are important:
            base,               // already built above from the default origin
            base_index_by_name, // already built above
            results_idx: Vec::new(),
            last_query: String::new(),
            last_results_idx: Vec::new(),

            sel_pos: None,
            dropdown_scroll_id: scrollable::Id::unique(),
            dropdown_open: false,
            base_names_lc: Vec::new(),
            // ...include any other fields you have here unchanged...
            base_hex_nopound: vec![],
        };
        s.base_names_lc = s
            .base
            .iter()
            .map(|&(_h, n)| n.to_ascii_lowercase())
            .collect();
        s.base_hex_nopound = s
            .base
            .iter()
            .map(|&(h, _)| {
                if let Some(stripped) = h.strip_prefix('#') {
                    stripped
                } else {
                    h
                }
            })
            .collect();
        // populate lowercase cache once at startup
        s.base_names_lc = s
            .base
            .iter()
            .map(|&(_h, n)| n.to_ascii_lowercase())
            .collect();
        s.repopulate_full_results();
        s
    }
}
