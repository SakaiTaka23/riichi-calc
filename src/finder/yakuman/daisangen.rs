use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Daisangen {}

impl YakuBase for Daisangen {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut dragon_count = 0;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon {
                        dragon_count += 1;
                    }
                }
                _ => continue
            }
        }

        if dragon_count == 3 {
            return Some(("大三元".to_string(), 1));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_shuntu, random_status};
    use crate::finder::yakuman::daisangen::Daisangen;
    use rand::random;

    #[test]
    fn daisangen() {
        let hand = [
            Mentsu::Koutsu(Tile { tile_type: TileType::Dragon, number: 1 }, random()),
            Mentsu::Koutsu(Tile { tile_type: TileType::Dragon, number: 2 }, random()),
            Mentsu::Koutsu(Tile { tile_type: TileType::Dragon, number: 3 }, random()),
            random_shuntu(true),
            random_janto(true),
        ];

        assert_eq!(Daisangen::validate(&random_field(), &from_hand(hand), &random_status()), Some(("大三元".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_shuntu, random_status};
    use crate::finder::yakuman::daisangen::Daisangen;
    use rand::random;

    #[test]
    fn valid_shosangen() {
        let hand = [
            Mentsu::Koutsu(Tile { tile_type: TileType::Dragon, number: 1 }, random()),
            Mentsu::Koutsu(Tile { tile_type: TileType::Dragon, number: 2 }, random()),
            random_shuntu(true),
            random_shuntu(true),
            Mentsu::Janto(Tile { tile_type: TileType::Dragon, number: 3 }),
        ];

        assert_eq!(Daisangen::validate(&random_field(), &from_hand(hand), &random_status()), None, "{:?}", hand);
    }
}
