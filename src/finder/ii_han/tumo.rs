use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{Status, WinMethod};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;

pub struct Tumo;

impl YakuBase for Tumo {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)> {
        if !is_menzen(&hand.hand) || status.win_method == WinMethod::Ron {
            return None;
        }

        Some(("門前清自摸和".to_string(), 1))
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::WinMethod;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::tumo::Tumo;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };

    #[test]
    fn valid_tumo() {
        let field = random_field();
        let hand = [
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.win_method = WinMethod::Tumo;
        assert_eq!(
            Tumo::validate(&field, &winning_hand, &status),
            Some(("門前清自摸和".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::status::WinMethod;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::tumo::Tumo;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_shuntu_number, random_status,
        random_tile_type,
    };

    #[test]
    fn not_menzen() {
        let random_shuntsu = Mentsu::Shuntsu(
            Tile {
                tile_type: random_tile_type(),
                number: random_shuntu_number(),
            },
            true,
        );
        let hand = [
            random_shuntsu,
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(
            Tumo::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }

    #[test]
    fn is_ron() {
        let field = random_field();
        let hand = [
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.win_method = WinMethod::Ron;
        assert_eq!(
            Tumo::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
