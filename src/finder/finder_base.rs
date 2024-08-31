use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;

pub trait YakuBase {
    /// check if certain yaku in valid
    ///
    /// returns yaku name han count if valid
    fn validate(field: &Field, hand: &Hand, status: &Status) -> Option<(String, u8)>;
}
