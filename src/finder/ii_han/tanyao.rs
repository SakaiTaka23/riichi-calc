use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Tanyao;

impl YakuBase for Tanyao {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentu in hand.hand {
            match mentu {
                Mentsu::Koutsu(x, _) | Mentsu::Kantsu(x, _) | Mentsu::Janto(x) => {
                    if x.tile_type == TileType::Wind || x.tile_type == TileType::Dragon {
                        return None;
                    }
                    if x.number == 1 || x.number == 9 {
                        return None;
                    }
                }
                Mentsu::Shuntsu(x, _) => {
                    if x.number == 1 || x.number == 7 {
                        return None;
                    }
                }
            }
        }

        Some(("断么九".to_string(), 1))
    }
}

#[cfg(test)]
mod tanyao_test_util {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::test_utils::random_suhai_tile_type;
    use rand::{random, Rng};

    pub fn generate_chuchan_pi_mentsu() -> Mentsu {
        let tile_type = random_suhai_tile_type();
        match random::<u8>() % 3 {
            0 => Mentsu::Shuntsu(
                Tile {
                    number: rand::thread_rng().gen_range(2..=6),
                    tile_type,
                },
                random(),
            ),
            1 => Mentsu::Koutsu(
                Tile {
                    number: rand::thread_rng().gen_range(2..=8),
                    tile_type,
                },
                random(),
            ),
            2 => Mentsu::Kantsu(
                Tile {
                    number: rand::thread_rng().gen_range(2..=8),
                    tile_type,
                },
                random(),
            ),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::Tile;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::tanyao::tanyao_test_util::generate_chuchan_pi_mentsu;
    use crate::finder::ii_han::tanyao::Tanyao;
    use crate::finder::test_utils::{
        from_hand, random_field, random_status, random_suhai_tile_type,
    };
    use rand::Rng;

    #[test]
    fn valid_tanyao() {
        let hand = [
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            Mentsu::Janto(Tile {
                tile_type: random_suhai_tile_type(),
                number: rand::thread_rng().gen_range(2..=8),
            }),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(
            Tanyao::validate(&field, &winning_hand, &status),
            Some(("断么九".to_string(), 1)),
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
    use crate::finder::ii_han::tanyao::tanyao_test_util::generate_chuchan_pi_mentsu;
    use crate::finder::ii_han::tanyao::Tanyao;
    use crate::finder::test_utils::{
        from_hand, random_field, random_status, random_suhai_tile_type,
    };
    use rand::{random, Rng};

    #[test]
    fn has_jihai() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            Mentsu::Janto(Tile {
                tile_type: random_suhai_tile_type(),
                number: rand::thread_rng().gen_range(2..=8),
            }),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(
            Tanyao::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }

    #[test]
    fn has_chuchan() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: random_suhai_tile_type(),
                    number: 1,
                },
                random(),
            ),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            generate_chuchan_pi_mentsu(),
            Mentsu::Janto(Tile {
                tile_type: random_suhai_tile_type(),
                number: rand::thread_rng().gen_range(2..=8),
            }),
        ];
        let winning_hand = from_hand(hand);
        let field = random_field();
        let status = random_status();
        assert_eq!(
            Tanyao::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}
