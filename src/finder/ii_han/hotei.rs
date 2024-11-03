use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Hotei;

impl YakuBase for Hotei {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Hotei) {
            return Some(("河底撈魚".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::SpecialWin;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::hotei::Hotei;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn valid_hotei() {
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
        status.special_win.insert(SpecialWin::Hotei);
        assert_eq!(Hotei::validate(&field, &winning_hand, &status), Some(("河底撈魚".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::hotei::Hotei;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn invalid_hotei() {
        let field = random_field();
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(Hotei::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
