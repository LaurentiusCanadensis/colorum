use crate::app_gui::App;
use crate::colors_helper::Origin;
use crate::messages::Msg;
use iced::Element;
use iced::widget::pick_list;

mod name_picker_widget {
    use super::*;
    use crate::app_gui::view::helpers::{filtered_names_for_origin, retain_selection};
    use crate::colors_helper::{
        HEAVY_MIN_QUERY, MAX_RESULTS, TokenMode, is_heavy_origin, origin_names, origin_rank,
        search_in_origin,
    };
    use iced::widget::pick_list::PickList;
    use iced::widget::{column, pick_list, text_input};
    use iced::{Alignment, Element, Length, Renderer, Theme};

    /// Combined search + name dropdown as a single widget.

    /// Search + dropdown as a single element.
    pub fn render<'a>(
        origin: Origin,
        search: &'a str,
        current_selection: Option<&'a str>,
    ) -> Element<'a, Msg> {
        let names = filtered_names_for_origin(origin, search);
        let selected_opt = retain_selection(current_selection, &names);

        let on_select: fn(&'static str) -> Msg = Msg::PickedName;

        let picker: PickList<&'static str, Vec<&'static str>, &'static str, Msg, Theme, Renderer> =
            pick_list(names, selected_opt, on_select)
                .placeholder({
                    if is_heavy_origin(origin) && search.trim().len() < HEAVY_MIN_QUERY {
                        "Type at least 1 letter…"
                    } else {
                        "Select a color"
                    }
                })
                .width(Length::Fill);

        picker.into()
    }
}
