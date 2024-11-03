use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Rinshan;

impl YakuBase for Rinshan {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Rinshan) {
            return Some(("嶺上開花".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::SpecialWin;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::rinshan::Rinshan;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn valid_rinshan() {
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
        status.special_win.insert(SpecialWin::Rinshan);
        assert_eq!(Rinshan::validate(&field, &winning_hand, &status), Some(("嶺上開花".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::rinshan::Rinshan;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn invalid_rinshan() {
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
        assert_eq!(Rinshan::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
