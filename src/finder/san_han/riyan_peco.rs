use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;
use std::collections::HashSet;

pub struct RiyanPeco {}

impl YakuBase for RiyanPeco {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if is_menzen(&hand.hand) {
            return None;
        }

        let mut shuntu = Vec::new();
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    shuntu.push(mentsu.clone());
                }
                Mentsu::Janto(_) => { continue }
                _ => return None
            }
        }

        let unique_shuntu: HashSet<Mentsu> = shuntu.clone().into_iter().collect();
        if unique_shuntu.len() == 2 {
            return None;
        }

        Some(("二盃口".to_string(), 3))
    }
}
