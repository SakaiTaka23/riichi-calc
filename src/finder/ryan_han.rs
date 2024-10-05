use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
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

pub fn ryan_han_yaku(field: &Field, hand: &Hand, status: &Status) -> Option<Vec<(String, u8)>> {
    let mut yaku: Vec<(String, u8)> = Vec::new();

    if let Some(y) = DoubleRiichi::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = SanshokuDoko::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Sankantu::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = ToiToi::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Sananko::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Shosangen::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Honroto::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Chanta::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Ixtukitukan::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = SanshokuDojun::validate(field, hand, status) { yaku.push(y); }

    if yaku.is_empty() {
        None
    } else {
        Some(yaku)
    }
}
