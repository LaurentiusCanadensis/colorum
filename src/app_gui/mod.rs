use std::collections::HashMap;


use std::sync::LazyLock;
// src/app_gui.rs
use crate::colors_helper::{self, COLORS_GITHUB, COLORS_HINDI, COLORS_NATIONAL, COLORS_PANTONE, COLORS_PERSIAN, COLORS_XKCD, COMBINED_COLORS, Origin};
use crate::hex::{combine_hex, sanitize_hex2};
use crate::messages::Msg;
use crate::rgb::hex_to_rgb;
use iced::widget::{PickList, button, column, container, pick_list, row, scrollable, text, text_input, Space, mouse_area};
use iced::{Alignment, Background, Color, Element, Length, Renderer, Task, Theme, clipboard, border};

pub mod app_helpers;
pub mod subscription;

pub mod update;
pub mod view;
pub struct App {
    pub rr: String,
    pub gg: String,
    pub bb: String,

    pub search: String,
    pub selected_name: Option<String>,

    pub selected_origin: Origin,
    pub status: String,

    query: String,

    dropdown_scroll_id: scrollable::Id,

    // Index-driven dropdown state
    base: Vec<(&'static str, &'static str)>,                  // current origin's slice (owned)
    base_index_by_name: std::collections::HashMap<&'static str, usize>, // name -> index in `base`
    results_idx: Vec<usize>,                                  // filtered result indices into `base`
    sel_pos: Option<usize>,                                   // cursor position in `results_idx`
    dropdown_open: bool
}


impl Default for App {
    fn default() -> Self {
        let selected_origin = Origin::All; // or your preferred default

        // materialize the current origin's list
        let base = crate::colors_helper::colors_for(selected_origin).to_vec();

        // build name -> index map
        let mut base_index_by_name = HashMap::with_capacity(base.len());
        for (i, &(_h, n)) in base.iter().enumerate() {
            base_index_by_name.insert(n, i);
        }

        Self {
            // initialize all your existing fields here
            // (examples — replace with your actual fields)
            rr: String::new(),
            gg: String::new(),
            bb: String::new(),
            search: String::new(),
            selected_name: None,
            selected_origin: selected_origin, // or your desired default
            status: "".to_string(),
            query: "".to_string(),

            base,
            base_index_by_name,
            results_idx: Vec::new(),
            sel_pos: None,

            dropdown_scroll_id: scrollable::Id::unique(),
            // ...any other fields you have...
            dropdown_open: false,
        }
    }
}