use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::{Status, WinMethod};
use crate::constants::tiles::Tile;
use crate::finder::finder_base::YakuBase;

pub struct Suanko;

impl YakuBase for Suanko {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)> {
        let mut janto: Option<Tile> = None;
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return None;
                }
                Mentsu::Koutsu(_, open) |
                Mentsu::Kantsu(_, open) => {
                    if open {
                        return None;
                    }
                }
                Mentsu::Janto(tile) => {
                    janto = Some(tile)
                }
            }
        }

        let janto = if janto.is_some() { janto.unwrap() } else { return None; };
        if hand.winning_tile == janto {
            Some(("四暗刻単騎".to_string(), 2))
        } else if status.win_method == WinMethod::Tumo {
            Some(("四暗刻".to_string(), 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::WinningHand;
    use crate::constants::status::WinMethod;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{random_field, random_janto, random_kantsu, random_koutsu, random_status};
    use crate::finder::yakuman::suanko::Suanko;

    #[test]
    fn valid_suanko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_koutsu(false, false),
            random_janto(false)
        ];
        let winning_hand = WinningHand { hand, winning_tile: hand[0].tile(), red_tile: 0 };
        let mut status = random_status();
        status.win_method = WinMethod::Tumo;

        assert_eq!(Suanko::validate(&random_field(), &winning_hand, &status), Some(("四暗刻".to_string(), 1)), "{:?}", hand);
    }

    #[test]
    fn valid_suanko_tanki() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_koutsu(false, false),
            random_janto(false)
        ];
        let winning_hand = WinningHand { hand, winning_tile: hand[4].tile(), red_tile: 0 };
        let status = random_status();

        assert_eq!(Suanko::validate(&random_field(), &winning_hand, &status), Some(("四暗刻単騎".to_string(), 2)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::WinningHand;
    use crate::constants::status::WinMethod;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{random_field, random_janto, random_kantsu, random_koutsu, random_status};
    use crate::finder::yakuman::suanko::Suanko;

    #[test]
    fn ron_suanko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_koutsu(false, false),
            random_janto(false)
        ];
        let winning_hand = WinningHand { hand, winning_tile: hand[0].tile(), red_tile: 0 };
        let mut status = random_status();
        status.win_method = WinMethod::Ron;

        assert_eq!(Suanko::validate(&random_field(), &winning_hand, &status), None, "{:?}", hand);
    }
}
