use crate::constants::tiles::Tile;

pub struct WinningHand {
    pub hand: Hand,
    pub winning_tile: Tile,
    pub red_tile: u8,
}

pub type Hand = [Mentsu; 5];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Mentsu {
    /// 刻子 `bool` defines if it is open
    Koutsu(Tile, bool),
    /// 順子 `Tile` defines the start of the mentsu `bool` defines if it is open
    Shuntsu(Tile, bool),
    /// 槓子 `bool` defines if it is open
    Kantsu(Tile, bool),
    /// 雀頭 `bool` defines if it is open
    Janto(Tile),
}
