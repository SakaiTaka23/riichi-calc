use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Pinfu {}

impl YakuBase for Pinfu {
    fn validate(field: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            match mentsu {
                Mentsu::Shuntsu(_, open) => { if *open { return None; } }
                Mentsu::Janto(tile) => { if !Self::is_valid_janto(tile, field) { return None; } }
                _ => { return None }
            }
        }
        Some(("平和".to_string(), 1))
    }
}

impl Pinfu {
    fn is_valid_janto(tile: &Tile, field: &Field) -> bool {
        match tile.tile_type {
            TileType::Manzu | TileType::Pinzu | TileType::Souzu => { true }
            TileType::Wind => {
                if is_same_wind(tile.number, &field.bakaze) || is_same_wind(tile.number, &field.zikaze) {
                    return false;
                }
                true
            }
            TileType::Dragon => { false }
        }
    }
}
