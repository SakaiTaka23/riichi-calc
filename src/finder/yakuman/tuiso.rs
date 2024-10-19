use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Tuiso {}

impl YakuBase for Tuiso {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) |
                Mentsu::Shuntsu(tile, _) |
                Mentsu::Kantsu(tile, _) |
                Mentsu::Janto(tile) => {
                    if tile.tile_type != TileType::Dragon && tile.tile_type != TileType::Wind {
                        return None;
                    }
                }
            }
        }

        Some(("字一色".to_string(), 1))
    }
}
