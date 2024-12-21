use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};

mod chiho;
mod chinroto;
mod churen;
mod daisangen;
mod ryuiso;
mod shosushi;
mod suanko;
mod sukantu;
mod tenho;
mod tuiso;

pub fn yakuman_yaku(
    field: &Field,
    winning_hand: &WinningHand,
    status: &Status,
) -> Vec<(String, u8)> {
    let validators: Vec<YakuValidator> = vec![
        tenho::Tenho::validate,
        chiho::Chiho::validate,
        daisangen::Daisangen::validate,
        suanko::Suanko::validate,
        tuiso::Tuiso::validate,
        ryuiso::Ryuiso::validate,
        chinroto::ChinRoto::validate,
        shosushi::Shosushi::validate,
        sukantu::Sukantu::validate,
        churen::Churen::validate,
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}
