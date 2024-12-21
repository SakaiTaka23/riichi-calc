use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::{Status, WinMethod};
use crate::constants::tiles::Tile;
use crate::finder::finder_base::YakuBase;

pub struct Sananko;

impl YakuBase for Sananko {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)> {
        let mut anko_count = 0;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(_, open) | Mentsu::Kantsu(_, open) => {
                    if !open {
                        anko_count += 1
                    }
                }
                _ => continue,
            }
        }

        if anko_count == 4 && status.win_method == WinMethod::Tumo {
            return None;
        }

        if status.win_method == WinMethod::Tumo || Self::is_ron_sananko(hand) {
            return Some(("三暗刻".to_string(), 2));
        }

        None
    }
}

impl Sananko {
    fn is_ron_sananko(
        WinningHand {
            hand, winning_tile, ..
        }: &WinningHand,
    ) -> bool {
        let shuntu = hand
            .clone()
            .into_iter()
            .filter(|m| {
                if let Mentsu::Shuntsu(_, _) = m {
                    return true;
                }
                false
            })
            .collect::<Vec<Mentsu>>();
        if shuntu.len() == 0 {
            return true;
        } else if shuntu.len() > 1 {
            return false;
        }
        let shuntu = shuntu[0];
        let shuntu_mentsu = [
            shuntu.tile(),
            Tile {
                tile_type: shuntu.tile().tile_type,
                number: shuntu.tile().number + 1,
            },
            Tile {
                tile_type: shuntu.tile().tile_type,
                number: shuntu.tile().number + 2,
            },
        ];

        shuntu_mentsu.contains(winning_tile)
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::status::WinMethod;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sananko::Sananko;
    use crate::finder::test_utils::{
        random_field, random_janto, random_kantsu, random_koutsu, random_shuntu, random_status,
    };

    #[test]
    fn existing_sananko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_shuntu(true),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[3].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Ron;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            Some(("三暗刻".to_string(), 2)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn tumori_sananko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_shuntu(true),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[0].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Tumo;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            Some(("三暗刻".to_string(), 2)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn ron_suanko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_koutsu(false, false),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[0].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Ron;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            Some(("三暗刻".to_string(), 2)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn complicated_sananko() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 3,
                },
                false,
            ),
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 2,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 6,
                },
                false,
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: 7,
                },
                false,
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Souzu,
                number: 9,
            }),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[0].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Ron;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            Some(("三暗刻".to_string(), 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::WinningHand;
    use crate::constants::status::WinMethod;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sananko::Sananko;
    use crate::finder::test_utils::{
        random_field, random_janto, random_kantsu, random_koutsu, random_shuntu, random_status,
    };

    #[test]
    fn ron_sananko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_shuntu(true),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[0].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Ron;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }

    #[test]
    fn suanko() {
        let hand = [
            random_koutsu(false, false),
            random_koutsu(false, false),
            random_kantsu(false, false),
            random_koutsu(false, false),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: hand[0].tile(),
            red_tile: 0,
        };
        let mut status = random_status();
        status.win_method = WinMethod::Tumo;

        assert_eq!(
            Sananko::validate(&random_field(), &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
