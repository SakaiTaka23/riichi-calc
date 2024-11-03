use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;

pub type YakuValidator = fn(&Field, &WinningHand, &Status) -> Option<(String, u8)>;

pub trait YakuBase {
    /// check if certain yaku in valid
    ///
    /// returns yaku name han count if valid
    fn validate(field: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)>;
}
