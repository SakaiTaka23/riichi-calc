use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Pinfu {}

impl YakuBase for Pinfu {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, open) => { if open { return None; } }
                Mentsu::Janto(tile) => { if !Self::is_valid_janto(&tile, field) { return None; } }
                _ => { return None }
            }
        }

        if !Self::is_riyanmen(&hand) { return None; }
        Some(("平和".to_string(), 1))
    }
}

impl Pinfu {
    fn is_valid_janto(tile: &Tile, field: &Field) -> bool {
        match tile.tile_type {
            TileType::Manzu | TileType::Pinzu | TileType::Souzu => { true }
            TileType::Wind => {
                if is_same_wind(tile.number, &field.bakaze) || is_same_wind(tile.number, &field.zikaze) {
                    return false;
                }
                true
            }
            TileType::Dragon => { false }
        }
    }

    fn is_riyanmen(WinningHand { hand, winning_tile, .. }: &WinningHand) -> bool {
        let start_number = hand.into_iter()
            .filter(|m| match m {
                Mentsu::Shuntsu(_, _) => true,
                _ => false
            })
            .filter(|m| m.tile().tile_type == winning_tile.tile_type)
            .into_iter()
            .map(|m| m.tile().number)
            .collect::<Vec<u8>>();

        match winning_tile.number {
            1 => { start_number.contains(&1) }
            2 => { start_number.contains(&2) }
            3 => { start_number.contains(&3) }
            4 => { start_number.contains(&2) | start_number.contains(&4) }
            5 => { start_number.contains(&3) | start_number.contains(&5) }
            6 => { start_number.contains(&4) | start_number.contains(&6) }
            7 => { start_number.contains(&5) }
            8 => { start_number.contains(&6) }
            9 => { start_number.contains(&7) }
            _ => { false }
        }
    }
}
