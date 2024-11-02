use crate::constants::status::RiichiStatus::{DoubleRiichi, NoRiichi, Riichi};
use crate::constants::status::SpecialWin::{Chankan, DaiichiTumo, Haitei, Hotei, Ipatu, Rinshan};
use crate::constants::status::Status;
use crate::constants::status::WinMethod::{Ron, Tumo};
use crate::parser::InputBase;

impl InputBase for Status {
    fn validate(&self) -> bool {
        match &self.riichi {
            NoRiichi => {}
            Riichi(x) => {
                if x.len() > 4 {
                    return false;
                }
            }
            DoubleRiichi(x) => {
                if x.len() > 4 {
                    return false;
                }
            }
        }

        let win = &self.special_win;
        if win.contains(&Ipatu) && win.contains(&Rinshan) {
            return false;
        }
        if win.contains(&Ipatu) && self.riichi == NoRiichi {
            return false;
        }

        if win.contains(&Rinshan) && win.contains(&Chankan) {
            return false;
        }
        if win.contains(&Chankan) && self.win_method != Ron {
            return false;
        }
        if win.contains(&Rinshan) && self.win_method != Tumo {
            return false;
        }

        if win.contains(&Haitei) && win.contains(&Hotei) {
            return false;
        }
        if win.contains(&Haitei) && self.win_method != Tumo {
            return false;
        }
        if win.contains(&Hotei) && self.win_method != Ron {
            return false;
        }

        if win.contains(&DaiichiTumo)
            && (win.len() != 1
            || self.riichi != NoRiichi
            || self.win_method != Tumo) {
            return false;
        }

        true
    }
}

#[cfg(test)]
pub mod status_input_utils {
    use super::*;
    use crate::constants::status::{SpecialWin, WinMethod};
    use crate::constants::tiles::Tile;
    use crate::constants::tiles::TileType::Manzu;

    pub fn build_status_input(is_riichi: bool, win_method: WinMethod, special_win: Vec<SpecialWin>) -> Status {
        let riichi = if is_riichi { Riichi(vec![Tile { number: 1, tile_type: Manzu }]) } else { NoRiichi };
        Status {
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

        let input = build_status_input(false, Ron, vec![]);
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

        let mut input = build_status_input(false, Ron, vec![]);
        input.riichi = Riichi(vec![
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

        let mut input = build_status_input(false, Ron, vec![]);
        input.riichi = DoubleRiichi(vec![
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

        let input = build_status_input(false, Ron, vec![Ipatu]);
        assert!(!input.validate());
    }

    #[test]
    fn ipatu_rinshan_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let mut input = build_status_input(true, Ron, vec![Ipatu, Rinshan]);
        input.riichi = Riichi(vec![]);
        assert!(!input.validate());
    }
}

mod kan_related {
    #[test]
    fn chakan_rinshan_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Chankan, Rinshan]);
        assert!(!input.validate());
    }

    #[test]
    fn chakan_ron_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Chankan]);
        assert!(input.validate());
    }

    #[test]
    fn chakan_tumo_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![Chankan]);
        assert!(!input.validate());
    }

    #[test]
    fn rinshan_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Rinshan]);
        assert!(!input.validate());
    }

    #[test]
    fn rinshan_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![Rinshan]);
        assert!(input.validate());
    }
}

mod kawa_related {
    #[test]
    fn haitei_hotei_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Haitei, Hotei]);
        assert!(!input.validate());
    }

    #[test]
    fn haitei_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Haitei]);
        assert!(!input.validate());
    }

    #[test]
    fn haitei_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![Haitei]);
        assert!(input.validate());
    }

    #[test]
    fn hotei_ron_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![Hotei]);
        assert!(input.validate());
    }

    #[test]
    fn hotei_tumo_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![Hotei]);
        assert!(!input.validate());
    }
}

mod tenchiho_related {
    #[test]
    fn daiichi_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![DaiichiTumo]);
        assert!(input.validate());
    }


    #[test]
    fn daiichi_tumo_riichi_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(true, Tumo, vec![DaiichiTumo]);
        assert!(!input.validate());
    }
    #[test]
    fn daiichi_tumo_ron_invalid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(true, Ron, vec![DaiichiTumo]);
        assert!(!input.validate());
    }
}
