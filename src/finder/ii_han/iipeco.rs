use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_menzen;
use std::collections::HashSet;

pub struct IIPeco {}

impl YakuBase for IIPeco {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(hand) {
            return None;
        }

        let mut shuntu: Vec<Mentsu> = Vec::new();
        for mentsu in hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => shuntu.push(mentsu.clone()),
                _ => {}
            }
        }

        let unique_mentsu: HashSet<Mentsu> = shuntu.clone().into_iter().collect();
        if unique_mentsu.len() == shuntu.len() {
            return None;
        }

        Some(("一盃口".to_string(), 1))
    }
}
