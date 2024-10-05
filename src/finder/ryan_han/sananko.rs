use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;

pub struct Sananko {}

impl YakuBase for Sananko {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        let mut anko_count = 0;

        for mentsu in hand {
            match mentsu {
                Mentsu::Koutsu(_, open)
                | Mentsu::Kantsu(_, open) => {
                    if !open {
                        anko_count += 1
                    }
                }
                _ => { continue }
            }
        }

        if anko_count == 3 {
            return Some(("三暗刻".to_string(), 2));
        }

        None
    }
}
