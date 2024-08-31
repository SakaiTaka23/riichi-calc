use crate::constants::field::Field;
use crate::parser::input_base::InputBase;

impl InputBase for Field {
    fn validate(&self) -> bool {
        if self.dora.len() > 4 {
            return false;
        }

        true
    }
}

#[test]
fn correct_input() {
    use crate::constants::field::{Bakaze, Zikaze};
    use crate::constants::tiles::{Tile, TileType};

    let input = Field {
        zikaze: Zikaze::East,
        bakaze: Bakaze::East,
        dora: vec![Tile { number: 1, tile_type: TileType::Manzu }],
    };
    assert!(input.validate());
}

#[test]
fn too_many_dora() {
    use crate::constants::field::{Bakaze, Zikaze};
    use crate::constants::tiles::{Tile, TileType};

    let input = Field {
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
