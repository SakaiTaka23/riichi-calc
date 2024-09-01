use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Bakaze {}

impl YakuBase for Bakaze {
    fn validate(field: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            let tile = mentsu.tile();
            if tile.tile_type != TileType::Wind { continue; }
            if is_same_wind(tile.number, &field.bakaze) {
                return Some(("役牌:場風牌".to_string(), 1));
            }
        }
        None
    }
}
