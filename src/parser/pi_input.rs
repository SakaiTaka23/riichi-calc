mod hand_creator;

use crate::constants::hand::Mentsu::Koutsu;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::manzu::MANZU_1;
use crate::constants::tiles::pi::{Tile, TileType};
use crate::parser::input_base::InputBase;

pub struct PiInput {
    pub hand: Vec<Tile>,
    pub naki: Vec<Mentsu>,
    pub hora: Tile,
}

#[derive(Debug, Default)]
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
    pub fn to_mentsu(&self) -> Vec<Hand> {
        // TODO 色ごとのベクターに分ける
        let _colors = self.to_hand_color();
        // TODO handからトイツの抜き出し
        let head_candidate = self.find_toitu();
        // TODO その色でできるか検証
        for _head in head_candidate.iter() {}

        // TODO 他の色についても実行
        // TODO 鳴きと組み合わせて返却

        let m = Koutsu(MANZU_1, false);
        vec![[m.clone(), m.clone(), m.clone(), m.clone(), m.clone()]]
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
