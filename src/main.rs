#![warn(dead_code, unused_imports, unused_mut, unused_variables)]
mod colors_helper;
mod core;
mod ui;

pub mod colors;
pub mod brand;

use ui::app_gui::App;
use iced::{Theme, application};

fn main() -> iced::Result {
    colorum::init_profiling();

    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .subscription(App::subscription)
        .run()
}
