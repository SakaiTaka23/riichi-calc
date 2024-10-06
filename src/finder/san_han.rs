use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

mod riyan_peco;
mod junchan;
mod honitu;

pub fn san_han_yaku(field: &Field, hand: &Hand, status: &Status) -> Option<Vec<(String, u8)>> {
    let mut yaku = Vec::new();

    if let Some(y) = riyan_peco::RiyanPeco::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = junchan::Junchan::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = honitu::Hoinitu::validate(field, hand, status) { yaku.push(y); }

    if yaku.is_empty() {
        None
    } else {
        Some(yaku)
    }
}
