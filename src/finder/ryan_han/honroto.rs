use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Honroto;

impl YakuBase for Honroto {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut yakuhai_count = 0;
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _)
                | Mentsu::Shuntsu(tile, _)
                | Mentsu::Kantsu(tile, _)
                | Mentsu::Janto(tile) => {
                    if (tile.tile_type == TileType::Manzu
                        || tile.tile_type == TileType::Pinzu
                        || tile.tile_type == TileType::Souzu)
                        && (tile.number != 1 && tile.number != 9)
                    {
                        return None;
                    } else if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind
                    {
                        yakuhai_count += 1;
                    }
                }
            }
        }

        if yakuhai_count == 0 {
            return None;
        }

        Some(("混老頭".to_string(), 2))
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::honroto::Honroto;
    use crate::finder::test_utils::{
        from_hand, random_field, random_status, random_suhai_tile_type,
    };
    use rand::random;

    #[test]
    fn valid_honroto() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 9,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Kantsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 1,
                },
                random(),
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 1,
            }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(
            Honroto::validate(&random_field(), &winning_hand, &random_status()),
            Some(("混老頭".to_string(), 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::honroto::Honroto;
    use crate::finder::test_utils::{
        from_hand, random_field, random_status, random_suhai_tile_type,
    };
    use rand::random;

    #[test]
    fn is_chinroto() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 9,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 1,
                },
                random(),
            ),
            Mentsu::Kantsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 9,
                },
                random(),
            ),
            Mentsu::Janto(Tile {
                tile_type: random_suhai_tile_type(),
                number: 1,
            }),
        ];
        let winning_hand = from_hand(hand);
        assert_eq!(
            Honroto::validate(&random_field(), &winning_hand, &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
