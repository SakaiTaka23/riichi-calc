use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct ToiToi {}

impl YakuBase for ToiToi {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return None
                }
                _ => { continue }
            }
        }

        Some(("対対和".to_string(), 2))
    }
}
