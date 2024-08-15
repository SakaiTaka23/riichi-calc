use crate::constants::field::{Bakaze, Zikaze};
use crate::constants::tiles::{Tile, TileType};
use crate::parser::input_base::InputBase;

pub struct FieldInput {
    pub zikaze: Zikaze,
    pub bakaze: Bakaze,
    pub dora: Vec<Tile>,
}

impl InputBase for FieldInput {
    fn validate(&self) -> bool {
        if self.dora.len() > 4 {
            return false;
        }

        true
    }
}

#[test]
fn correct_input() {
    let input = FieldInput {
        zikaze: Zikaze::East,
        bakaze: Bakaze::East,
        dora: vec![Tile { number: 1, tile_type: TileType::Manzu }],
    };
    assert!(input.validate());
}

#[test]
fn too_many_dora() {
    let input = FieldInput {
        zikaze: Zikaze::East,
        bakaze: Bakaze::East,
        dora: vec![
            Tile { number: 1, tile_type: TileType::Manzu },
            Tile { number: 1, tile_type: TileType::Manzu },
            Tile { number: 1, tile_type: TileType::Manzu },
            Tile { number: 1, tile_type: TileType::Manzu },
            Tile { number: 1, tile_type: TileType::Manzu }
        ],
    };
    assert!(!input.validate());
}
