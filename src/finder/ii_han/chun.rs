use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Chun {}

impl YakuBase for Chun {
    fn validate(_: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in winning_hand.hand {
            let tile = mentsu.tile();
            if tile.tile_type == TileType::Dragon && tile.number == 3 {
                return Some(("役牌:中".to_string(), 1));
            }
        }
        None
    }
}
