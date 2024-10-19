use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sukantu {}

impl YakuBase for Sukantu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Kantsu(_, _) => { continue; }
                _ => { return None; }
            }
        }

        Some(("四槓子".to_string(), 1))
    }
}
