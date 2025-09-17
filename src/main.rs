mod app;
mod colors_helper;
mod hex;
mod messages;
mod rgb;
mod widgets;

mod github_colors;
mod hindi_colors;
mod national_colors;
mod pantone_colors;
mod persian_colors;

use crate::rgb::hex_to_rgb;
use app::App;
use iced::{Theme, application};

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}
