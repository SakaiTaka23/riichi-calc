use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::ii_han::bakaze::Bakaze;
use crate::finder::ii_han::chankan::Chankan;
use crate::finder::ii_han::chun::Chun;
use crate::finder::ii_han::haitei::Haitei;
use crate::finder::ii_han::haku::Haku;
use crate::finder::ii_han::hatu::Hatu;
use crate::finder::ii_han::hotei::Hotei;
use crate::finder::ii_han::iipeco::IIPeco;
use crate::finder::ii_han::ipatu::Ipatu;
use crate::finder::ii_han::pinfu::Pinfu;
use crate::finder::ii_han::riichi::Riichi;
use crate::finder::ii_han::rinshan::Rinshan;
use crate::finder::ii_han::tanyao::Tanyao;
use crate::finder::ii_han::tumo::Tumo;
use crate::finder::ii_han::zikaze::Zikaze;

mod riichi;
mod tanyao;
mod tumo;
mod zikaze;
mod bakaze;
mod haku;
mod hatu;
mod chun;
mod pinfu;
mod iipeco;
mod chankan;
mod rinshan;
mod haitei;
mod hotei;
mod ipatu;

pub fn ii_han_yaku(field: &Field, hand: &Hand, status: &Status) -> Option<Vec<(String, u8)>> {
    let mut yaku: Vec<(String, u8)> = Vec::new();

    if let Some(y) = Riichi::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Tanyao::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Tumo::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Zikaze::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Bakaze::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Haku::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Hatu::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Chun::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Pinfu::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = IIPeco::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Chankan::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Rinshan::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Haitei::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Hotei::validate(field, hand, status) { yaku.push(y); }
    if let Some(y) = Ipatu::validate(field, hand, status) { yaku.push(y); }

    if yaku.is_empty() {
        None
    } else {
        Some(yaku)
    }
}
