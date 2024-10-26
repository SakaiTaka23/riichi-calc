use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Aka {}

impl YakuBase for Aka {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if hand.red_tile > 0 {
            Some(("赤ドラ".to_string(), hand.red_tile))
        } else {
            None
        }
    }
}
