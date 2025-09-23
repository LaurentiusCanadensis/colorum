#![warn(dead_code, unused_imports, unused_mut, unused_variables)]
mod colors_helper;
mod core;
mod ui;

pub mod colors;
pub mod brand;

use crate::core::rgb::hex_to_rgb;
use ui::app_gui::App;
// Unused: use app_gui::view::view;
use iced::{Theme, application};




// Unused: use colors::*;

fn main() -> iced::Result {
    colorum::init_profiling();

    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .subscription(App::subscription)
        .run()
}
