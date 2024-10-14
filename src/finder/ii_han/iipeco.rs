use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;
use std::collections::HashSet;

pub struct IIPeco {}

impl YakuBase for IIPeco {
    fn validate(_: &Field, winning_hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(&winning_hand.hand) {
            return None;
        }

        let mut shuntu: Vec<Mentsu> = Vec::new();
        for mentsu in winning_hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => shuntu.push(mentsu.clone()),
                _ => {}
            }
        }

        let unique_shuntu: HashSet<Mentsu> = shuntu.clone().into_iter().collect();
        if unique_shuntu.len() == shuntu.len() - 1 {
            return None;
        }

        Some(("一盃口".to_string(), 1))
    }
}
