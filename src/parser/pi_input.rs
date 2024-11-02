mod hand_creator;

use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::{Tile, TileType};
use crate::parser::pi_input::hand_creator::create_hand;
use crate::parser::ValidationError::{InvalidHand, InvalidTileNumber};
use crate::parser::{InputBase, ValidationError};

#[derive(Clone)]
pub struct PiInput {
    pub hand: Vec<Tile>,
    pub naki: Vec<Mentsu>,
    pub hora: Tile,
}

#[derive(Clone, Debug, Default)]
struct PiHandColor {
    dragon: Vec<u8>,
    manzu: Vec<u8>,
    pinzu: Vec<u8>,
    souzu: Vec<u8>,
    wind: Vec<u8>,
}

// TODO convert mentsu to hand to check no tile has more than 4 pieces?
// TODO check the dora and ura dora as well-to-do this
impl InputBase for PiInput {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.naki.len() * 3 + self.hand.len() != 13 {
            return Err(InvalidHand("Using too many tiles".to_string()));
        }

        for tile in &self.hand {
            if tile.number < 1 {
                return Err(InvalidTileNumber("Tile number should be greater than or equal to 1".to_string(), tile.number));
            }
            match tile.tile_type {
                TileType::Wind => {
                    if tile.number > 4 {
                        return Err(InvalidTileNumber("Invalid wind number".to_string(), tile.number));
                    }
                }
                TileType::Dragon => {
                    if tile.number > 3 {
                        return Err(InvalidTileNumber("Invalid dragon number".to_string(), tile.number));
                    }
                }
                _ => {
                    if tile.number > 10 {
                        return Err(InvalidTileNumber("Invalid number tile number".to_string(), tile.number));
                    }
                }
            }
        }

        if self.naki.len() > 4 {
            return Err(InvalidHand("Too many naki".to_string()));
        }

        for furo in &self.naki {
            match furo {
                Mentsu::Koutsu(_, x) | Mentsu::Shuntsu(_, x) => {
                    if !x {
                        return Err(InvalidHand("Unopened koutsu or shuntsu in naki".to_string()));
                    }
                }
                Mentsu::Janto(_) => {
                    return Err(InvalidHand("Janto cannot be in naki".to_string()));
                }
                Mentsu::Kantsu(_, _) => {
                    continue
                }
            }
        }

        Ok(())
    }
}

impl PiInput {
    pub fn to_mentsu(&self) -> Option<(Vec<Hand>, u8)> {
        let (colors, red_count) = self.to_hand_color();
        let head_candidate = self.find_toitu();
        let mut menzen_hand: Vec<Hand> = Vec::new();
        for head in head_candidate.iter() {
            let hand = create_hand(&mut colors.clone(), head, &self.naki);
            if hand.is_some() {
                menzen_hand.push(hand?);
            }
        }

        if menzen_hand.is_empty() {
            None
        } else {
            Some((menzen_hand, red_count))
        }
    }

    fn to_hand_color(&self) -> (PiHandColor, u8) {
        let mut hand = PiHandColor::default();
        let mut red_count = 0;

        for &pi in self.hand.iter() {
            let pi = if pi.number == 10 {
                red_count += 1;
                Tile {
                    number: 5,
                    tile_type: pi.tile_type.clone(),
                }
            } else {
                pi
            };

            match pi.tile_type.clone() {
                TileType::Dragon => hand.dragon.push(pi.number),
                TileType::Manzu => hand.manzu.push(pi.number),
                TileType::Pinzu => hand.pinzu.push(pi.number),
                TileType::Souzu => hand.souzu.push(pi.number),
                TileType::Wind => hand.wind.push(pi.number),
            }
        }

        match self.hora.tile_type {
            TileType::Dragon => hand.dragon.push(self.hora.number),
            TileType::Manzu => hand.manzu.push(self.hora.number),
            TileType::Pinzu => hand.pinzu.push(self.hora.number),
            TileType::Souzu => hand.souzu.push(self.hora.number),
            TileType::Wind => hand.wind.push(self.hora.number),
        }

        (hand, red_count)
    }

    fn find_toitu(&self) -> Vec<Tile> {
        let mut seen = Vec::new();
        let mut duplicates = Vec::new();

        for tile in &self.hand {
            if seen.contains(tile) {
                if !duplicates.contains(tile) {
                    duplicates.push(tile.clone());
                }
            } else {
                seen.push(tile.clone());
            }
        }

        if seen.contains(&self.hora) {
            duplicates.push(self.hora.clone());
        }

        duplicates
    }
}

#[cfg(test)]
mod validation_test {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::parser::pi_input::PiInput;
    use crate::parser::InputBase;
    use crate::parser::ValidationError::{InvalidHand, InvalidTileNumber};

    #[test]
    fn menzen_input() {
        let input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 7, tile_type: TileType::Manzu, },
                Tile { number: 7, tile_type: TileType::Manzu, },
                Tile { number: 7, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(input.validate(), Ok(()));
    }

    #[test]
    fn furo_input() {
        let input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, true)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(input.validate(), Ok(()));
    }

    #[test]
    fn invalid_pi() {
        let too_big_suhai_input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 90, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, true)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(too_big_suhai_input.validate(), Err(InvalidTileNumber("Invalid number tile number".to_string(), 90)));

        let too_big_wind_input = PiInput {
            hand: vec![
                Tile { number: 5, tile_type: TileType::Wind, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, true)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(too_big_wind_input.validate(), Err(InvalidTileNumber("Invalid wind number".to_string(), 5)));

        let too_big_dragon_input = PiInput {
            hand: vec![
                Tile { number: 4, tile_type: TileType::Dragon, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, true)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(too_big_dragon_input.validate(), Err(InvalidTileNumber("Invalid dragon number".to_string(), 4)));

        let too_small_suhai_input = PiInput {
            hand: vec![
                Tile { number: 0, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, true)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(too_small_suhai_input.validate(), Err(InvalidTileNumber("Tile number should be greater than or equal to 1".to_string(), 0)));
    }

    #[test]
    fn menzen_naki() {
        let naki_anko_input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Koutsu(Tile { number: 7, tile_type: TileType::Manzu, }, false)
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(naki_anko_input.validate(), Err(InvalidHand("Unopened koutsu or shuntsu in naki".to_string())));

        let naki_janto_input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 1, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 3, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 5, tile_type: TileType::Manzu, },
                Tile { number: 9, tile_type: TileType::Manzu, },
            ],
            naki: vec![
                Mentsu::Janto(Tile { number: 7, tile_type: TileType::Manzu, })
            ],
            hora: Tile { number: 9, tile_type: TileType::Manzu },
        };

        assert_eq!(naki_janto_input.validate(), Err(InvalidHand("Janto cannot be in naki".to_string())));
    }
}

#[cfg(test)]
mod convertor_test {
    use crate::constants::hand::Mentsu::{Janto, Shuntsu};
    use crate::constants::tiles::{Tile, TileType};
    use crate::parser::pi_input::PiInput;
    use crate::parser::InputBase;

    #[test]
    fn all_shuntsu_pinfu_iipeco() {
        let input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu },
                Tile { number: 2, tile_type: TileType::Manzu },
                Tile { number: 3, tile_type: TileType::Manzu },
                Tile { number: 1, tile_type: TileType::Manzu },
                Tile { number: 2, tile_type: TileType::Manzu },
                Tile { number: 3, tile_type: TileType::Manzu },
                Tile { number: 4, tile_type: TileType::Pinzu },
                Tile { number: 5, tile_type: TileType::Pinzu },
                Tile { number: 6, tile_type: TileType::Pinzu },
                Tile { number: 9, tile_type: TileType::Pinzu },
                Tile { number: 6, tile_type: TileType::Souzu },
                Tile { number: 7, tile_type: TileType::Souzu },
                Tile { number: 8, tile_type: TileType::Souzu },
            ],
            naki: vec![],
            hora: Tile { number: 9, tile_type: TileType::Pinzu },
        };
        let hand = vec![[
            Janto(Tile { number: 9, tile_type: TileType::Pinzu }),
            Shuntsu(Tile { number: 4, tile_type: TileType::Pinzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 6, tile_type: TileType::Souzu }, false),
        ]];
        assert!(input.validate().is_ok());
        assert_eq!(input.to_mentsu(), Some((hand, 0)))
    }

    #[test]
    fn all_shuntsu_pinfu_iipeco_red() {
        let input = PiInput {
            hand: vec![
                Tile { number: 1, tile_type: TileType::Manzu },
                Tile { number: 2, tile_type: TileType::Manzu },
                Tile { number: 3, tile_type: TileType::Manzu },
                Tile { number: 1, tile_type: TileType::Manzu },
                Tile { number: 2, tile_type: TileType::Manzu },
                Tile { number: 3, tile_type: TileType::Manzu },
                Tile { number: 4, tile_type: TileType::Pinzu },
                Tile { number: 10, tile_type: TileType::Pinzu },
                Tile { number: 6, tile_type: TileType::Pinzu },
                Tile { number: 9, tile_type: TileType::Pinzu },
                Tile { number: 6, tile_type: TileType::Souzu },
                Tile { number: 7, tile_type: TileType::Souzu },
                Tile { number: 8, tile_type: TileType::Souzu },
            ],
            naki: vec![],
            hora: Tile { number: 9, tile_type: TileType::Pinzu },
        };
        let hand = vec![[
            Janto(Tile { number: 9, tile_type: TileType::Pinzu }),
            Shuntsu(Tile { number: 4, tile_type: TileType::Pinzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 6, tile_type: TileType::Souzu }, false),
        ]];
        assert!(input.validate().is_ok());
        assert_eq!(input.to_mentsu(), Some((hand, 1)))
    }
}
