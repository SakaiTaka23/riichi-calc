use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::check_kuisagari;

pub struct Junchan;

impl YakuBase for Junchan {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) | Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        return None;
                    } else if tile.number == 1 || tile.number == 9 {
                        continue;
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

        check_kuisagari(&hand.hand, "純全帯么九".to_string(), 3)
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::san_han::junchan::Junchan;
    use crate::finder::test_utils::{from_hand, random_field, random_status};

    #[test]
    fn valid_junchan() {
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 1,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 7,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 1,
                },
                false,
            ),
            Mentsu::Kantsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 9,
                },
                false,
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Manzu,
                number: 9,
            }),
        ];

        assert_eq!(
            Junchan::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("純全帯么九".to_string(), 3)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn valid_junchan_naki() {
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 1,
                },
                true,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 7,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 1,
                },
                false,
            ),
            Mentsu::Kantsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 9,
                },
                false,
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Manzu,
                number: 9,
            }),
        ];

        assert_eq!(
            Junchan::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("純全帯么九".to_string(), 2)),
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
    use crate::finder::san_han::junchan::Junchan;
    use crate::finder::test_utils::{from_hand, random_field, random_status};

    #[test]
    fn chanta() {
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 1,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 7,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 9,
                },
                false,
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Manzu,
                number: 9,
            }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(
            Junchan::validate(&random_field(), &winning_hand, &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
