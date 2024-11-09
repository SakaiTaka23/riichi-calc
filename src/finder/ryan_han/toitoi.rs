use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct ToiToi;

impl YakuBase for ToiToi {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return None
                }
                _ => { continue }
            }
        }

        Some(("対対和".to_string(), 2))
    }
}

#[cfg(test)]
mod valid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::toitoi::ToiToi;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_koutsu, random_status};

    #[test]
    fn valid_toitoi() {
        let hand = [
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_janto(false),
        ];

        assert_eq!(ToiToi::validate(&random_field(), &from_hand(hand), &random_status()), Some(("対対和".to_string(), 2)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::toitoi::ToiToi;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_koutsu, random_shuntu, random_status};

    #[test]
    fn have_shuntu() {
        let hand = [
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_shuntu(true),
            random_janto(false),
        ];

        assert_eq!(ToiToi::validate(&random_field(), &from_hand(hand), &random_status()), None, "{:?}", hand);
    }
}
