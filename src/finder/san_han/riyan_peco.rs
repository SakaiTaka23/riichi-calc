use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;
use std::collections::HashSet;

pub struct RiyanPeco;

impl YakuBase for RiyanPeco {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(&hand.hand) {
            return None;
        }

        let mut shuntu = Vec::new();
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    shuntu.push(mentsu.clone());
                }
                Mentsu::Janto(_) => continue,
                _ => return None,
            }
        }

        let unique_shuntu: HashSet<Mentsu> = shuntu.clone().into_iter().collect();
        if unique_shuntu.len() != 2 {
            return None;
        }
        let unique_shuntu = unique_shuntu.into_iter().collect::<Vec<Mentsu>>();
        let count1 = shuntu.iter().filter(|&&x| x == unique_shuntu[0]).count();
        let count2 = shuntu.iter().filter(|&&x| x == unique_shuntu[1]).count();
        if count1 != 2 || count2 != 2 {
            return None;
        }

        Some(("二盃口".to_string(), 3))
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::san_han::riyan_peco::RiyanPeco;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_status, random_suhai_tile_type,
    };

    #[test]
    fn valid_riyanpeco() {
        let shuntu_1 = 2;
        let color_1 = random_suhai_tile_type();
        let shuntu_2 = 3;
        let color_2 = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color_1,
                    number: shuntu_1,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color_1,
                    number: shuntu_1,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color_2,
                    number: shuntu_2,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color_2,
                    number: shuntu_2,
                },
                false,
            ),
            random_janto(false),
        ];

        assert_eq!(
            RiyanPeco::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("二盃口".to_string(), 3)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::finder::finder_base::YakuBase;
    use crate::finder::san_han::riyan_peco::RiyanPeco;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_shuntu, random_shuntu_unique,
        random_status,
    };

    #[test]
    fn iipeco() {
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
        assert_eq!(
            RiyanPeco::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }

    #[test]
    fn sananko_type() {
        let random_shuntsu = random_shuntu(false);
        let hand = [
            random_shuntsu,
            random_shuntsu,
            random_shuntsu,
            random_shuntu_unique(false, vec![random_shuntsu]),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(
            RiyanPeco::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
