mod app;
mod colors;
mod messages;
mod util;
mod widgets;

use app::App;
use iced::{Theme, application};

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .theme(|_| Theme::Light)
        .run()
}
