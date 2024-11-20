use crate::constants::field::{Field, Wind};
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status, WinMethod};
use crate::finder::finder_base::YakuBase;

pub struct Chiho;

impl YakuBase for Chiho {
    fn validate(field: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::DaiichiTumo) &&
            status.win_method.eq(&WinMethod::Tumo) &&
            field.zikaze.ne(&Wind::East) {
            return Some(("地和".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::field::Wind;
    use crate::constants::status::{SpecialWin, WinMethod};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};
    use crate::finder::yakuman::chiho::Chiho;

    #[test]
    fn valid_chiho() {
        let mut field = random_field();
        field.zikaze = Wind::South;
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.special_win = vec![SpecialWin::DaiichiTumo].into_iter().collect();
        status.win_method = WinMethod::Tumo;
        assert_eq!(Chiho::validate(&field, &winning_hand, &status), Some(("地和".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::status::WinMethod;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};
    use crate::finder::yakuman::chiho::Chiho;

    #[test]
    fn nothing() {
        let field = random_field();
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.special_win = vec![].into_iter().collect();
        status.win_method = WinMethod::Tumo;
        assert_eq!(Chiho::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
