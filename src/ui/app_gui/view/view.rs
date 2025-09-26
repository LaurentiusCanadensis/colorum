use crate::ui::app_gui::App;
use crate::ui::messages::Msg;
use iced::widget::{container, image, column, row, text_input, pick_list, button, scrollable};
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

        // Only show wheel alone for very small windows
        let show_only_wheel = self.window_width < 300.0 || self.window_height < 300.0;

        // Make wheel bigger relative to container size
        let wheel_size = if show_only_wheel {
            // Very small window: adjust wheel size to fit
            (self.window_width.min(self.window_height) * 0.8).min(350.0).max(200.0)
        } else {
            // Normal and larger windows: bigger relative to container
            (self.window_width * 0.55).min(600.0).max(350.0)
        };

        // Hide RGB inputs when window is under 600px in either dimension
        let hide_inputs = self.window_width < 600.0 || self.window_height < 600.0;

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
        let color_analytics = self.view_color_analytics_with_width(Length::Fixed(panel_width));

        // Create search and dropdown interface
        let search_box = iced::widget::text_input("Search color name…", &self.query)
            .on_input(Msg::QueryChanged)
            .on_submit(Msg::PressedEnter)
            .padding(8)
            .width(Length::Fixed(panel_width));

        // Origin dropdown with clear button
        let origins_list = crate::ui::app_gui::app_helpers::origins_vec();
        let origin_dd = iced::widget::pick_list(
            origins_list,
            Some(self.selected_origin),
            Msg::OriginPicked,
        )
        .placeholder("Origin")
        .width(Length::Shrink);

        let mut origin_row = row![]
            .push(origin_dd)
            .spacing(8)
            .align_y(Alignment::Center);

        // Only show clear button if window is wide enough (800px or more)
        if self.window_width >= 800.0 {
            let clear_btn = iced::widget::button("Clear")
                .on_press(Msg::Clear)
                .padding([4, 8]);
            origin_row = origin_row.push(clear_btn);
        }

        // Create dropdown if we have results
        let mut search_column = column![]
            .push(origin_row)
            .push(search_box)
            .spacing(8);

        if self.dropdown_open && !self.results_idx.is_empty() {
            search_column = search_column.push(self.view_dropdown());
        }

        let search_block = search_column;

        let content = if show_only_wheel {
            // Very small window: show only the wheel, centered
            container(wheel_only)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding([8, 8])
        } else {
            // Normal size and above: maintain consistent 60x60 grid layout
            // - Left: 40x60 wheel (full height)
            // - Top right: 20x30 analytics
            // - Bottom right: 20x30 search

            // Create wheel container for left side (40x60 area)
            let wheel_container = container(wheel_only)
                .width(Length::FillPortion(40))
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding([12, 12])
                .style(|_theme| iced::widget::container::Style {
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                });

            // Create analytics container for top right (20x30 area) with scrollable content
            let scrollable_analytics = scrollable(color_analytics)
                .width(Length::Fill)
                .height(Length::Fill);

            let analytics_container = container(scrollable_analytics)
                .width(Length::FillPortion(20))
                .height(Length::FillPortion(30))
                .padding([8, 8])
                .align_x(Alignment::Start)
                .align_y(Alignment::Start)
                .style(|_theme| iced::widget::container::Style {
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                });

            // Create search container for bottom right (20x30 area)
            let search_container = container(search_block)
                .width(Length::FillPortion(20))
                .height(Length::FillPortion(30))
                .padding([12, 12])
                .align_x(Alignment::Start)
                .align_y(Alignment::Start)
                .style(|_theme| iced::widget::container::Style {
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                });

            // Stack the right side containers vertically
            let right_side = column![]
                .push(analytics_container)
                .push(search_container)
                .spacing(8)
                .width(Length::FillPortion(20))
                .height(Length::Fill);

            // Create the main horizontal layout with consistent 60x60 grid ratios
            container(row![]
                .push(wheel_container)
                .push(right_side)
                .spacing(8)
                .width(Length::Fill)
                .height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
        };

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
            [4, 4]  // Small padding
        } else if is_small_window {
            [6, 6]  // Moderate padding
        } else {
            [8, 8]  // Better padding
        };

        let final_content = final_content
            .padding(padding);

        final_content
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