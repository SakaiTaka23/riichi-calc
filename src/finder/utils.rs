use crate::constants::field::Wind;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::{Tile, TileType};

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

pub fn check_kuisagari(hand: &Hand, yaku_name: String, han: u8) -> Option<(String, u8)> {
    let han = if is_menzen(hand) { han } else { han - 1 };

    Some((yaku_name, han))
}

/**
* return: Manzu, Pinzu, Sozu, WInd, Dragon
*/
pub fn split_colors(hand: &Hand) -> (Vec<Mentsu>, Vec<Mentsu>, Vec<Mentsu>, Vec<Mentsu>, Vec<Mentsu>) {
    let mut manzu = Vec::new();
    let mut pinzu = Vec::new();
    let mut sozu = Vec::new();
    let mut wind = Vec::new();
    let mut dragon = Vec::new();

    for mentsu in hand {
        if mentsu.tile().tile_type == TileType::Manzu {
            manzu.push(mentsu.clone());
        } else if mentsu.tile().tile_type == TileType::Pinzu {
            pinzu.push(mentsu.clone());
        } else if mentsu.tile().tile_type == TileType::Souzu {
            sozu.push(mentsu.clone());
        } else if mentsu.tile().tile_type == TileType::Wind {
            wind.push(mentsu.clone());
        } else if mentsu.tile().tile_type == TileType::Dragon {
            dragon.push(mentsu.clone());
        }
    }

    (manzu, pinzu, sozu, wind, dragon)
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
