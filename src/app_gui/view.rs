use iced::Element;
use iced::widget::pick_list;
use crate::app_gui::App;
use crate::messages::Msg;
use crate::colors_helper::Origin;

impl App {


    pub fn view(&self) -> Element<Msg> {
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
        let wheel_view = wheel.view("RGB Wheel", &self.rr, &self.gg, &self.bb);


        // --- filtered names (FAST; uses your cached/indexed search) ---
        let filtered_names: Vec<&'static str> = self.filtered_names();

        // Keep selected if still present
        let selected_opt: Option<&'static str> = self.selected_name.as_deref().and_then(|cur| {
            filtered_names
                .iter()
                .copied()
                .find(|s| s.eq_ignore_ascii_case(cur))
        });

        // Origins dropdown
        let origins_list = {
            let mut v = vec![
                Origin::All,
                Origin::Css,
                Origin::XKCD,
                Origin::Pantone,
                Origin::Hindi,
                Origin::Persian,
                Origin::National,
                Origin::Brands,

            ];
            #[cfg(feature = "github-colors")]
            {
                v.push(Origin::GitHub);
            }
            v
        };
        let origin_dd = pick_list(origins_list, Some(self.selected_origin), Msg::OriginPicked)
            .placeholder("Origin")
            .width(Length::Shrink);
        use iced::widget::{column, container, row, scrollable, text, text_input, mouse_area, Space};
        use iced::{Alignment, Length, Background, Color};
        use iced::border;

        // Search box
        let search_box = text_input("Search color name…", &self.query)
            .on_input(Msg::QueryChanged)
            .on_submit(Msg::PressedEnter)
            .padding(8)
            .width(Length::Fill);

        // Index-driven dropdown (no recompute here)
        fn view_dropdown(app: &App) -> iced::Element<Msg> {
            if app.results_idx.is_empty() {
                return Space::with_height(0).into();
            }

            let mut col = column![]
                .spacing(2)
                .padding(4)
                .align_x(Alignment::Start)
                .width(Length::Fill);

            for (row, &idx) in app.results_idx.iter().enumerate() {
                let (hex, name) = app.base[idx];
                let is_sel = app.sel_pos == Some(row);
                let label = if is_sel { format!("▶ {}  {}", name, hex) } else { format!("{}  {}", name, hex) };

                let row_body = container(text(label))
                    .padding([6, 8])
                    .width(Length::Fill)
                    .style(move |_theme: &iced::Theme| {
                        if is_sel {
                            iced::widget::container::Style {
                                background: Some(Background::Color(Color { r: 0.20, g: 0.40, b: 0.80, a: 0.20 })),
                                border: border::Border { radius: 8.0.into(), ..Default::default() },
                                ..Default::default()
                            }
                        } else {
                            iced::widget::container::Style::default()
                        }
                    });

                let click = mouse_area(row_body).on_press(Msg::DropdownClicked(row));
                col = col.push(click);
            }

            scrollable(col)
                .id(app.dropdown_scroll_id.clone())    // <— required
                .height(Length::Fixed(220.0))
                .width(Length::Fill)
                .into()
        }

        let dropdown = view_dropdown(self);

        // Put search + dropdown together
        let stacked_controls = container(
            column![
        search_box,
        dropdown,       // single dropdown only (index-driven)
    ]
                .spacing(8)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
            .padding([4, 8])
            .width(Length::Fill)
            .align_x(Alignment::Center);

        // Keep the rest of your layout as you like:
        let clear_btn = iced::widget::button("Clear")
            .on_press(Msg::Clear)
            .padding([8, 12]);

        let content = column![
    container(wheel_view)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding([4, 0]),

    stacked_controls,

    row![origin_dd, clear_btn]
        .spacing(10)
        .align_y(Alignment::Center)
        .width(Length::Shrink),
]
            .align_x(Alignment::Center)
            .spacing(12)
            .padding([8, 8]);

        scrollable(container(content).width(Length::Fill).align_x(Alignment::Center)).into()

    }
}



