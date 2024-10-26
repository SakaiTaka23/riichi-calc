use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{RiichiStatus, Status};
use crate::finder::dora::dora_finder::find_dora;
use crate::finder::finder_base::YakuBase;

pub struct Ura {}

impl YakuBase for Ura {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)> {
        match &status.riichi {
            RiichiStatus::NoRiichi => {
                None
            }
            RiichiStatus::Riichi(dora) |
            RiichiStatus::DoubleRiichi(dora) => {
                let dora_count = find_dora(&dora, &hand.hand);
                Some(("裏ドラ".to_string(), dora_count))
            }
        }
    }
}
