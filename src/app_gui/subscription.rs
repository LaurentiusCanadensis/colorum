use crate::app_gui::App;
use crate::messages::Msg;

impl App {
    // add near your other imports in app_gui.rs

    // then in Application::subscription():
    pub fn subscription(&self) -> iced::Subscription<Msg> {
        let keyboard = iced::keyboard::on_key_press(|key, _mods| match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                Some(Msg::KeyPressed(key))
            }
            _ => None,
        });

        if self.show_splash {
            let timer = iced::time::every(std::time::Duration::from_millis(100))
                .map(|_| Msg::Tick);
            iced::Subscription::batch([keyboard, timer])
        } else {
            keyboard
        }
    }
}
