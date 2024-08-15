use crate::constants::tiles::Tile;

#[derive(Clone, Debug)]
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

pub type Hand = [Mentsu; 5];
