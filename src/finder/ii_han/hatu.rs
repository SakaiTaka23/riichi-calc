use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Hatu {}

impl YakuBase for Hatu {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            let tile = mentsu.tile();
            if tile.tile_type == TileType::Dragon && tile.number == 2 {
                return Some(("役牌:發".to_string(), 1));
            }
        }
        None
    }
}
