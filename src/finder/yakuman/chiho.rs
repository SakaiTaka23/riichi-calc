use crate::constants::field::{Field, Wind};
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status, WinMethod};
use crate::finder::finder_base::YakuBase;

pub struct Chiho {}

impl YakuBase for Chiho {
    fn validate(field: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::DaiichiTumo) &&
            status.win_method.eq(&WinMethod::Tumo) &&
            field.zikaze.ne(&Wind::East) {
            return Some(("地和".to_string(), 13));
        }

        None
    }
}
