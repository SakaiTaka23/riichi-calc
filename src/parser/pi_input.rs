mod hand_creator;

use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::{Tile, TileType};
use crate::parser::input_base::InputBase;
use crate::parser::pi_input::hand_creator::create_hand;

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
    fn validate(&self) -> bool {
        if self.naki.len() * 3 + self.hand.len() != 13 {
            return false;
        }

        for tile in &self.hand {
            if tile.number < 1 {
                return false;
            }
            match tile.tile_type {
                TileType::Wind => {
                    if tile.number > 4 {
                        return false;
                    }
                }
                TileType::Dragon => {
                    if tile.number > 3 {
                        return false;
                    }
                }
                _ => {
                    if tile.number > 9 {
                        return false;
                    }
                }
            }
        }

        if self.naki.len() > 4 {
            return false;
        }

        for furo in &self.naki {
            match furo {
                Mentsu::Koutsu(_, x) | Mentsu::Shuntsu(_, x) | Mentsu::Kantsu(_, x) => {
                    if !x {
                        return false;
                    }
                }
                Mentsu::Janto(_) => {
                    return false;
                }
            }
        }

        true
    }
}

impl PiInput {
    pub fn to_mentsu(&self) -> Option<Vec<Hand>> {
        let colors = self.to_hand_color();
        let head_candidate = self.find_toitu();
        for head in head_candidate.iter() {
            let mut menzen_hand: Vec<Hand> = Vec::new();
            let hand = create_hand(&colors.clone(), head);
            if hand.is_some() {
                menzen_hand.push(hand.unwrap());
                return Some(menzen_hand);
            }
        }

        None
    }

    fn to_hand_color(&self) -> PiHandColor {
        let mut hand = PiHandColor::default();

        for pi in self.hand.iter() {
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

        hand
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

    assert!(input.validate());
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

    assert!(input.validate());
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

    assert!(!too_big_suhai_input.validate());

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

    assert!(!too_big_wind_input.validate());

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

    assert!(!too_big_dragon_input.validate());

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

    assert!(!too_small_suhai_input.validate());
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

    assert!(!naki_anko_input.validate());

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

    assert!(!naki_janto_input.validate());
}