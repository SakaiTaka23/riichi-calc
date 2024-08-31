use crate::constants::field::Field;
use crate::constants::hand::Hand;
use crate::constants::status::{RiichiStatus, Status};
use crate::finder::finder_base::YakuBase;

pub struct Riichi {}

impl YakuBase for Riichi {
    fn validate(_: &Field, _: &Hand, status: &Status) -> Option<(String, u8)> {
        match status.riichi {
            RiichiStatus::Riichi(_) => Some(("立直".to_string(), 1)),
            _ => None,
        }
    }
}
