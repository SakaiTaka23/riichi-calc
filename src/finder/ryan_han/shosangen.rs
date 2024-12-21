use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Shosangen;

impl YakuBase for Shosangen {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut dragon_count = 0;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon {
                        dragon_count += 1;
                    }
                }
                Mentsu::Janto(tile) => {
                    if tile.tile_type != TileType::Dragon {
                        return None;
                    }

                    dragon_count += 1;
                }
                _ => continue,
            }
        }

        if dragon_count == 3 {
            return Some(("小三元".to_string(), 2));
        }

        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::shosangen::Shosangen;
    use crate::finder::test_utils::{from_hand, random_field, random_shuntu, random_status};
    use rand::random;

    #[test]
    fn valid_shosangen() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 2,
                },
                random(),
            ),
            random_shuntu(true),
            random_shuntu(true),
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 3,
            }),
        ];

        assert_eq!(
            Shosangen::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("小三元".to_string(), 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::shosangen::Shosangen;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_shuntu, random_status,
    };
    use rand::random;

    #[test]
    fn daisangen() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 3,
                },
                random(),
            ),
            random_shuntu(true),
            random_janto(true),
        ];

        assert_eq!(
            Shosangen::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
