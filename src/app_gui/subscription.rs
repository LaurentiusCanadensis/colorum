use crate::app_gui::App;
use crate::messages::Msg;

impl App {
    // add near your other imports in app_gui.rs

    // then in Application::subscription():
    pub fn subscription(&self) -> iced::Subscription<Msg> {
        iced::keyboard::on_key_press(|key, _mods| match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                Some(Msg::KeyPressed(key))
            }
            _ => None,
        })
    }
}
