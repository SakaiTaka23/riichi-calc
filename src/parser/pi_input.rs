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

impl InputBase for PiInput {
    fn validate(&self) -> bool {
        if self.naki.len() * 3 + self.hand.len() != 13 {
            return false;
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
