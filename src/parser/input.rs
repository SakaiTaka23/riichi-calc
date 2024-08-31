use crate::constants::field::Field;
use crate::constants::status::Status;
use crate::parser::input_base::InputBase;
use crate::parser::pi_input::PiInput;

pub struct Input {
    pub pi_input: PiInput,
    pub field_input: Field,
    pub status_input: Status,
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
