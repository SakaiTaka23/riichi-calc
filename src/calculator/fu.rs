use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;

mod agari;
mod futei;
mod machi;
mod mentsu;

trait FuBase {
    fn validate(field: &Field, hand: &WinningHand, status: &Status) -> u8;
}

pub fn calculate_fu(field: &Field, hand: &WinningHand, status: &Status) -> u8 {
    let fu = futei::Futei::validate(field, hand, status)
        + mentsu::Mentsu::validate(field, hand, status)
        + agari::Agari::validate(field, hand, status)
        + machi::Machi::validate(field, hand, status);

    round_10(fu)
}

fn round_10(value: u8) -> u8 {
    if value % 10 == 0 {
        value
    } else {
        (value / 10 + 1) * 10
    }
}
