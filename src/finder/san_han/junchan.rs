use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::check_kuisagari;

pub struct Junchan {}

impl YakuBase for Junchan {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) | Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        return None;
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

        check_kuisagari(hand, "純全帯么九".to_string(), 3)
    }
}
