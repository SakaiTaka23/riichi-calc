use crate::parser::field_input::FieldInput;
use crate::parser::input_base::InputBase;
use crate::parser::pi_input::PiInput;
use crate::parser::status_input::StatusInput;

pub struct Input {
    pub pi_input: PiInput,
    pub field_input: FieldInput,
    pub status_input: StatusInput,
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
