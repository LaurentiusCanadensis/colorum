use iced::widget::scrollable;
use std::collections::HashMap;
use crate::core::color_types::{HexCode, ColorName};
use crate::core::rgb::CopyFormat;

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
    pub search_input_id: iced::widget::text_input::Id,

    // Index-driven dropdown state
    pub base: Vec<(HexCode, ColorName)>,
    pub base_index_by_name: HashMap<ColorName, usize>,
    pub results_idx: Vec<usize>,
    pub sel_pos: Option<usize>,
    pub dropdown_open: bool,
    last_query: String,
    last_results_idx: Vec<usize>,
    pub base_hex_nopound: Vec<&'static str>, // NEW: "E53B3B" for "#E53B3B"

    // Splash screen state
    pub show_splash: bool,
    pub splash_start_time: Option<std::time::Instant>,

    // Window size for responsive behavior
    pub window_width: f32,
    pub window_height: f32,

    // Copy format cycling for center click
    pub copy_format: CopyFormat,
    pub format_feedback: Option<(String, std::time::Instant)>, // (message, timestamp)

}

impl Default for App {
    fn default() -> Self {
        let selected_origin = crate::colors_helper::Origin::All;

        // materialize the current origin's list
        let base = crate::colors_helper::origin_slice(selected_origin).to_vec();

        // build name -> index map
        let mut base_index_by_name = HashMap::with_capacity(base.len());
        for (i, (_h, n)) in base.iter().enumerate() {
            base_index_by_name.insert(*n, i);
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
            search_input_id: iced::widget::text_input::Id::unique(),
            dropdown_open: false,
            base_names_lc: Vec::new(),
            // ...include any other fields you have here unchanged...
            base_hex_nopound: vec![],

            // Initialize splash screen
            show_splash: true,
            splash_start_time: Some(std::time::Instant::now()),

            // Initialize window size (will be updated by window events)
            window_width: 800.0,
            window_height: 600.0,

            // Initialize copy format
            copy_format: CopyFormat::default(),
            format_feedback: None,

        };
        // populate lowercase cache and hex without pound once at startup
        s.base_names_lc = s
            .base
            .iter()
            .map(|(_h, n)| n.as_str().to_ascii_lowercase())
            .collect();
        s.base_hex_nopound = s
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
        s.repopulate_full_results();
        s
    }
}
