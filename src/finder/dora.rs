use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

mod aka;
mod omote;
mod ura;
mod dora_finder;

pub fn dora_count(field: &Field, hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let mut dora = Vec::new();

    if let Some(y) = aka::Aka::validate(field, hand, status) { dora.push(y); }
    if let Some(y) = omote::Omote::validate(field, hand, status) { dora.push(y); }
    if let Some(y) = ura::Ura::validate(field, hand, status) { dora.push(y); }

    dora
}
