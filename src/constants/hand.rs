use crate::constants::tiles::pi::Tile;

#[derive(Debug)]
pub struct KoutsuMentsu {
    pub tile: Tile,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Shuntsu {
    pub start_from: Tile,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Kantsu {
    pub tile: Tile,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Janto {
    pub tile: Tile,
}

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

pub enum WinMethod {
    Tumo,
    Ron,
}
