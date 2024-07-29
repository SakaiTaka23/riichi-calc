use crate::constants::field::{Bakaze, Zikaze};
use crate::constants::tiles::pi::Tile;
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
