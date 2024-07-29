use crate::constants::hand::Mentsu;
use crate::constants::tiles::pi::Tile;
use crate::parser::input_base::InputBase;

pub struct PiInput {
    pub hand: Vec<Tile>,
    pub naki: Vec<Mentsu>,
    pub hora: Tile,
}

impl InputBase for PiInput {
    fn validate(&self) -> bool {
        if self.naki.len() * 3 + self.hand.len() != 13 {
            return false;
        }

        true
    }
}
