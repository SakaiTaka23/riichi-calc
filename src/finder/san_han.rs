use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};

mod riyan_peco;
mod junchan;
mod honitu;

pub fn san_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let validators: Vec<YakuValidator> = vec![
        riyan_peco::RiyanPeco::validate,
        junchan::Junchan::validate,
        honitu::Honitu::validate
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}
