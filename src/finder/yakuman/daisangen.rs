use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Daisangen {}

impl YakuBase for Daisangen {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        let mut dragon_count = 0;

        for mentsu in hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon {
                        dragon_count += 1;
                    }
                }
                _ => continue
            }
        }

        if dragon_count == 3 {
            return Some(("大三元".to_string(), 13));
        }

        None
    }
}
