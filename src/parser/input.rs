use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::parser::input_base::InputBase;
use crate::parser::pi_input::PiInput;

pub struct ParsedHand {
    pub field: Field,
    pub winning_hand: WinningHand,
    pub status: Status,
}

pub struct Input {
    pub pi_input: PiInput,
    pub field_input: Field,
    pub status_input: Status,
}

impl Input {
    pub fn parse_hand(self) -> Option<Vec<ParsedHand>> {
        if !self.validate() { return None; }

        let hands = self.pi_input.to_mentsu();
        if hands.is_none() { return None; }
        let mut result = Vec::new();

        for hand in hands.unwrap() {
            result.push(
                ParsedHand {
                    field: self.field_input.clone(),
                    winning_hand: WinningHand {
                        hand,
                        winning_tile: self.pi_input.hora.clone(),
                    },
                    status: self.status_input.clone(),
                }
            )
        }

        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }
}

impl InputBase for Input {
    fn validate(&self) -> bool {
        if !(self.pi_input.validate() &&
            self.field_input.validate() &&
            self.status_input.validate()) {
            return false;
        }

        true
    }
}
