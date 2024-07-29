use crate::constants::tiles::pi::TileType;

#[derive(Debug)]
pub struct Koutsu {
    pub tile_type: TileType,
    pub tile_number: u8,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Shuntsu {
    pub start_from: TileType,
    pub start_number: u8,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Kantsu {
    pub tile_type: TileType,
    pub tile_number: u8,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Janto {
    pub tile_type: TileType,
    pub tile_number: u8,
}

#[derive(Debug)]
pub enum Mentsu {
    Koutsu(Koutsu),
    Shuntsu(Shuntsu),
    Kantsu(Kantsu),
    Janto(Janto),
}

pub enum WinMethod {
    Tumo,
    Ron,
}
