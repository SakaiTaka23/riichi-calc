use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{SpecialWin, Status};
use crate::finder::finder_base::YakuBase;

pub struct Ipatu;

impl YakuBase for Ipatu {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if status.special_win.contains(&SpecialWin::Ipatu) {
            return Some(("一発".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::SpecialWin;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::ipatu::Ipatu;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn valid_ipatu() {
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
        status.special_win.insert(SpecialWin::Ipatu);
        assert_eq!(Ipatu::validate(&field, &winning_hand, &status), Some(("一発".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::ipatu::Ipatu;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};

    #[test]
    fn invalid_ipatu() {
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
        assert_eq!(Ipatu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
