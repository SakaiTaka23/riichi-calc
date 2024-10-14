use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sankantu {}

impl YakuBase for Sankantu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let mut kan_count = 0;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Kantsu(_, _) => {
                    kan_count += 1
                }
                _ => {}
            }
        }

        if kan_count == 3 {
            return Some(("三槓子".to_string(), 2));
        }

        None
    }
}
