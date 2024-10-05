use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sankantu {}

impl YakuBase for Sankantu {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        let mut kan_count = 0;

        for mentsu in hand {
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
