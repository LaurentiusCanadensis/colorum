use iced::widget::svg;

#[cfg(feature = "outlined-logos")]
const WORDMARK: &[u8] = include_bytes!("../assets/logo_colorum_wordmark_outlined.svg");
#[cfg(not(feature = "outlined-logos"))]
const WORDMARK: &[u8] = include_bytes!("../assets/logo_colorum_wordmark.svg");

#[cfg(feature = "outlined-logos")]
const STACKED: &[u8]  = include_bytes!("../assets/logo_colorum_stacked_outlined.svg");
#[cfg(not(feature = "outlined-logos"))]
const STACKED: &[u8]  = include_bytes!("../assets/logo_colorum_stacked.svg");

#[cfg(feature = "outlined-logos")]
const MARK: &[u8]     = include_bytes!("../assets/logo_colorum_mark_outlined.svg");
#[cfg(not(feature = "outlined-logos"))]
const MARK: &[u8]     = include_bytes!("../assets/logo_colorum_mark.svg");

pub fn wordmark() -> svg::Handle { svg::Handle::from_memory(WORDMARK) }
pub fn stacked()  -> svg::Handle { svg::Handle::from_memory(STACKED) }
pub fn mark()     -> svg::Handle { svg::Handle::from_memory(MARK) }

