#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    R,
    G,
    B,
}

#[derive(Debug, Clone)]
pub enum Msg {
    QueryChanged(String),
    PickChanged(String),
    CenterClicked, // ⬅️ new
    PickedName(&'static str),   // was String


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
}
