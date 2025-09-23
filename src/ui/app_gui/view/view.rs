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
        // Show splash screen for 1 second
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

        // Create a simple wheel with color info display
        let wheel_only = wheel.view_with_color_info(
            "Colorum Wheel",
            &self.rr,
            &self.gg,
            &self.bb,
            wheel_size,
            hide_inputs,
            self.selected_name.as_deref(),
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
            Some(self.view_dropdown())
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
            .width(Length::Fixed(1080.0))  // 600.0 * 1.8 = 80% bigger
            .height(Length::Fixed(502.2));  // 279.0 * 1.8 = 80% bigger

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