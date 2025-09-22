use crate::color_types::{HexCode, ColorName};

/// Persian color names with structured types.
/// Each entry is a tuple of (HexCode, ColorName) with sortable components.
pub const COLORS_PERSIAN: &[(HexCode, ColorName)] = &[
    (HexCode::new("#C81D11"), ColorName::new("Persian red")),
    (HexCode::new("#701C1C"), ColorName::new("Persian plum")),
    (HexCode::new("#F77FBE"), ColorName::new("Persian pink")),
    (HexCode::new("#FE28A2"), ColorName::new("Persian rose")),
    (HexCode::new("#00A693"), ColorName::new("Persian green")),
    (HexCode::new("#D99058"), ColorName::new("Persian orange")),
    (HexCode::new("#32127A"), ColorName::new("Persian indigo")),
    (HexCode::new("#1C39BB"), ColorName::new("Persian blue")),
    (HexCode::new("#0067A5"), ColorName::new("Persian medium blue")),
];
