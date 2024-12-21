use crate::constants::field::Field;
use crate::constants::hand::Mentsu::Janto;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Bakaze;

impl YakuBase for Bakaze {
    fn validate(field: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in winning_hand.hand {
            if let Janto(_) = mentsu {
                continue;
            }
            let tile = mentsu.tile();
            if tile.tile_type != TileType::Wind {
                continue;
            }
            if is_same_wind(tile.number, &field.bakaze) {
                return Some(("役牌:場風牌".to_string(), 1));
            }
        }
        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::field::Wind;
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::bakaze::Bakaze;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };
    use rand::random;

    #[test]
    fn valid_bakaze() {
        let mut field = random_field();
        field.bakaze = Wind::East;
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_janto(false),
        ];
        assert_eq!(
            Bakaze::validate(&field, &from_hand(hand), &random_status()),
            Some(("役牌:場風牌".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::field::Wind;
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::bakaze::Bakaze;
    use crate::finder::test_utils::{from_hand, random_field, random_mentsu, random_status};

    #[test]
    fn bakaze_as_janto() {
        let mut field = random_field();
        field.bakaze = Wind::East;
        let hand = [
            Mentsu::Janto(Tile {
                tile_type: TileType::Wind,
                number: 1,
            }),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
        ];
        assert_eq!(
            Bakaze::validate(&field, &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
