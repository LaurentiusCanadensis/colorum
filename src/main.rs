mod app;
mod colors;
mod messages;
mod hex;
mod rgb;
mod widgets;

use app::App;
use iced::{Theme, application};
use crate::rgb::hex_to_rgb;

fn main() -> iced::Result {

    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}

