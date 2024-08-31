use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::{Status, WinMethod};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;

pub struct Tumo {}

impl YakuBase for Tumo {
    fn validate(_: &Field, hand: &Hand, status: &Status) -> Option<(String, u8)> {
        if !is_menzen(&hand) || status.win_method == WinMethod::Ron {
            return None;
        }

        Some(("門前清自摸和".to_string(), 1))
    }
}
