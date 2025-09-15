mod app;
mod messages;
mod colors;
mod util;
mod widgets;

use app::App;
use iced::{application, Theme};

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}