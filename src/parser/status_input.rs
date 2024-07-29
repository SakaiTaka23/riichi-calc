use crate::constants::status::{RiichiStatus, SpecialWin, WinMethod};
use crate::constants::status::SpecialWin::{DaiichiTumo, Haitei, Hotei, Ipatu, Rinshan};
use crate::parser::input_base::InputBase;

pub struct StatusInput {
    pub riichi: RiichiStatus,
    pub win_method: WinMethod,
    pub special_win: Vec<SpecialWin>,
}

impl InputBase for StatusInput {
    fn validate(&self) -> bool {
        let win = &self.special_win;
        if win.contains(&Ipatu) && win.contains(&Rinshan) {
            return false;
        }
        if win.contains(&Haitei) && win.contains(&Hotei) {
            return false;
        }
        if win.contains(&DaiichiTumo) && win.len() != 0 {
            return false;
        }

        true
    }
}
