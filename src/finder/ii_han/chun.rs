use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Chun;

impl YakuBase for Chun {
    fn validate(_: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in winning_hand.hand {
            if let Mentsu::Janto(_) = mentsu { continue; }
            let tile = mentsu.tile();
            if tile.tile_type == TileType::Dragon && tile.number == 3 {
                return Some(("役牌:中".to_string(), 1));
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
    use crate::finder::ii_han::chun::Chun;
    use crate::finder::test_utils::{from_hand, random_field, random_janto, random_mentsu, random_status};
    use rand::random;

    #[test]
    fn valid_chun() {
        let field = random_field();
        let hand = [
            Mentsu::Koutsu(
                Tile { tile_type: TileType::Dragon, number: 3 },
                random(),
            ),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(Chun::validate(&field, &winning_hand, &status), Some(("役牌:中".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::chun::Chun;
    use crate::finder::test_utils::{from_hand, random_field, random_mentsu, random_status};

    #[test]
    fn chun_as_janto() {
        let field = random_field();
        let hand = [
            Mentsu::Janto(
                Tile { tile_type: TileType::Dragon, number: 3 },
            ),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
        ];
        let winning_hand = from_hand(hand);
        let status = random_status();
        assert_eq!(Chun::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
