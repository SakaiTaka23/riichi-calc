use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{is_menzen, split_colors};

pub struct Churen;

impl YakuBase for Churen {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(&hand.hand) {
            return None;
        }

        if !Self::is_one_color(&hand.hand) {
            return None;
        }

        let mut numbers: [u8; 9] = Default::default();
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) => {
                    if tile.number != 1 && tile.number != 9 {
                        return None;
                    }
                    numbers[tile.number as usize - 1] += 3;
                }
                Mentsu::Shuntsu(tile, _) => {
                    numbers[tile.number as usize - 1] += 1;
                    numbers[tile.number as usize] += 1;
                    numbers[tile.number as usize + 1] += 1;
                }
                Mentsu::Kantsu(_, _) => {
                    return None;
                }
                Mentsu::Janto(tile) => {
                    numbers[tile.number as usize - 1] += 2;
                }
            }
        }

        if !Self::is_churen_hand(&numbers) {
            return None;
        }

        let hora_number = hand.winning_tile.number;
        numbers[hora_number as usize - 1] -= 1;

        if Self::is_churen_hand(&numbers) {
            Some(("純正九蓮宝燈".to_string(), 2))
        } else {
            Some(("九蓮宝燈".to_string(), 1))
        }
    }
}

impl Churen {
    fn is_one_color(hand: &Hand) -> bool {
        let (manzu, pinzu, sozu, wind, dragon) = split_colors(hand);
        let non_empty_count = [!manzu.is_empty(), !pinzu.is_empty(), !sozu.is_empty()]
            .iter()
            .filter(|&&x| x)
            .count();
        if non_empty_count != 1 {
            return false;
        }
        if !(wind.is_empty() && dragon.is_empty()) {
            false;
        }

        true
    }

    fn is_churen_hand(numbers: &[u8; 9]) -> bool {
        for &num in numbers.iter() {
            if num < 1 {
                return false;
            }
        }
        if numbers[0] < 3 || numbers[8] < 3 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::{Hand, Mentsu, WinningHand};
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{random_field, random_status, random_suhai_tile_type};
    use crate::finder::yakuman::churen::Churen;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref HAND: (Hand, TileType) = {
            let tile_type = random_suhai_tile_type();
            let hand = [
                Mentsu::Koutsu(
                    Tile {
                        tile_type,
                        number: 1,
                    },
                    false,
                ),
                Mentsu::Shuntsu(
                    Tile {
                        tile_type,
                        number: 2,
                    },
                    false,
                ),
                Mentsu::Shuntsu(
                    Tile {
                        tile_type,
                        number: 5,
                    },
                    false,
                ),
                Mentsu::Koutsu(
                    Tile {
                        tile_type,
                        number: 9,
                    },
                    false,
                ),
                Mentsu::Janto(Tile {
                    tile_type,
                    number: 8,
                }),
            ];
            (hand, tile_type)
        };
    }

    #[test]
    fn valid_churen() {
        let (hand, tile_type) = HAND.clone();
        let winning = WinningHand {
            hand,
            winning_tile: Tile {
                tile_type,
                number: 3,
            },
            red_tile: 0,
        };

        assert_eq!(
            Churen::validate(&random_field(), &winning, &random_status()),
            Some(("九蓮宝燈".to_string(), 1)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn valid_junsei_churen() {
        let (hand, tile_type) = HAND.clone();
        let winning = WinningHand {
            hand,
            winning_tile: Tile {
                tile_type,
                number: 8,
            },
            red_tile: 0,
        };

        assert_eq!(
            Churen::validate(&random_field(), &winning, &random_status()),
            Some(("純正九蓮宝燈".to_string(), 2)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn hora_with_1() {
        let (hand, tile_type) = HAND.clone();
        let winning = WinningHand {
            hand,
            winning_tile: Tile {
                tile_type,
                number: 1,
            },
            red_tile: 0,
        };

        assert_eq!(
            Churen::validate(&random_field(), &winning, &random_status()),
            Some(("九蓮宝燈".to_string(), 1)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{random_field, random_status};
    use crate::finder::yakuman::churen::Churen;

    #[test]
    fn not_one_color() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 1,
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
            Mentsu::Shuntsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: 5,
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
                number: 8,
            }),
        ];

        assert_eq!(
            Churen::validate(
                &random_field(),
                &WinningHand {
                    hand,
                    winning_tile: Tile {
                        tile_type: TileType::Manzu,
                        number: 3
                    },
                    red_tile: 0
                },
                &random_status()
            ),
            None
        );
    }
}
