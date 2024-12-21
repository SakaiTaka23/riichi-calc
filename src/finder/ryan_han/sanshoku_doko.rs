use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct SanshokuDoko;

impl YakuBase for SanshokuDoko {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut suhai = Vec::new();
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Dragon || tile.tile_type == TileType::Wind {
                        continue;
                    }

                    suhai.push(tile.number);
                }
                _ => continue,
            }
        }

        for number in suhai.clone() {
            let count = count_u8_in_vec(number, &suhai);
            if count == 3 {
                return Some(("三色同刻".to_string(), 2));
            }
        }

        None
    }
}

fn count_u8_in_vec(target: u8, vec: &Vec<u8>) -> usize {
    vec.iter().filter(|&&x| x == target).count()
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::sanshoku_doko::SanshokuDoko;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_shuntu, random_status,
    };
    use rand::{random, Rng};

    #[test]
    fn valid_sanshoku_doko() {
        let kotu_number = rand::thread_rng().gen_range(1..=9);
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: kotu_number,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Pinzu,
                    number: kotu_number,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Souzu,
                    number: kotu_number,
                },
                random(),
            ),
            random_shuntu(true),
            random_janto(false),
        ];

        assert_eq!(
            SanshokuDoko::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(("三色同刻".to_string(), 2)),
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
    use crate::finder::ryan_han::sanshoku_doko::SanshokuDoko;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_shuntu, random_status,
    };
    use rand::{random, Rng};

    #[test]
    fn two_color_sanshoku() {
        let kotu_number = rand::thread_rng().gen_range(1..=9);
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Manzu,
                    number: kotu_number,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Pinzu,
                    number: kotu_number,
                },
                random(),
            ),
            random_shuntu(true),
            random_shuntu(true),
            random_janto(false),
        ];

        assert_eq!(
            SanshokuDoko::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}
