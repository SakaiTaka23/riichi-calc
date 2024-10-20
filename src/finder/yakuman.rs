use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

mod tenho;
mod chiho;
mod daisangen;
mod suanko;
mod tuiso;
mod ryuiso;
mod chinroto;
mod shosushi;
mod sukantu;
mod churen;

pub fn yakuman_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let mut yaku = Vec::new();

    if let Some(y) = tenho::Tenho::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = chiho::Chiho::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = daisangen::Daisangen::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = suanko::Suanko::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = tuiso::Tuiso::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = ryuiso::Ryuiso::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = chinroto::ChinRoto::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = shosushi::Shosushi::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = sukantu::Sukantu::validate(field, winning_hand, status) { yaku.push(y); }
    if let Some(y) = churen::Churen::validate(field, winning_hand, status) { yaku.push(y); }

    yaku
}
