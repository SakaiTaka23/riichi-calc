use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;
use std::collections::HashSet;

pub struct IIPeco;

impl YakuBase for IIPeco {
    fn validate(_: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(&winning_hand.hand) {
            return None;
        }

        let mut shuntu: Vec<Mentsu> = Vec::new();
        for mentsu in winning_hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => shuntu.push(mentsu.clone()),
                _ => {}
            }
        }

        let unique_shuntu: HashSet<Mentsu> = shuntu.clone().into_iter().collect();
        if unique_shuntu.len() != 1 {
            return None;
        }

        Some(("一盃口".to_string(), 1))
    }
}

#[cfg(test)]
mod valid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::iipeco::IIPeco;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_shuntu, random_status};

    #[test]
    fn valid_iipeco() {
        let random_shuntsu = random_shuntu(false);
        let hand = [
            random_shuntsu,
            random_shuntsu,
            random_mentsu(false, false),
            random_mentsu(false, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(IIPeco::validate(&field, &winning_hand, &status), Some(("一盃口".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::iipeco::IIPeco;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_koutsu, random_mentsu, random_shuntu, random_shuntu_number, random_status, random_tile_type};

    #[test]
    fn ryanpeko() {
        let random_shuntsu_1 = random_shuntu(false);
        let random_shuntsu_2 = random_shuntu(false);
        let hand = [
            random_shuntsu_1,
            random_shuntsu_1,
            random_shuntsu_2,
            random_shuntsu_2,
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(IIPeco::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn open_iipeco() {
        let random_shuntsu = Mentsu::Shuntsu(
            Tile {
                tile_type: random_tile_type(),
                number: random_shuntu_number(),
            }, true);
        let hand = [
            random_shuntsu,
            random_shuntsu,
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(IIPeco::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn with_different_color() {
        let shuntu_number = random_shuntu_number();
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: shuntu_number,
                }, true),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: shuntu_number,
                }, false),
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(IIPeco::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
