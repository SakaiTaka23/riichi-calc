use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Chankan;

impl YakuBase for Chankan {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Chankan) {
            return Some(("搶槓".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::SpecialWin;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::chankan::Chankan;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn valid_chankan() {
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
        status.special_win.insert(SpecialWin::Chankan);
        assert_eq!(Chankan::validate(&field, &winning_hand, &status), Some(("搶槓".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::chankan::Chankan;
    use crate::finder::test_utils::{from_hand, random_field, random_mentsu, random_status};

    #[test]
    fn invalid_chankan() {
        let field = random_field();
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(Chankan::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
