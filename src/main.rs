mod app;
mod colors_helper;
mod hex;
mod messages;
mod rgb;
mod widgets;

pub mod colors;

use crate::rgb::hex_to_rgb;
use app::App;
use iced::{Theme, application};

use colors::*;

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}
