use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::Tile;
use crate::finder::finder_base::YakuBase;

pub struct Suanko {}

impl YakuBase for Suanko {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut janto: Option<Tile> = None;
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return None;
                }
                Mentsu::Koutsu(_, open) |
                Mentsu::Kantsu(_, open) => {
                    if open {
                        return None;
                    }
                }
                Mentsu::Janto(tile) => {
                    janto = Some(tile)
                }
            }
        }

        let janto = if janto.is_some() { janto.unwrap() } else { return None; };
        if hand.winning_tile == janto {
            Some(("四暗刻単騎".to_string(), 2))
        } else {
            Some(("四暗刻".to_string(), 1))
        }
    }
}
