#![warn(dead_code, unused_imports, unused_mut, unused_variables)]
mod app_gui;
mod colors_helper;
mod hex;
pub mod messages;
mod rgb;
mod widgets;

pub mod colors;
pub mod color_types;
use crate::rgb::hex_to_rgb;
use app_gui::App;
use app_gui::view::view;
use iced::{Theme, application};

pub mod brand;




use colors::*;

fn main() -> iced::Result {
    rust_colors::init_profiling();

    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .subscription(App::subscription)
        .run()
}
