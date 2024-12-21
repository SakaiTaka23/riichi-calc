use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sankantu;

impl YakuBase for Sankantu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut kan_count = 0;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Kantsu(_, _) => kan_count += 1,
                _ => {}
            }
        }

        if kan_count == 3 {
            return Some(("三槓子".to_string(), 2));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sankantu::Sankantu;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_kantsu, random_koutsu, random_status,
    };

    #[test]
    fn valid_sankantu() {
        let hand = [
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_koutsu(true, false),
            random_janto(false),
        ];

        assert_eq!(
            Sankantu::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("三槓子".to_string(), 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sankantu::Sankantu;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_kantsu, random_status,
    };

    #[test]
    fn sukantu() {
        let hand = [
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_janto(false),
        ];

        assert_eq!(
            Sankantu::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
