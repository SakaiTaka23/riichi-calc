use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::dora::dora_finder::find_dora;
use crate::finder::finder_base::YakuBase;

pub struct Omote;

impl YakuBase for Omote {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let dora_count = find_dora(&field.dora, &hand.hand);

        if dora_count > 0 {
            Some(("ドラ".to_string(), dora_count))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod count {
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::dora::omote::Omote;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_status};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref WINNING_HAND: WinningHand = from_hand([
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 1
                },
                false
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 4
                },
                false
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3
                },
                false
            ),
            Mentsu::Kantsu(
                Tile {
                    tile_type: TileType::Pinzu,
                    number: 2
                },
                false
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Manzu,
                number: 9
            }),
        ]);
    }

    #[test]
    fn one_in_shuntu() {
        let mut field = random_field();
        field.dora = vec![Tile {
            tile_type: TileType::Manzu,
            number: 2,
        }];

        assert_eq!(
            Omote::validate(&field, &WINNING_HAND, &random_status()),
            Some(("ドラ".to_string(), 1))
        );
    }

    #[test]
    fn dora_anko() {
        let mut field = random_field();
        field.dora = vec![Tile {
            tile_type: TileType::Wind,
            number: 3,
        }];

        assert_eq!(
            Omote::validate(&field, &WINNING_HAND, &random_status()),
            Some(("ドラ".to_string(), 3))
        );
    }

    #[test]
    fn kantu_moro_nori() {
        let mut field = random_field();
        field.dora = vec![Tile {
            tile_type: TileType::Pinzu,
            number: 2,
        }];

        assert_eq!(
            Omote::validate(&field, &WINNING_HAND, &random_status()),
            Some(("ドラ".to_string(), 4))
        );
    }

    #[test]
    fn should_not_print_in_zero() {
        let mut field = random_field();
        field.dora = vec![Tile {
            tile_type: TileType::Wind,
            number: 4,
        }];

        assert_eq!(
            Omote::validate(&field, &WINNING_HAND, &random_status()),
            None
        );
    }
}
