use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::{clipboard, Event, Task};
use crate::app_gui::App;
use crate::colors_helper::{Origin, sanitize_hex2};
use crate::hex::combine_hex;
use crate::messages::Msg;

impl App {
    pub fn title(&self) -> String {
        "rondel".into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::RawEvent(Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                match key.as_ref() {
                    Key::Named(Named::ArrowUp)
                    | Key::Named(Named::ArrowDown)
                    | Key::Named(Named::ArrowLeft)
                    | Key::Named(Named::ArrowRight)
                    | Key::Named(Named::Enter) => {
                        // Re-use your existing key handler:
                        return self.update(Msg::KeyPressed(key));
                    }
                    _ => {}
                }
                Task::none()
            }

            Msg::MoveSelection(d) => {
                self.move_selection(d);
                Task::none()
            }
            Msg::ActivateSelection => {
                self.activate_selected();
                Task::none()
            }

            Msg::OpenDropdown => {
                self.dropdown_open = true;
                Task::none()
            }
            Msg::CloseDropdown => {
                self.dropdown_open = false;
                Task::none()
            }
            Msg::RChanged(s) => {
                self.rr = sanitize_hex2(&s);
                Task::none()
            }
            Msg::GChanged(s) => {
                self.gg = sanitize_hex2(&s);
                Task::none()
            }
            Msg::BChanged(s) => {
                self.bb = sanitize_hex2(&s);
                Task::none()
            }

            Msg::WheelChanged(ch, v) => {
                let hh = format!("{v:02X}");
                match ch {
                    crate::messages::Channel::R => self.rr = hh,
                    crate::messages::Channel::G => self.gg = hh,
                    crate::messages::Channel::B => self.bb = hh,
                }
                Task::none()
            }

            Msg::SearchChanged(s) => {
                self.search = s;
                let names = self.filtered_names();
                if let Some(first) = names.first() {
                    self.apply_selected_name(first);
                } else {
                    // no matches for current query; keep previous RGB or clear selection:
                    self.selected_name = None; // <- or comment this out if you want to keep old selection
                }
                Task::none()
            }

            Msg::PickedName(name) => {
                self.apply_selected_name(name);
                Task::none()
            }

            Msg::PickChanged(name) => {
                self.apply_selected_name(&name);
                Task::none()
            }

            Msg::CenterClicked => {
                let text = combine_hex(&self.rr, &self.gg, &self.bb);
                clipboard::write(text)
            }

            Msg::CopyHex(s) => clipboard::write(s),

            Msg::Clear => {
                self.rr.clear();
                self.gg.clear();
                self.bb.clear();
                self.search.clear();
                self.selected_name = None;
                self.status.clear();
                self.selected_origin = Origin::All;
                Task::none()
            }

            Msg::QueryChanged(q) => {
                self.query = q;
                self.rebuild_results();                 // sets sel_pos = Some(0) when non-empty
                return self.scroll_to_selected();
            }

            Msg::OriginPicked(o) => {
                self.selected_origin = o;
                self.reindex_origin();
                self.rebuild_results();
                return self.scroll_to_selected();
            }
            Msg::KeyPressed(key) => {
                use iced::keyboard::{Key, key::Named};
                let mut moved = false;
                match key {
                    Key::Named(Named::ArrowDown) => {
                        self.move_selection(1);
                        moved = true;
                    }
                    Key::Named(Named::ArrowUp) => {
                        self.move_selection(-1);
                        moved = true;
                    }
                    Key::Named(Named::ArrowRight) => {
                        self.move_selection(10);
                        moved = true;
                    }
                    Key::Named(Named::ArrowLeft) => {
                        self.move_selection(-10);
                        moved = true;
                    }
                    Key::Named(Named::Enter) => {
                        self.activate_selected();
                        return Task::none();
                    }
                    _ => {}
                }
                if moved {
                    self.activate_selected();          // wheel updates immediately
                    return self.scroll_to_selected();  // pin in view
                }
                Task::none()
            }
            Msg::PressedEnter => {
                self.activate_selected();
                iced::Task::none()
            }
            Msg::DropdownClicked(row) => {
                if row < self.results_idx.len() {
                    self.sel_pos = Some(row);
                    self.activate_selected();
                    return self.scroll_to_selected();  // keep clicked item at top
                }
                Task::none()
            }

            _ => { Task::none() }
        }
    }
}