use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};

mod aka;
mod dora_finder;
mod omote;
mod ura;

pub fn dora_count(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let validators: Vec<YakuValidator> = vec![
        aka::Aka::validate,
        omote::Omote::validate,
        ura::Ura::validate,
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}
