use crate::constants::tiles::Tile;

///
/// # Fields
///
/// - [hand](Hand) - the hand this contains the winning_tile
/// - [winning_tile](Tile) - the winning tile
/// - [red_tile](u8) - number red tile in the hand
///
pub struct WinningHand {
    pub hand: Hand,
    pub winning_tile: Tile,
    pub red_tile: u8,
}

pub type Hand = [Mentsu; 5];

///
/// 面子
/// `bool` defines if it is open
///
/// # Types
///
/// - [Koutsu](Mentsu::Koutsu)
/// - [Shuntsu](Mentsu::Shuntsu)
/// - [Kantsu](Mentsu::Kantsu)
/// - [Janto](Mentsu::Janto)
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Mentsu {
    /// 刻子
    Koutsu(Tile, bool),
    /// 順子 [Tile] defines the start of the mentsu
    Shuntsu(Tile, bool),
    /// 槓子
    Kantsu(Tile, bool),
    /// 雀頭
    Janto(Tile),
}
