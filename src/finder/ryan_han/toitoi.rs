use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct ToiToi {}

impl YakuBase for ToiToi {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return None
                }
                _ => { continue }
            }
        }

        Some(("三槓子".to_string(), 2))
    }
}
