use crate::app_gui::App;
use crate::colors_helper::{Origin, REGISTRY};
use crate::messages::Msg;
use crate::widgets::color_wheel::WheelSearchProps;
use iced::widget::{container, image, pick_list, scrollable, svg, text_input};
use iced::{Alignment, border, Element, Length};
use crate::app_gui::app_helpers::origins_vec;
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

        let wheel = crate::widgets::color_wheel::ColorWheel::new(r, g, b, Msg::WheelChanged);

        let wheel_view = wheel.view_with_search_props(
            "Colorum Wheel",
            &self.rr,
            &self.gg,
            &self.bb,
            WheelSearchProps {
                query: &self.query,
                results_idx: &self.results_idx,
                sel_pos: self.sel_pos,
                base: &self.base,
                scroll_id: self.dropdown_scroll_id.clone(),
                on_query: Msg::QueryChanged,
                on_enter: || Msg::PressedEnter,
                on_click_row: Msg::DropdownClicked,
            },
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




        let content = iced::widget::Column::new()
            // Center the wheel without forcing width; it will scale to the window.
            .push(
                container(wheel_view)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([4, 0]),
            )
            // Input above dropdown, centered, shrink-friendly (inline instead of `stacked_controls`)
            // Bottom row with origin selector + clear button
            .push(
                iced::widget::Row::new()
                    .push(origin_dd)
                    .push(clear_btn)
                    .spacing(10)
                    .align_y(iced::Alignment::Center)
                    .width(Length::Shrink),
            )
            .align_x(Alignment::Center)
            .spacing(12)
            .padding([8, 8]);

        scrollable(
            container(content)
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
