use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Zikaze;

impl YakuBase for Zikaze {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            if let Mentsu::Janto(_) = mentsu {
                continue;
            }
            let tile = mentsu.tile();
            if tile.tile_type != TileType::Wind {
                continue;
            }
            if is_same_wind(tile.number, &field.zikaze) {
                return Some(("役牌:自風牌".to_string(), 1));
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
    use crate::finder::ii_han::zikaze::Zikaze;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };
    use rand::random;

    #[test]
    fn valid_zikaze() {
        let mut field = random_field();
        field.zikaze = Wind::East;
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
            Zikaze::validate(&field, &from_hand(hand), &random_status()),
            Some(("役牌:自風牌".to_string(), 1))
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::field::Wind;
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::zikaze::Zikaze;
    use crate::finder::test_utils::{from_hand, random_field, random_mentsu, random_status};

    #[test]
    fn zikaze_as_janto() {
        let mut field = random_field();
        field.zikaze = Wind::East;
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
            Zikaze::validate(&field, &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
