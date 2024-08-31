use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::ii_han::riichi::Riichi;
use crate::finder::ii_han::tanyao::Tanyao;

mod riichi;
mod tanyao;
mod tumo;
mod zikaze;
mod bakaze;

fn ii_han_yaku(field: &Field, hand: &Hand, status: &Status) {
    Riichi::validate(field, hand, status);
    Tanyao::validate(field, hand, status);
}
