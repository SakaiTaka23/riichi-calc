use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sukantu;

impl YakuBase for Sukantu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Janto(_) | Mentsu::Kantsu(_, _) => {
                    continue;
                }
                _ => {
                    return None;
                }
            }
        }

        Some(("四槓子".to_string(), 1))
    }
}

#[cfg(test)]
mod valid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_kantsu, random_status,
    };
    use crate::finder::yakuman::sukantu::Sukantu;

    #[test]
    fn valid_sukantu() {
        let hand = [
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_kantsu(true, false),
            random_janto(false),
        ];

        assert_eq!(
            Sukantu::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("四槓子".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_kantsu, random_koutsu, random_status,
    };
    use crate::finder::yakuman::sukantu::Sukantu;

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
            Sukantu::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
