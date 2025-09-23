use crate::core::color_types::{HexCode, ColorName, Entity, Ordering};
use crate::colors_helper::Origin;

/// Persian color names with structured types.
/// Each entry is a tuple of (HexCode, ColorName) with sortable components.
pub const COLORS_PERSIAN: &[(HexCode, ColorName)] = &[
    (HexCode::new("#C81D11"), ColorName::new_full("Persian red", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#701C1C"), ColorName::new_full("Persian plum", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#F77FBE"), ColorName::new_full("Persian pink", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#FE28A2"), ColorName::new_full("Persian rose", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#00A693"), ColorName::new_full("Persian green", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#D99058"), ColorName::new_full("Persian orange", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#32127A"), ColorName::new_full("Persian indigo", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#1C39BB"), ColorName::new_full("Persian blue", Entity::Place, Origin::Persian, Ordering::Name)),
    (HexCode::new("#0067A5"), ColorName::new_full("Persian medium blue", Entity::Place, Origin::Persian, Ordering::Name)),
];
