use crate::constants::field::Wind;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::Tile;

pub fn is_menzen(hand: &Hand) -> bool {
    for mentsu in hand {
        match mentsu {
            Mentsu::Koutsu(_, x)
            | Mentsu::Shuntsu(_, x)
            | Mentsu::Kantsu(_, x) => { if *x { return false; } }
            Mentsu::Janto(_) => {}
        }
    }

    true
}

pub fn is_same_wind(tile_number: u8, wind: &Wind) -> bool {
    match tile_number {
        1 => wind == &Wind::East,
        2 => wind == &Wind::South,
        3 => wind == &Wind::West,
        4 => wind == &Wind::North,
        _ => false
    }
}

impl Mentsu {
    pub fn tile(&self) -> Tile {
        match self {
            Mentsu::Koutsu(tile, _) => tile.clone(),
            Mentsu::Shuntsu(tile, _) => tile.clone(),
            Mentsu::Kantsu(tile, _) => tile.clone(),
            Mentsu::Janto(tile) => tile.clone(),
        }
    }
}