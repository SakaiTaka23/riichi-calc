use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Haku;

impl YakuBase for Haku {
    fn validate(_: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in winning_hand.hand {
            if let Mentsu::Janto(_) = mentsu {
                continue;
            }
            let tile = mentsu.tile();
            if tile.tile_type == TileType::Dragon && tile.number == 1 {
                return Some(("役牌:白".to_string(), 1));
            }
        }
        None
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::haku::Haku;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };
    use rand::random;

    #[test]
    fn valid_haku() {
        let field = random_field();
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 1,
                },
                random(),
            ),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(
            Haku::validate(&field, &winning_hand, &status),
            Some(("役牌:白".to_string(), 1)),
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
    use crate::finder::ii_han::haku::Haku;
    use crate::finder::test_utils::{from_hand, random_field, random_mentsu, random_status};

    #[test]
    fn haku_as_janto() {
        let field = random_field();
        let hand = [
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 1,
            }),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
            random_mentsu(true, true),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(
            Haku::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
