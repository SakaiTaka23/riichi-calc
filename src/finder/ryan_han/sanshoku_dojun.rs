use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct SanshokuDojun {}

impl YakuBase for SanshokuDojun {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, _, _) = split_colors(hand);

        for mentsu in manzu {
            match mentsu {
                Mentsu::Shuntsu(tile, _) => {
                    let start_number = tile.number;
                    if pinzu.contains(&Mentsu::Shuntsu(Tile { number: start_number, tile_type: TileType::Pinzu }, false)) &&
                        sozu.contains(&Mentsu::Shuntsu(Tile { number: start_number, tile_type: TileType::Souzu }, false)) {
                        return check_kuisagari(hand, "三色同刻".to_string(), 2);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
