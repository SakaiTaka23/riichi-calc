use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::TileType;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Zikaze {}

impl YakuBase for Zikaze {
    fn validate(field: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand {
            match mentsu {
                Mentsu::Koutsu(x, _)
                | Mentsu::Kantsu(x, _) => {
                    if x.tile_type != TileType::Wind { continue; }
                    if is_same_wind(x.number, &field.zikaze) {
                        return Some(("自風".to_string(), 1));
                    }
                }
                _ => {}
            }
        }
        None
    }
}
