use crate::ui::app_gui::App;
use crate::ui::messages::Msg;
use iced::widget::{container, row, column, image};
use iced::{Alignment, Element, Length};

impl App {


    pub fn view(&self) -> Element<'_, Msg> {
        // Show splash screen for 2 seconds
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

        // Make wheel size responsive to window size - increased by ~45px due to removing text below wheel
        let wheel_size = if is_very_small_window {
            (self.window_width.min(self.window_height) * 0.4).min(245.0).max(195.0)
        } else if is_small_window {
            (self.window_width.min(self.window_height) * 0.5).min(325.0).max(245.0)
        } else {
            (self.window_width.min(self.window_height) * 0.6).min(445.0).max(325.0)
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

        // Calculate panel width - larger at startup, scales with wheel
        let panel_width = (wheel_size * 0.4).max(200.0).min(300.0);

        // Create the new analytics and search panels with proportional sizing
        let color_analytics = self.view_color_analytics_with_width(panel_width);
        let search_block = self.view_search_block_with_width(panel_width);

        // Hide panels for very small windows - show only the wheel
        // For wide windows, be very lenient with height requirement
        let show_panels = if self.window_width > 800.0 {
            // Very wide windows: almost always show panels
            self.window_height > 200.0
        } else {
            // Normal windows: standard requirements
            self.window_width > 400.0 && self.window_height > 250.0
        };

        // Always stack panels vertically - analytics on top, search below
        let side_panels: Element<'_, Msg> = if show_panels {
            column![]
                .push(color_analytics)
                .push(search_block)
                .spacing(6)
                .align_x(Alignment::Start)
                .into()
        } else {
            // Empty space when panels are hidden
            iced::widget::Space::with_height(Length::Fixed(0.0)).into()
        };

        // Create main content with wheel and panels
        let wheel_container = container(wheel_only)
            .width(Length::Shrink)
            .align_x(Alignment::Center);

        let main_content = if !show_panels {
            // Very small window: show only the wheel, centered
            container(wheel_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
        } else if self.window_width > 450.0 {
            // Wide enough for side-by-side layout: wheel left, panels right
            let horizontal_layout = row![]
                .push(wheel_container)
                .push(side_panels)
                .spacing(20)
                .align_y(Alignment::Start);

            container(horizontal_layout)
                .width(Length::Fill)
                .align_x(Alignment::Center)
        } else {
            // Medium window: wheel on top, panels below
            let vertical_layout = column![]
                .push(wheel_container)
                .push(side_panels)
                .spacing(10)
                .align_x(Alignment::Center);

            container(vertical_layout)
                .width(Length::Fill)
                .align_x(Alignment::Center)
        };

        let mut content = column![]
            .push(main_content)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        // Add format feedback if present
        if let Some((feedback_msg, _)) = &self.format_feedback {
            content = content.push(
                container(
                    iced::widget::text(feedback_msg)
                        .size(14)
                        .color(iced::Color::from_rgb(0.3, 0.8, 0.3)), // Green color
                )
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding([1, 0]),
            );
        }


        let final_content = content;

        // Ultra compact spacing and padding for all window sizes
        let spacing = if is_very_small_window {
            0  // No spacing
        } else if is_small_window {
            0  // No spacing
        } else {
            1  // Minimal spacing
        };

        let padding = if is_very_small_window {
            [0, 0]  // No padding
        } else if is_small_window {
            [0, 0]  // No padding
        } else {
            [1, 1]  // Minimal padding
        };

        let final_content = final_content
            .align_x(Alignment::Center)
            .spacing(spacing)
            .padding(padding);

        container(final_content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn splash_view(&self) -> Element<'_, Msg> {
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