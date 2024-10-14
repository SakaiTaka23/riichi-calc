use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::check_kuisagari;

pub struct Chanta {}

impl YakuBase for Chanta {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) | Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        continue;
                    } else if tile.number == 1 || tile.number == 9 {
                        continue
                    } else {
                        return None;
                    }
                }
                Mentsu::Shuntsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        // invalid
                        return None;
                    } else if tile.number != 1 || tile.number != 7 {
                        return None;
                    }
                }
            }
        }

        check_kuisagari(&hand.hand, "混全帯么九".to_string(), 2)
    }
}
