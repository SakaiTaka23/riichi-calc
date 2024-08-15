use std::collections::HashSet;

use crate::constants::status::SpecialWin::{Chakan, DaiichiTumo, Haitei, Hotei, Ipatu, Rinshan};
use crate::constants::status::{Dora, RiichiStatus, SpecialWin, WinMethod};
use crate::parser::input_base::InputBase;

pub struct StatusInput {
    pub dora: Dora,
    pub riichi: RiichiStatus,
    pub win_method: WinMethod,
    pub special_win: HashSet<SpecialWin>,
}

impl InputBase for StatusInput {
    fn validate(&self) -> bool {
        match &self.riichi {
            RiichiStatus::NoRiichi => {}
            RiichiStatus::Riichi(x) => {
                if x.len() > 4 {
                    return false;
                }
            }
            RiichiStatus::DoubleRiichi(x) => {
                if x.len() > 4 {
                    return false;
                }
            }
        }

        let win = &self.special_win;
        if win.contains(&Ipatu) && win.contains(&Rinshan) {
            return false;
        }
        if win.contains(&Ipatu) && self.riichi == RiichiStatus::NoRiichi {
            return false;
        }

        if win.contains(&Rinshan) && win.contains(&Chakan) {
            return false;
        }
        if win.contains(&Chakan) && self.win_method != WinMethod::Ron {
            return false;
        }
        if win.contains(&Rinshan) && self.win_method != WinMethod::Tumo {
            return false;
        }

        if win.contains(&Haitei) && win.contains(&Hotei) {
            return false;
        }
        if win.contains(&Haitei) && self.win_method != WinMethod::Tumo {
            return false;
        }
        if win.contains(&Hotei) && self.win_method != WinMethod::Ron {
            return false;
        }

        if win.contains(&DaiichiTumo)
            && (win.len() != 1
            || self.riichi != RiichiStatus::NoRiichi
            || self.win_method != WinMethod::Tumo) {
            return false;
        }

        true
    }
}

#[cfg(test)]
pub mod status_input_utils {
    use super::*;
    use crate::constants::tiles::Tile;
    use crate::constants::tiles::TileType::Manzu;

    pub fn build_status_input(is_riichi: bool, win_method: WinMethod, special_win: Vec<SpecialWin>) -> StatusInput {
        let riichi = if is_riichi { RiichiStatus::Riichi(vec![Tile { number: 1, tile_type: Manzu }]) } else { RiichiStatus::NoRiichi };
        StatusInput {
            dora: vec![Tile { number: 1, tile_type: Manzu }],
            riichi,
            win_method,
            special_win: special_win.into_iter().collect(),
        }
    }
}

mod correct {
    #[test]
    fn correct_input() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![]);
        assert!(input.validate());
    }
}

#[allow(unused_imports)]
mod dora_related {
    #[test]
    fn too_many_dora_riichi() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType::Manzu;

        let mut input = build_status_input(false, WinMethod::Ron, vec![]);
        input.riichi = RiichiStatus::Riichi(vec![
            Tile { number: 1, tile_type: Manzu },
            Tile { number: 2, tile_type: Manzu },
            Tile { number: 3, tile_type: Manzu },
            Tile { number: 4, tile_type: Manzu },
            Tile { number: 5, tile_type: Manzu },
        ]);
        assert!(!input.validate());
    }

    #[test]
    fn too_many_dora_double_riichi() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType::Manzu;

        let mut input = build_status_input(false, WinMethod::Ron, vec![]);
        input.riichi = RiichiStatus::DoubleRiichi(vec![
            Tile { number: 1, tile_type: Manzu },
            Tile { number: 2, tile_type: Manzu },
            Tile { number: 3, tile_type: Manzu },
            Tile { number: 4, tile_type: Manzu },
            Tile { number: 5, tile_type: Manzu },
        ]);
        assert!(!input.validate());
    }
}

mod ipatu_related {
    #[test]
    fn ipatu_without_riichi_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Ipatu]);
        assert!(!input.validate());
    }

    #[test]
    fn ipatu_rinshan_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let mut input = build_status_input(true, WinMethod::Ron, vec![Ipatu, Rinshan]);
        input.riichi = RiichiStatus::Riichi(vec![]);
        assert!(!input.validate());
    }
}

mod kan_related {
    #[test]
    fn chakan_rinshan_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Chakan, Rinshan]);
        assert!(!input.validate());
    }

    #[test]
    fn chakan_ron_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Chakan]);
        assert!(input.validate());
    }

    #[test]
    fn chakan_tumo_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Tumo, vec![Chakan]);
        assert!(!input.validate());
    }

    #[test]
    fn rinshan_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Rinshan]);
        assert!(!input.validate());
    }

    #[test]
    fn rinshan_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Tumo, vec![Rinshan]);
        assert!(input.validate());
    }
}

mod kawa_related {
    #[test]
    fn haitei_hotei_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Haitei, Hotei]);
        assert!(!input.validate());
    }

    #[test]
    fn haitei_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Haitei]);
        assert!(!input.validate());
    }

    #[test]
    fn haitei_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Tumo, vec![Haitei]);
        assert!(input.validate());
    }

    #[test]
    fn hotei_ron_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Ron, vec![Hotei]);
        assert!(input.validate());
    }

    #[test]
    fn hotei_tumo_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Tumo, vec![Hotei]);
        assert!(!input.validate());
    }
}

mod tenchiho_related {
    #[test]
    fn daiichi_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, WinMethod::Tumo, vec![DaiichiTumo]);
        assert!(input.validate());
    }


    #[test]
    fn daiichi_tumo_riichi_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(true, WinMethod::Tumo, vec![DaiichiTumo]);
        assert!(!input.validate());
    }
    #[test]
    fn daiichi_tumo_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(true, WinMethod::Ron, vec![DaiichiTumo]);
        assert!(!input.validate());
    }
}
