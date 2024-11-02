use crate::constants::status::RiichiStatus::{DoubleRiichi, NoRiichi, Riichi};
use crate::constants::status::SpecialWin::{Chankan, DaiichiTumo, Haitei, Hotei, Ipatu, Rinshan};
use crate::constants::status::WinMethod::{Ron, Tumo};
use crate::constants::status::{SpecialWin, Status, WinMethod};
use crate::parser::ValidationError::{InvalidWinCombination, OutOfRange};
use crate::parser::{InputBase, ValidationError};
use std::fmt::Display;

impl InputBase for Status {
    fn validate(&self) -> Result<(), ValidationError> {
        match &self.riichi {
            NoRiichi => {}
            Riichi(x) => {
                if x.len() > 4 {
                    return Err(OutOfRange("Number of dora should be less than or equal to 4".to_string()));
                }
            }
            DoubleRiichi(x) => {
                if x.len() > 4 {
                    return Err(OutOfRange("Number of dora should be less than or equal to 4".to_string()));
                }
            }
        }

        let win = &self.special_win;
        if win.contains(&Ipatu) && win.contains(&Rinshan) {
            return Err(InvalidWinCombination(Ipatu.to_string(), Rinshan.to_string()));
        }
        if win.contains(&Ipatu) && self.riichi == NoRiichi {
            return Err(InvalidWinCombination(Ipatu.to_string(), "No riichi".to_string()));
        }

        if win.contains(&Rinshan) && win.contains(&Chankan) {
            return Err(InvalidWinCombination(Rinshan.to_string(), Chankan.to_string()));
        }
        if win.contains(&Chankan) && self.win_method != Ron {
            return Err(InvalidWinCombination(Chankan.to_string(), Tumo.to_string()));
        }
        if win.contains(&Rinshan) && self.win_method != Tumo {
            return Err(InvalidWinCombination(Rinshan.to_string(), Ron.to_string()));
        }

        if win.contains(&Haitei) && win.contains(&Hotei) {
            return Err(InvalidWinCombination(Haitei.to_string(), Hotei.to_string()));
        }
        if win.contains(&Haitei) && self.win_method != Tumo {
            return Err(InvalidWinCombination(Haitei.to_string(), Ron.to_string()));
        }
        if win.contains(&Hotei) && self.win_method != Ron {
            return Err(InvalidWinCombination(Hotei.to_string(), Tumo.to_string()));
        }

        if win.contains(&DaiichiTumo)
            && (win.len() != 1
            || self.riichi != NoRiichi
            || self.win_method != Tumo) {
            return Err(InvalidWinCombination(DaiichiTumo.to_string(), "Other win combination".to_string()));
        }

        Ok(())
    }
}

impl Display for SpecialWin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ipatu => write!(f, "一発"),
            Chankan => write!(f, "搶槓"),
            Rinshan => write!(f, "嶺上開花"),
            Haitei => write!(f, "海底自摸"),
            Hotei => write!(f, "河底撈魚"),
            DaiichiTumo => write!(f, "第一自摸"),
        }
    }
}

impl Display for WinMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ron => write!(f, "ロン"),
            Tumo => write!(f, "ツモ"),
        }
    }
}

#[cfg(test)]
mod status_input_utils {
    use crate::constants::status::RiichiStatus::{NoRiichi, Riichi};
    use crate::constants::status::{SpecialWin, Status, WinMethod};
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

#[cfg(test)]
mod correct {
    use crate::constants::status::WinMethod::Ron;
    use crate::parser::InputBase;

    #[test]
    fn correct_input() {
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Ron, vec![]);
        assert!(input.validate().is_ok());
    }
}

#[cfg(test)]
mod dora_related {
    use crate::constants::status::RiichiStatus::{DoubleRiichi, Riichi};
    use crate::constants::status::WinMethod::Ron;
    use crate::constants::tiles::Tile;
    use crate::constants::tiles::TileType::Manzu;
    use crate::parser::status_input::status_input_utils::build_status_input;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::OutOfRange;

    #[test]
    fn too_many_uradora_riichi() {
        let mut input = build_status_input(false, Ron, vec![]);
        input.riichi = Riichi(vec![
            Tile { number: 1, tile_type: Manzu },
            Tile { number: 2, tile_type: Manzu },
            Tile { number: 3, tile_type: Manzu },
            Tile { number: 4, tile_type: Manzu },
            Tile { number: 5, tile_type: Manzu },
        ]);
        assert_eq!(input.validate(), Err(OutOfRange("Number of dora should be less than or equal to 4".to_string())));
    }

    #[test]
    fn too_many_uradora_double_riichi() {
        let mut input = build_status_input(false, Ron, vec![]);
        input.riichi = DoubleRiichi(vec![
            Tile { number: 1, tile_type: Manzu },
            Tile { number: 2, tile_type: Manzu },
            Tile { number: 3, tile_type: Manzu },
            Tile { number: 4, tile_type: Manzu },
            Tile { number: 5, tile_type: Manzu },
        ]);
        assert_eq!(input.validate(), Err(OutOfRange("Number of dora should be less than or equal to 4".to_string())));
    }
}

#[cfg(test)]
mod ipatu_related {
    use crate::constants::status::RiichiStatus::Riichi;
    use crate::constants::status::SpecialWin::{Ipatu, Rinshan};
    use crate::constants::status::WinMethod::Ron;
    use crate::parser::status_input::status_input_utils::build_status_input;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::InvalidWinCombination;

    #[test]
    fn ipatu_without_riichi_invalid() {
        let input = build_status_input(false, Ron, vec![Ipatu]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Ipatu.to_string(), "No riichi".to_string())));
    }

    #[test]
    fn ipatu_rinshan_invalid() {
        let mut input = build_status_input(true, Ron, vec![Ipatu, Rinshan]);
        input.riichi = Riichi(vec![]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Ipatu.to_string(), Rinshan.to_string())));
    }
}

#[cfg(test)]
mod kan_related {
    use crate::constants::status::SpecialWin::{Chankan, Rinshan};
    use crate::constants::status::WinMethod::{Ron, Tumo};
    use crate::parser::status_input::status_input_utils::build_status_input;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::InvalidWinCombination;

    #[test]
    fn chakan_rinshan_invalid() {
        let input = build_status_input(false, Ron, vec![Chankan, Rinshan]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Chankan.to_string(), Rinshan.to_string())));
    }

    #[test]
    fn chakan_ron_valid() {
        let input = build_status_input(false, Ron, vec![Chankan]);
        assert_eq!(input.validate(), Ok(()));
    }

    #[test]
    fn chakan_tumo_invalid() {
        let input = build_status_input(false, Tumo, vec![Chankan]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Chankan.to_string(), Tumo.to_string())));
    }

    #[test]
    fn rinshan_ron_invalid() {
        let input = build_status_input(false, Ron, vec![Rinshan]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Rinshan.to_string(), Ron.to_string())));
    }

    #[test]
    fn rinshan_tumo_valid() {
        use super::*;
        use crate::parser::status_input::status_input_utils::build_status_input;

        let input = build_status_input(false, Tumo, vec![Rinshan]);
        assert_eq!(input.validate(), Ok(()));
    }
}

#[cfg(test)]
mod kawa_related {
    use crate::constants::status::SpecialWin::{Haitei, Hotei};
    use crate::constants::status::WinMethod::{Ron, Tumo};
    use crate::parser::status_input::status_input_utils::build_status_input;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::InvalidWinCombination;

    #[test]
    fn haitei_hotei_invalid() {
        let input = build_status_input(false, Ron, vec![Haitei, Hotei]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Haitei.to_string(), Hotei.to_string())));
    }

    #[test]
    fn haitei_ron_invalid() {
        let input = build_status_input(false, Ron, vec![Haitei]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Haitei.to_string(), Ron.to_string())));
    }

    #[test]
    fn haitei_tumo_valid() {
        let input = build_status_input(false, Tumo, vec![Haitei]);
        assert_eq!(input.validate(), Ok(()));
    }

    #[test]
    fn hotei_ron_valid() {
        let input = build_status_input(false, Ron, vec![Hotei]);
        assert_eq!(input.validate(), Ok(()));
    }

    #[test]
    fn hotei_tumo_invalid() {
        let input = build_status_input(false, Tumo, vec![Hotei]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(Hotei.to_string(), Tumo.to_string())));
    }
}

#[cfg(test)]
mod tenchiho_related {
    use crate::constants::status::SpecialWin::DaiichiTumo;
    use crate::constants::status::WinMethod::{Ron, Tumo};
    use crate::parser::status_input::status_input_utils::build_status_input;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::InvalidWinCombination;

    #[test]
    fn daiichi_tumo_valid() {
        let input = build_status_input(false, Tumo, vec![DaiichiTumo]);
        assert_eq!(input.validate(), Ok(()));
    }


    #[test]
    fn daiichi_tumo_riichi_invalid() {
        let input = build_status_input(true, Tumo, vec![DaiichiTumo]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(DaiichiTumo.to_string(), "Other win combination".to_string())));
    }
    #[test]
    fn daiichi_tumo_ron_invalid() {
        let input = build_status_input(true, Ron, vec![DaiichiTumo]);
        assert_eq!(input.validate(), Err(InvalidWinCombination(DaiichiTumo.to_string(), "Other win combination".to_string())));
    }
}
