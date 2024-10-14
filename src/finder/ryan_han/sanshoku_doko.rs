use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct SanshokuDoko {}

impl YakuBase for SanshokuDoko {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut suhai = Vec::new();
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) |
                Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        continue;
                    }

                    suhai.push(tile.number);
                }
                _ => continue
            }
        }

        for number in suhai.clone() {
            let count = count_u8_in_vec(number, &suhai);
            if count == 3 {
                return Some(("三色同順".to_string(), 2));
            }
        }

        None
    }
}

fn count_u8_in_vec(target: u8, vec: &Vec<u8>) -> usize {
    vec.iter().filter(|&&x| x == target).count()
}
