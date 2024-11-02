use crate::constants::field::Field;
use crate::parser::ValidationError::OutOfRange;
use crate::parser::{InputBase, ValidationError};

impl InputBase for Field {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.dora.len() > 4 {
            return Err(OutOfRange("The number of dora should be less than or equal to 4".to_string()));
        }

        Ok(())
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
    assert_eq!(input.validate(), Ok(()));
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
    assert_eq!(input.validate(), Err(OutOfRange("The number of dora should be less than or equal to 4".to_string())));
}
