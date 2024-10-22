use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Zikaze {}

impl YakuBase for Zikaze {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            let tile = mentsu.tile();
            if tile.tile_type != TileType::Wind { continue; }
            if is_same_wind(tile.number, &field.zikaze) {
                return Some(("役牌:自風牌".to_string(), 1));
            }
        }
        None
    }
}
