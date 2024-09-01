use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Chankan {}

impl YakuBase for Chankan {
    fn validate(_: &Field, _: &Hand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Chakan) {
            return Some(("搶槓".to_string(), 1));
        }

        None
    }
}
