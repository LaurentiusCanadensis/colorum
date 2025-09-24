#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    R,
    G,
    B,
}

use iced::Event;

#[derive(Debug, Clone)]
pub enum Msg {
    PressedEnter,
    QueryChanged(String),
    PickChanged(String),
    CenterClicked,            // ⬅️ new
    PickedName(&'static str), // was String
    DropdownClicked(usize),   // mouse click row (position in results list)

    // text inputs
    RChanged(String),
    GChanged(String),
    BChanged(String),
    // wheels
    WheelChanged(Channel, u8),
    // search/dropdown
    SearchChanged(String),
    //PickedName(String),
    // misc
    Clear,
    CopyHex(String),
    OriginPicked(crate::colors_helper::Origin), // <— NEW

    KeyPressed(iced::keyboard::Key),
    KeyPressedWithMods(iced::keyboard::Key, iced::keyboard::Modifiers),
    MoveSelection(i32), // +1 down, -1 up; you can also use ±10 for paging
    ActivateSelection,  // Enter / Right
    OpenDropdown,
    CloseDropdown,
    RawEvent(Event),
    Tick,
    WindowResized(f32, f32), // width, height

    // New keyboard shortcuts
    FocusSearch,        // Ctrl+F - Focus search box
    CopyCurrentColor,   // Ctrl+C - Copy current color
    AdjustColorWheel(Channel, i8), // Arrow keys - Fine color adjustments

    // Recently used colors
    SelectFromHistory(String), // Select a color from history by hex string
}
