use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Tuiso;

impl YakuBase for Tuiso {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _)
                | Mentsu::Shuntsu(tile, _)
                | Mentsu::Kantsu(tile, _)
                | Mentsu::Janto(tile) => {
                    if tile.tile_type != TileType::Dragon && tile.tile_type != TileType::Wind {
                        return None;
                    }
                }
            }
        }

        Some(("字一色".to_string(), 1))
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_status};
    use crate::finder::yakuman::tuiso::Tuiso;
    use rand::random;

    #[test]
    fn valid_tuiso() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3,
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
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 1,
            }),
        ];

        assert_eq!(
            Tuiso::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("字一色".to_string(), 1)),
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
    use crate::finder::test_utils::{from_hand, random_field, random_shuntu, random_status};
    use crate::finder::yakuman::tuiso::Tuiso;
    use rand::random;

    #[test]
    fn nothing() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3,
                },
                random(),
            ),
            random_shuntu(true),
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 1,
            }),
        ];

        assert_eq!(
            Tuiso::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
