use crate::ui::app_gui::App;
use crate::colors_helper::{Origin, REGISTRY};
use crate::ui::messages::Msg;
use crate::ui::widgets::color_wheel::WheelSearchProps;
use iced::widget::{container, image, pick_list, scrollable, svg, text_input};
use iced::{Alignment, border, Element, Length};
use crate::ui::app_gui::app_helpers::origins_vec;
use crate::brand;

impl App {


    pub fn view(&self) -> Element<Msg> {
        // Show splash screen for 5 seconds
        if self.show_splash {
            return self.splash_view();
        }

        // --- helpers ---
        fn u8_from_hex2(s: &str) -> u8 {
            if s.len() == 2 {
                u8::from_str_radix(s, 16).unwrap_or(0)
            } else {
                0
            }
        }

        // --- wheel state ---
        let r = u8_from_hex2(&self.rr);
        let g = u8_from_hex2(&self.gg);
        let b = u8_from_hex2(&self.bb);

        let wheel = crate::ui::widgets::color_wheel::ColorWheel::new(r, g, b, Msg::WheelChanged);

        // Determine responsive sizing based on window dimensions
        let is_small_window = self.window_width < 500.0 || self.window_height < 450.0;
        let is_very_small_window = self.window_width < 350.0 || self.window_height < 350.0;

        // Better proportional sizing - reduce by about 1/3 each step
        let wheel_size = if is_very_small_window {
            180.0  // Very small screens - still usable
        } else if is_small_window {
            240.0  // Small screens - moderate reduction
        } else {
            300.0  // Normal screens - back to original
        };

        // Only hide inputs on very small screens
        let hide_inputs = is_very_small_window;

        // Create a simple wheel without search for better layout control
        let wheel_only = wheel.view_with_size(
            "Colorum Wheel",
            &self.rr,
            &self.gg,
            &self.bb,
            wheel_size,
            hide_inputs,
        );

        // --- filtered names (FAST; uses your cached/indexed search) ---
        let filtered_names: Vec<&'static str> = self.filtered_names();

        // Keep selected if still present
        let _selected_opt: Option<&'static str> = self.selected_name.as_deref().and_then(|cur| {
            filtered_names
                .iter()
                .copied()
                .find(|s| s.eq_ignore_ascii_case(cur))
        });

        // Old:
        // let origins_list = vec![Origin::All, Origin::XKCD, ...];

        // New: derive from REGISTRY so it auto-includes new palettes
        let origins_list = origins_vec();

        let origin_dd = iced::widget::pick_list(
            origins_list,
            Some(self.selected_origin), // <- must be Some(current)
            Msg::OriginPicked,          // <- on_select
        )
        .placeholder("Origin")
        .width(iced::Length::Shrink);

        // Search box (uses your existing query + messages)

        // Keep the rest of your layout as you like:
        let clear_btn = iced::widget::button("Clear")
            .on_press(Msg::Clear)
            .padding([6, 10]);




        // Search box and dropdown
        let search_box = iced::widget::text_input("Search color name…", &self.query)
            .on_input(Msg::QueryChanged)
            .on_submit(Msg::PressedEnter)
            .padding(8)
            .width(Length::Fill);

        // Create dropdown if we have results
        let dropdown: Option<iced::Element<'_, Msg>> = if self.dropdown_open && !self.results_idx.is_empty() {
            // Limit items based on window size: 3 for very small, 5 for small, more for larger
            let max_items = if is_very_small_window {
                3  // Very constrained
            } else if is_small_window {
                5  // Moderately constrained
            } else {
                if self.results_idx.len() > 10 { 8 } else { self.results_idx.len() }
            };

            let dropdown_height = if is_very_small_window {
                60.0  // Very compact
            } else if is_small_window {
                100.0  // Moderately compact
            } else {
                180.0  // Full size
            };

            let mut col: iced::widget::Column<'_, crate::ui::messages::Msg, iced::Theme, iced::Renderer> = iced::widget::Column::new()
                .spacing(1)
                .padding(4)
                .align_x(Alignment::Start)
                .width(Length::Fill);

            for (row, &idx) in self.results_idx.iter().take(max_items).enumerate() {
                let (hex, name) = self.base[idx];
                let is_sel = self.sel_pos == Some(row);
                let label = if is_sel {
                    format!("▶ {}  {}", name.as_str(), hex.as_str())
                } else {
                    format!("{}  {}", name.as_str(), hex.as_str())
                };

                let row_body = container(iced::widget::text(label))
                    .padding([4, 6])
                    .width(Length::Fill)
                    .style(move |_theme: &iced::Theme| {
                        if is_sel {
                            iced::widget::container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.20,
                                    g: 0.40,
                                    b: 0.80,
                                    a: 0.20,
                                })),
                                border: border::Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        } else {
                            iced::widget::container::Style::default()
                        }
                    });

                let click = iced::widget::mouse_area(row_body).on_press(Msg::DropdownClicked(row));
                col = col.push(click);
            }

            Some(
                iced::widget::scrollable(col)
                    .id(self.dropdown_scroll_id.clone())
                    .height(Length::Fixed(dropdown_height))
                    .width(Length::Fill)
                    .into()
            )
        } else {
            None
        };

        let content = iced::widget::Column::new()
            // Center the wheel
            .push(
                container(wheel_only)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([4, 0]),
            )
            // Origin selector below wheel
            .push(
                container(origin_dd)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([4, 0]),
            )
            // Search box
            .push(
                container(search_box)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([0, 8]),
            );

        // Add dropdown if present
        let mut final_content = content;
        if let Some(dd) = dropdown {
            final_content = final_content.push(
                container(dd)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([0, 8]),
            );
        }

        // Bottom row with clear button
        final_content = final_content.push(
            container(
                iced::widget::Row::new()
                    .push(clear_btn)
                    .spacing(10)
                    .align_y(iced::Alignment::Center)
                    .width(Length::Shrink),
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([4, 0]),
        );

        // Adjust spacing and padding based on window size
        let spacing = if is_very_small_window {
            2  // Very tight spacing
        } else if is_small_window {
            4  // Moderate spacing
        } else {
            6  // Normal spacing
        };

        let padding = if is_very_small_window {
            [2, 2]  // Very tight padding
        } else if is_small_window {
            [4, 4]  // Moderate padding
        } else {
            [8, 8]  // Normal padding
        };

        final_content = final_content
            .align_x(Alignment::Center)
            .spacing(spacing)
            .padding(padding);

        scrollable(
            container(final_content)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .into()
    }

    fn splash_view(&self) -> Element<Msg> {
        let logo = image("src/assets/logo.png")
            .width(Length::Fixed(600.0))
            .height(Length::Fixed(279.0));

        let content = container(logo)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
