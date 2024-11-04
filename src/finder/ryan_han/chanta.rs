use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::check_kuisagari;

pub struct Chanta;

impl YakuBase for Chanta {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut yakuhai_count = 0;
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) | Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        yakuhai_count += 1;
                        continue;
                    } else if tile.number == 1 || tile.number == 9 {
                        continue
                    } else {
                        return None;
                    }
                }
                Mentsu::Shuntsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        unreachable!()
                    } else if tile.number != 1 && tile.number != 7 {
                        return None;
                    }
                }
            }
        }

        if yakuhai_count == 0 {
            return None;
        }

        check_kuisagari(&hand.hand, "混全帯么九".to_string(), 2)
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::chanta::Chanta;
    use crate::finder::test_utils::{from_hand, random_field, random_status};

    #[test]
    fn valid_chanta() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 1 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 7 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 1 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Souzu, number: 9 }, false),
            Mentsu::Janto(Tile { tile_type: TileType::Manzu, number: 9 }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(Chanta::validate(&random_field(), &winning_hand, &random_status()), Some(("混全帯么九".to_string(), 2)), "{:?}", hand);
    }

    #[test]
    fn kuisagari_chanta() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 1 }, true),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 7 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 1 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Souzu, number: 9 }, false),
            Mentsu::Janto(Tile { tile_type: TileType::Manzu, number: 9 }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(Chanta::validate(&random_field(), &winning_hand, &random_status()), Some(("混全帯么九".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::chanta::Chanta;
    use crate::finder::test_utils::{from_hand, random_field, random_status};
    use rand::random;

    #[test]
    fn is_junchan() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 1 }, random()),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 7 }, random()),
            Mentsu::Koutsu(Tile { tile_type: TileType::Pinzu, number: 1 }, random()),
            Mentsu::Koutsu(Tile { tile_type: TileType::Souzu, number: 9 }, random()),
            Mentsu::Janto(Tile { tile_type: TileType::Manzu, number: 9 }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(Chanta::validate(&random_field(), &winning_hand, &random_status()), None, "{:?}", hand);
    }
}
