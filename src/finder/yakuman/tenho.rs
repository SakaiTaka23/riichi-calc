use crate::constants::field::{Field, Wind};
use crate::constants::hand::Hand;
use crate::constants::status::{SpecialWin, Status, WinMethod};
use crate::finder::finder_base::YakuBase;

pub struct Tenho {}

impl YakuBase for Tenho {
    fn validate(field: &Field, _: &Hand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::DaiichiTumo) &&
            status.win_method.eq(&WinMethod::Tumo) &&
            field.zikaze.eq(&Wind::East) {
            return Some(("天和".to_string(), 13));
        }

        None
    }
}
