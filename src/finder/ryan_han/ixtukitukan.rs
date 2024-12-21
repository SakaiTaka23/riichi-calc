use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct Ixtukitukan;

impl YakuBase for Ixtukitukan {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, _, _) = split_colors(&hand.hand);

        if Self::check_shuntu(manzu) | Self::check_shuntu(pinzu) | Self::check_shuntu(sozu) {
            return check_kuisagari(&hand.hand, "一気通貫".to_string(), 2);
        }

        None
    }
}

impl Ixtukitukan {
    fn check_shuntu(hand: Vec<Mentsu>) -> bool {
        let numbers = hand
            .into_iter()
            .filter(|m| {
                if let Mentsu::Shuntsu(_, _) = m {
                    return true;
                }
                false
            })
            .map(|m| m.tile().number)
            .collect::<Vec<u8>>();

        numbers.contains(&1) && numbers.contains(&4) && numbers.contains(&7)
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::ixtukitukan::Ixtukitukan;
    use crate::finder::test_utils::{
        from_hand, random_field, random_status, random_suhai_tile_type, random_tile,
    };

    #[test]
    fn valid_ixtukitukan() {
        let color = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 1,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 4,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 7,
                },
                false,
            ),
            Mentsu::Koutsu(random_tile(), false),
            Mentsu::Janto(random_tile()),
        ];
        assert_eq!(
            Ixtukitukan::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("一気通貫".to_string(), 2)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn kuisagari_ixtukitukan() {
        let color = random_suhai_tile_type();
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 1,
                },
                true,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 4,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: color,
                    number: 7,
                },
                false,
            ),
            Mentsu::Koutsu(random_tile(), false),
            Mentsu::Janto(random_tile()),
        ];
        assert_eq!(
            Ixtukitukan::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("一気通貫".to_string(), 1)),
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
    use crate::finder::ryan_han::ixtukitukan::Ixtukitukan;
    use crate::finder::test_utils::{from_hand, random_field, random_status, random_tile};

    #[test]
    fn different_color() {
        let hand = [
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 1,
                },
                true,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 4,
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
            Mentsu::Koutsu(random_tile(), false),
            Mentsu::Janto(random_tile()),
        ];
        assert_eq!(
            Ixtukitukan::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
