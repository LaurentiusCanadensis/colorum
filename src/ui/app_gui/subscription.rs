use crate::ui::app_gui::App;
use crate::ui::messages::Msg;

impl App {
    // add near your other imports in app_gui.rs

    // then in Application::subscription():
    pub fn subscription(&self) -> iced::Subscription<Msg> {
        let keyboard = iced::keyboard::on_key_press(|key, mods| match key {
            // Handle arrow keys with/without shift for blue adjustments
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                // Pass modifiers with key message for shift handling
                Some(Msg::KeyPressedWithMods(key, mods))
            }
            // Ctrl+F for search focus
            iced::keyboard::Key::Character(ref c) if c == "f" && mods.control() => {
                Some(Msg::FocusSearch)
            }
            // Ctrl+C for copying current color
            iced::keyboard::Key::Character(ref c) if c == "c" && mods.control() => {
                Some(Msg::CopyCurrentColor)
            }
            _ => None,
        });

        // Window resize events
        let window_events = iced::window::resize_events().map(|(_, size)| {
            Msg::WindowResized(size.width, size.height)
        });

        if self.show_splash {
            let timer = iced::time::every(std::time::Duration::from_millis(100))
                .map(|_| Msg::Tick);
            iced::Subscription::batch([keyboard, timer, window_events])
        } else {
            iced::Subscription::batch([keyboard, window_events])
        }
    }
}
