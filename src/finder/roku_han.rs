use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::roku_han::chinitu::Chinitu;

mod chinitu;

pub fn roku_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let mut yaku = Vec::new();

    if let Some(y) = Chinitu::validate(field, winning_hand, status) { yaku.push(y); }

    yaku
}
