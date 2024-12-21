use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{RiichiStatus, Status};
use crate::finder::finder_base::YakuBase;

pub struct Riichi;

impl YakuBase for Riichi {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<(String, u8)> {
        match status.riichi {
            RiichiStatus::Riichi(_) => Some(("立直".to_string(), 1)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::RiichiStatus;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::riichi::Riichi;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status, random_tile,
    };

    #[test]
    fn valid_riichi() {
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
        status.riichi = RiichiStatus::Riichi(vec![random_tile()]);
        assert_eq!(
            Riichi::validate(&field, &winning_hand, &status),
            Some(("立直".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::status::RiichiStatus;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::riichi::Riichi;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status, random_tile,
    };
    use rand::random;

    #[test]
    fn invalid_riichi() {
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
        status.riichi = match random::<u8>() % 2 {
            0 => RiichiStatus::NoRiichi,
            1 => RiichiStatus::DoubleRiichi(vec![random_tile(), random_tile()]),
            _ => unreachable!(),
        };
        assert_eq!(
            Riichi::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
