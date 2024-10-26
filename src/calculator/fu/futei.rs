use crate::calculator::fu::FuBase;
use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;

pub struct Futei {}

impl FuBase for Futei {
    fn validate(_: &Field, _: &WinningHand, _: &Status) -> u8 {
        20
    }
}
