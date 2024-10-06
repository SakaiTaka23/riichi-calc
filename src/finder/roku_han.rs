use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::roku_han::chinitu::Chinitu;

mod chinitu;

pub fn roku_han_yaku(field: &Field, hand: &Hand, status: &Status) -> Option<Vec<(String, u8)>> {
    let mut yaku = Vec::new();

    if let Some(y) = Chinitu::validate(field, hand, status) { yaku.push(y); }

    if yaku.is_empty() {
        None
    } else {
        Some(yaku)
    }
}
