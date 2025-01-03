use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::Tile;
use crate::parser::ValidationError::InvalidHand;

mod field_input;
mod pi_input;
mod status_input;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidHand(String),
    InvalidTileNumber(String, u8),
    InvalidWinCombination(String, String),
    OutOfRange(String),
}

trait InputBase {
    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Debug)]
pub struct ParsedHand {
    pub field: Field,
    pub winning_hand: WinningHand,
    pub status: Status,
}

///
/// # Fields
///
/// - `hand` - the hand including the hora tile (winning_tile)
/// - `naki` - any mentsu created by naki
/// - `hora` - the winning tile
///
#[derive(Clone)]
pub struct PiInput {
    pub hand: Vec<Tile>,
    pub naki: Vec<Mentsu>,
    pub hora: Tile,
}

///
/// # Fields
///
/// - [pi_input](PiInput)
/// - [field_input](Field)
/// - [status_input](Status)
///
pub struct Input {
    pub pi_input: PiInput,
    pub field_input: Field,
    pub status_input: Status,
}

impl Input {
    pub fn new(pi_input: PiInput, field_input: Field, status_input: Status) -> Self {
        Self {
            pi_input,
            field_input,
            status_input,
        }
    }

    pub fn parse_hand(self) -> Result<Vec<ParsedHand>, ValidationError> {
        self.validate()?;

        let hand_results = self.pi_input.to_mentsu();
        if hand_results.is_none() {
            return Err(InvalidHand("Not a valid winning hand".to_string()));
        }
        let (hands, red_tiles) = hand_results.unwrap();

        let mut result = Vec::new();

        for hand in hands {
            result.push(ParsedHand {
                field: self.field_input.clone(),
                winning_hand: WinningHand {
                    hand,
                    winning_tile: self.pi_input.hora.clone(),
                    red_tile: red_tiles,
                },
                status: self.status_input.clone(),
            })
        }

        Ok(result)
    }
}

impl InputBase for Input {
    fn validate(&self) -> Result<(), ValidationError> {
        self.pi_input.validate()?;
        self.field_input.validate()?;
        self.status_input.validate()?;

        Ok(())
    }
}
