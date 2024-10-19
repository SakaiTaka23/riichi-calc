use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;

pub struct Shosushi {}

impl YakuBase for Shosushi {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut wind_count = 0;
        let mut janto: Option<Tile> = None;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => { continue }
                Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Wind {
                        janto = Some(tile);
                        wind_count += 1;
                    } else {
                        continue;
                    }
                }
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Wind {
                        wind_count += 1;
                    }
                }
            }
        }

        if wind_count != 4 {
            return None;
        }

        if janto.is_some() {
            Some(("小四喜".to_string(), 1))
        } else {
            Some(("大四喜".to_string(), 2))
        }
    }
}
