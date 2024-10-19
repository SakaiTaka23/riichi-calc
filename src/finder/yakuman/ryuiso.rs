use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;

pub struct Ryuiso {}

impl YakuBase for Ryuiso {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentu in hand.hand {
            match mentu {
                Mentsu::Shuntsu(tile, _) => {
                    if !Ryuiso::is_allowed_shutsu(tile) {
                        return None;
                    }
                }
                Mentsu::Koutsu(tile, _) |
                Mentsu::Kantsu(tile, _) |
                Mentsu::Janto(tile) => {
                    if !Ryuiso::is_allowed_tile(tile) {
                        return None;
                    }
                }
            }
        }

        Some(("緑一色".to_string(), 1))
    }
}

impl Ryuiso {
    fn is_allowed_shutsu(tile: Tile) -> bool {
        if tile.number == 2 && tile.tile_type == TileType::Souzu {
            true
        } else {
            false
        }
    }

    fn is_allowed_tile(tile: Tile) -> bool {
        let allowed: Vec<Tile> = Vec::from([
            Tile { number: 2, tile_type: TileType::Souzu },
            Tile { number: 3, tile_type: TileType::Souzu },
            Tile { number: 4, tile_type: TileType::Souzu },
            Tile { number: 6, tile_type: TileType::Souzu },
            Tile { number: 8, tile_type: TileType::Souzu },
            Tile { number: 2, tile_type: TileType::Dragon },
        ]);

        if allowed.contains(&tile) {
            true
        } else {
            false
        }
    }
}
