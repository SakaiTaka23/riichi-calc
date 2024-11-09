use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct Honitu;

impl YakuBase for Honitu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, wind, dragon) = split_colors(&hand.hand);
        let non_empty_count = [!manzu.is_empty(), !pinzu.is_empty(), !sozu.is_empty()]
            .iter().filter(|&&x| x).count();

        if non_empty_count != 1 {
            return None;
        }
        if wind.is_empty() && dragon.is_empty() {
            return None;
        }

        check_kuisagari(&hand.hand, "混一色".to_string(), 3)
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::san_han::honitu::Honitu;
    use crate::finder::test_utils::{from_hand, random_field, random_status, random_suhai_tile_type};

    #[test]
    fn valid_honitu() {
        let color = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: color, number: 1 }, false),
            Mentsu::Koutsu(Tile { tile_type: color, number: 7 }, false),
            Mentsu::Kantsu(Tile { tile_type: color, number: 2 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 1 }, false),
            Mentsu::Janto(Tile { tile_type: color, number: 9 }),
        ];

        assert_eq!(Honitu::validate(&random_field(), &from_hand(hand), &random_status()), Some(("混一色".to_string(), 3)), "{:?}", hand);
    }

    #[test]
    fn valid_honitu_naki() {
        let color = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: color, number: 1 }, false),
            Mentsu::Koutsu(Tile { tile_type: color, number: 7 }, false),
            Mentsu::Kantsu(Tile { tile_type: color, number: 2 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 1 }, true),
            Mentsu::Janto(Tile { tile_type: color, number: 9 }),
        ];

        assert_eq!(Honitu::validate(&random_field(), &from_hand(hand), &random_status()), Some(("混一色".to_string(), 2)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::san_han::honitu::Honitu;
    use crate::finder::test_utils::{from_hand, random_field, random_status, random_suhai_tile_type};

    #[test]
    fn chinitu() {
        let color = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: color, number: 1 }, false),
            Mentsu::Koutsu(Tile { tile_type: color, number: 7 }, false),
            Mentsu::Kantsu(Tile { tile_type: color, number: 2 }, false),
            Mentsu::Koutsu(Tile { tile_type: color, number: 1 }, false),
            Mentsu::Janto(Tile { tile_type: color, number: 9 }),
        ];

        assert_eq!(Honitu::validate(&random_field(), &from_hand(hand), &random_status()), None, "{:?}", hand);
    }
}
