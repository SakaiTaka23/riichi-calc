use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct ChinRoto {}

impl YakuBase for ChinRoto {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) |
                Mentsu::Shuntsu(tile, _) |
                Mentsu::Kantsu(tile, _) |
                Mentsu::Janto(tile) => {
                    if (tile.tile_type == TileType::Manzu || tile.tile_type == TileType::Pinzu || tile.tile_type == TileType::Souzu)
                        && (tile.number != 1 || tile.number != 9) {
                        return None;
                    } else if tile.tile_type == TileType::Wind || tile.tile_type == TileType::Dragon {
                        return None;
                    }
                }
            }
        }

        Some(("清老頭".to_string(), 1))
    }
}
