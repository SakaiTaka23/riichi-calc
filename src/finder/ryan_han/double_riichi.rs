use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::RiichiStatus::DoubleRiichi as DoubleRiichiStatus;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct DoubleRiichi {}

impl YakuBase for DoubleRiichi {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        match status.riichi {
            DoubleRiichiStatus(_) => {
                return Some(("ダブルリーチ".to_string(), 2))
            }
            _ => None
        }
    }
}
