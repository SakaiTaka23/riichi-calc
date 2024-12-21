use crate::constants::tiles::Tile;

/// Field struct holds the game state
///
/// # Fields
///
/// - [zikaze](Zikaze) - self wind
/// - [bakaze](Bakaze) - field wind
/// - [honba](u8) - number of 本場
/// - [dora](Dora) - dora must be the dora itself
#[derive(Debug, Clone)]
pub struct Field {
    pub zikaze: Zikaze,
    pub bakaze: Bakaze,
    pub honba: u8,
    pub dora: Dora,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Wind {
    East = 1,
    South = 2,
    West = 3,
    North = 4,
}

/// 場風
pub type Bakaze = Wind;

/// 自風
pub type Zikaze = Wind;

/// ドラ length 1~4
pub type Dora = Vec<Tile>;
