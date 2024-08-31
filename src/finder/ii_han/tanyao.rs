use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;

pub struct Tanyao {}

impl YakuBase for Tanyao {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentu in hand {
            match mentu {
                Mentsu::Koutsu(x, _)
                | Mentsu::Kantsu(x, _)
                | Mentsu::Janto(x) => {
                    if x.tile_type == TileType::Wind || x.tile_type == TileType::Dragon { return None; }
                    if x.number == 1 || x.number == 9 { return None; }
                }
                Mentsu::Shuntsu(x, _) => { if x.number == 1 || x.number == 7 { return None; } }
            }
        }

        Some(("断么九".to_string(), 1))
    }
}
