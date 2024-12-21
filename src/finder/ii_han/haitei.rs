use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Haitei;

impl YakuBase for Haitei {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Haitei) {
            return Some(("海底自摸".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::SpecialWin;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::haitei::Haitei;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };

    #[test]
    fn valid_haitei() {
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
        status.special_win.insert(SpecialWin::Haitei);
        assert_eq!(
            Haitei::validate(&field, &winning_hand, &status),
            Some(("海底自摸".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::haitei::Haitei;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };

    #[test]
    fn invalid_haitei() {
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
        assert_eq!(
            Haitei::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
