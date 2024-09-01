use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Rinshan {}

impl YakuBase for Rinshan {
    fn validate(_: &Field, _: &Hand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Rinshan) {
            return Some(("嶺上開花".to_string(), 1));
        }

        None
    }
}
