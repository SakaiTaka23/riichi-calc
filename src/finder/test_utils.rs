#![allow(unused_imports)]
use crate::constants::field::{Field, Wind};
use crate::constants::hand::{Hand, Mentsu, WinningHand};
use crate::constants::status::{RiichiStatus, Status, WinMethod};
use crate::constants::tiles::{Tile, TileType};
use rand::{random, Rng};
use std::collections::HashSet;

// Hand related utility functions
#[cfg(test)]
const ALL_TILE_VARIANTS: [TileType; 5] = [
    TileType::Manzu,
    TileType::Pinzu,
    TileType::Souzu,
    TileType::Wind,
    TileType::Dragon,
];

#[cfg(test)]
const SUHAI_TILE_VARIANTS: [TileType; 3] = [TileType::Manzu, TileType::Pinzu, TileType::Souzu];

#[cfg(test)]
pub fn random_tile_number(tile_type: &TileType) -> u8 {
    match tile_type {
        TileType::Manzu | TileType::Pinzu | TileType::Souzu => rand::thread_rng().gen_range(1..=9),
        TileType::Wind => rand::thread_rng().gen_range(1..=4),
        TileType::Dragon => rand::thread_rng().gen_range(1..=3),
    }
}

#[cfg(test)]
pub fn random_tile() -> Tile {
    let tile_type = random_tile_type();
    Tile {
        number: random_tile_number(&tile_type),
        tile_type,
    }
}

#[cfg(test)]
pub fn random_tile_type() -> TileType {
    ALL_TILE_VARIANTS[rand::thread_rng().gen_range(0..ALL_TILE_VARIANTS.len())]
}

#[cfg(test)]
pub fn random_suhai_tile_type() -> TileType {
    SUHAI_TILE_VARIANTS[rand::thread_rng().gen_range(0..SUHAI_TILE_VARIANTS.len())]
}

#[cfg(test)]
pub fn random_shuntu_number() -> u8 {
    rand::thread_rng().gen_range(1..=7)
}

#[cfg(test)]
pub fn random_shuntu(can_be_open: bool) -> Mentsu {
    Mentsu::Shuntsu(
        Tile {
            number: random_shuntu_number(),
            tile_type: SUHAI_TILE_VARIANTS
                [rand::thread_rng().gen_range(0..SUHAI_TILE_VARIANTS.len())],
        },
        if can_be_open { random() } else { false },
    )
}

#[cfg(test)]
pub fn random_shuntu_unique(can_be_open: bool, existing: Vec<Mentsu>) -> Mentsu {
    loop {
        let shuntu = random_shuntu(can_be_open);
        if !existing.contains(&shuntu) {
            return shuntu;
        }
    }
}

#[cfg(test)]
pub fn random_koutsu(can_be_open: bool, suhai_only: bool) -> Mentsu {
    let tile_type = if suhai_only {
        random_suhai_tile_type()
    } else {
        random_tile_type()
    };
    Mentsu::Koutsu(
        Tile {
            number: random_tile_number(&tile_type),
            tile_type,
        },
        if can_be_open { random() } else { false },
    )
}

#[cfg(test)]
pub fn random_koutsu_unique(can_be_open: bool, suhai_only: bool, existing: Vec<Mentsu>) -> Mentsu {
    loop {
        let koutsu = random_koutsu(can_be_open, suhai_only);
        if !existing.contains(&koutsu) {
            return koutsu;
        }
    }
}

#[cfg(test)]
pub fn random_kantsu(can_be_open: bool, suhai_only: bool) -> Mentsu {
    let tile_type = if suhai_only {
        random_suhai_tile_type()
    } else {
        random_tile_type()
    };
    Mentsu::Kantsu(
        Tile {
            number: random_tile_number(&tile_type),
            tile_type,
        },
        if can_be_open { random() } else { false },
    )
}

#[cfg(test)]
pub fn random_mentsu(can_be_open: bool, suhai_only: bool) -> Mentsu {
    match random::<u8>() % 3 {
        0 => random_shuntu(can_be_open),
        1 => random_koutsu(can_be_open, suhai_only),
        2 => random_kantsu(can_be_open, suhai_only),
        _ => unreachable!(),
    }
}

#[cfg(test)]
pub fn random_mentsu_unique(can_be_open: bool, suhai_only: bool, existing: Vec<Mentsu>) -> Mentsu {
    loop {
        let mentsu = random_mentsu(can_be_open, suhai_only);
        if !existing.contains(&mentsu) {
            return mentsu;
        }
    }
}

#[cfg(test)]
pub fn random_janto(suhai_only: bool) -> Mentsu {
    let tile_type = if suhai_only {
        random_suhai_tile_type()
    } else {
        random_tile_type()
    };
    Mentsu::Janto(Tile {
        number: random_tile_number(&tile_type),
        tile_type,
    })
}

#[cfg(test)]
pub fn from_hand(hand: Hand) -> WinningHand {
    WinningHand {
        hand,
        winning_tile: random_tile(),
        red_tile: 0,
    }
}

// Field related utility functions
#[cfg(test)]
pub fn random_wind() -> Wind {
    match random::<u8>() % 4 {
        0 => Wind::East,
        1 => Wind::South,
        2 => Wind::West,
        3 => Wind::North,
        _ => unreachable!(),
    }
}

#[cfg(test)]
pub fn random_field() -> Field {
    Field {
        honba: random::<u8>() % 3,
        zikaze: random_wind(),
        bakaze: random_wind(),
        dora: vec![random_tile()],
    }
}

// status related utility functions
#[cfg(test)]
pub fn random_status() -> Status {
    let riichi = match random::<u8>() % 3 {
        0 => RiichiStatus::NoRiichi,
        1 => RiichiStatus::Riichi(vec![random_tile()]),
        2 => RiichiStatus::DoubleRiichi(vec![random_tile()]),
        _ => unreachable!(),
    };
    let win_method = if random::<bool>() {
        WinMethod::Tumo
    } else {
        WinMethod::Ron
    };
    Status {
        riichi,
        win_method,
        special_win: HashSet::new(),
    }
}
