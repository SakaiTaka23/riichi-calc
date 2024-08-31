use crate::constants::tiles::Tile;

pub struct Field {
    pub zikaze: Zikaze,
    pub bakaze: Bakaze,
    pub dora: Dora,
}

#[derive(PartialEq)]
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
