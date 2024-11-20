use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct SanshokuDojun;

impl YakuBase for SanshokuDojun {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, _, _) = split_colors(&hand.hand);
        let manzu_start_numbers = Self::shuntu_start_number(manzu);
        let pinzu_start_numbers = Self::shuntu_start_number(pinzu);
        let sozu_start_numbers = Self::shuntu_start_number(sozu);

        for mentsu in manzu_start_numbers {
            if pinzu_start_numbers.contains(&mentsu) && sozu_start_numbers.contains(&mentsu) {
                return check_kuisagari(&hand.hand, "三色同順".to_string(), 2);
            }
        }

        None
    }
}

impl SanshokuDojun {
    fn shuntu_start_number(color_hand: Vec<Mentsu>) -> Vec<u8> {
        color_hand.into_iter()
            .filter(|m| {
                if let Mentsu::Shuntsu(_, _) = m {
                    return true;
                }
                false
            })
            .map(|m| m.tile().number)
            .collect::<Vec<u8>>()
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sanshoku_dojun::SanshokuDojun;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_shuntu_number, random_status};
    use rand::random;

    #[test]
    fn valid_sanshoku_dojun() {
        let shuntu_number = random_shuntu_number();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: shuntu_number }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: shuntu_number }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: shuntu_number }, false),
            random_mentsu(false, false),
            random_janto(false),
        ];

        assert_eq!(SanshokuDojun::validate(&random_field(), &from_hand(hand), &random_status()), Some(("三色同順".to_string(), 2)), "{:?}", hand);
    }

    #[test]
    fn naki_sanshoku_dojun() {
        let shuntu_number = random_shuntu_number();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: shuntu_number }, true),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: shuntu_number }, random()),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: shuntu_number }, random()),
            random_mentsu(true, false),
            random_janto(false),
        ];

        assert_eq!(SanshokuDojun::validate(&random_field(), &from_hand(hand), &random_status()), Some(("三色同順".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sanshoku_dojun::SanshokuDojun;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_koutsu, random_shuntu_number, random_status};

    #[test]
    fn two_color_sanshioku() {
        let shuntu_number = random_shuntu_number();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: shuntu_number }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: shuntu_number }, false),
            random_koutsu(true, false),
            random_koutsu(true, false),
            random_janto(false),
        ];

        assert_eq!(SanshokuDojun::validate(&random_field(), &from_hand(hand), &random_status()), None, "{:?}", hand);
    }
}
