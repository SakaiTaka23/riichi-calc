use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};
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

pub fn ii_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<(String, u8)> {
    let validators: Vec<YakuValidator> = vec![
        Riichi::validate,
        Tanyao::validate,
        Tumo::validate,
        Zikaze::validate,
        Bakaze::validate,
        Haku::validate,
        Hatu::validate,
        Chun::validate,
        Pinfu::validate,
        IIPeco::validate,
        Chankan::validate,
        Rinshan::validate,
        Haitei::validate,
        Hotei::validate,
        Ipatu::validate,
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}
