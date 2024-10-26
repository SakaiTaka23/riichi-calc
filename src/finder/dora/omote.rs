use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::dora::dora_finder::find_dora;
use crate::finder::finder_base::YakuBase;

pub struct Omote {}

impl YakuBase for Omote {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let dora_count = find_dora(&field.dora, &hand.hand);

        if dora_count > 0 {
            Some(("ドラ".to_string(), dora_count))
        } else {
            None
        }
    }
}
