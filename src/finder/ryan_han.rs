use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};
use crate::finder::ryan_han::chanta::Chanta;
use crate::finder::ryan_han::double_riichi::DoubleRiichi;
use crate::finder::ryan_han::honroto::Honroto;
use crate::finder::ryan_han::ixtukitukan::Ixtukitukan;
use crate::finder::ryan_han::sananko::Sananko;
use crate::finder::ryan_han::sankantu::Sankantu;
use crate::finder::ryan_han::sanshoku_dojun::SanshokuDojun;
use crate::finder::ryan_han::sanshoku_doko::SanshokuDoko;
use crate::finder::ryan_han::shosangen::Shosangen;
use crate::finder::ryan_han::toitoi::ToiToi;

mod double_riichi;
mod sanshoku_doko;
mod sankantu;
mod toitoi;
mod sananko;
mod shosangen;
mod honroto;
mod chanta;
mod ixtukitukan;
mod sanshoku_dojun;

pub fn ryan_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let validators: Vec<YakuValidator> = vec![
        DoubleRiichi::validate,
        SanshokuDoko::validate,
        Sankantu::validate,
        ToiToi::validate,
        Sananko::validate,
        Shosangen::validate,
        Honroto::validate,
        Chanta::validate,
        Ixtukitukan::validate,
        SanshokuDojun::validate
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}
