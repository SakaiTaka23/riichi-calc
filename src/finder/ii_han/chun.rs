use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Haku {}

impl YakuBase for Haku {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            let tile = mentsu.tile();
            if tile.tile_type == TileType::Dragon && tile.number == 3 {
                return Some(("役牌:中".to_string(), 1));
            }
        }
        None
    }
}
