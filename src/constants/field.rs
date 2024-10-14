use crate::constants::tiles::Tile;

#[derive(Clone)]
pub struct Field {
    pub zikaze: Zikaze,
    pub bakaze: Bakaze,
    pub dora: Dora,
}

#[derive(Clone, PartialEq)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

/// 場風
pub type Bakaze = Wind;

/// 自風
pub type Zikaze = Wind;

/// ドラ length 1~4
pub type Dora = Vec<Tile>;
