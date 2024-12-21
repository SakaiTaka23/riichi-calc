use crate::constants::tiles::Tile;
use std::fmt;

///
/// # Fields
///
/// - [hand](Hand) - the hand this contains the winning_tile
/// - [winning_tile](Tile) - the winning tile
/// - [red_tile](u8) - number red tile in the hand
///
#[derive(Debug)]
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

impl fmt::Display for Mentsu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mentsu::Koutsu(tile, is_open) => {
                write!(
                    f,
                    "[{}{}{}]{}",
                    tile,
                    tile,
                    tile,
                    if *is_open { "!" } else { "" }
                )
            }
            Mentsu::Shuntsu(start_tile, is_open) => {
                let start_tile = *start_tile;
                let mut second_tile = start_tile;
                second_tile.number += 1;
                let mut third_tile = second_tile;
                third_tile.number += 1;
                write!(
                    f,
                    "[{}{}{}]{}",
                    start_tile,
                    second_tile,
                    third_tile,
                    if *is_open { "!" } else { "" }
                )
            }
            Mentsu::Kantsu(tile, is_open) => {
                write!(
                    f,
                    "[{}{}{}{}]{}",
                    tile,
                    tile,
                    tile,
                    tile,
                    if *is_open { "!" } else { "" }
                )
            }
            Mentsu::Janto(tile) => {
                write!(f, "[{}{}]", tile, tile)
            }
        }
    }
}
