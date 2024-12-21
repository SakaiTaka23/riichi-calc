use crate::calculator::fu::FuBase;
use crate::constants::field::Field;
use crate::constants::hand::Mentsu as M;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};

pub struct Mentsu {}

impl FuBase for Mentsu {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> u8 {
        let mut fu = 0;
        for mentsu in hand.hand {
            match mentsu {
                M::Koutsu(tile, open) => {
                    let mut kotu_fu = 0;
                    if open {
                        kotu_fu += 2
                    } else {
                        kotu_fu += 4
                    }

                    if Self::is_yaochu(&tile) {
                        kotu_fu *= 2
                    }

                    fu += kotu_fu
                }
                M::Shuntsu(_, _) => continue,
                M::Kantsu(tile, open) => {
                    let mut kotu_fu = 0;
                    if open {
                        kotu_fu += 8
                    } else {
                        kotu_fu += 16
                    }

                    if Self::is_yaochu(&tile) {
                        kotu_fu *= 2
                    }

                    fu += kotu_fu
                }
                M::Janto(tile) => {
                    if Self::is_sangenhai(&tile) || Self::is_yakuhai(&field, &tile) {
                        fu += 2
                    }
                }
            }
        }

        fu
    }
}

impl Mentsu {
    fn is_yaochu(tile: &Tile) -> bool {
        if (tile.tile_type == TileType::Wind || tile.tile_type == TileType::Dragon)
            || (tile.tile_type == TileType::Manzu
                || tile.tile_type == TileType::Pinzu
                || tile.tile_type == TileType::Souzu && (tile.number == 1 || tile.number == 9))
        {
            true
        } else {
            false
        }
    }

    fn is_sangenhai(tile: &Tile) -> bool {
        if tile.tile_type == TileType::Dragon {
            true
        } else {
            false
        }
    }

    fn is_yakuhai(field: &Field, tile: &Tile) -> bool {
        if tile.tile_type != TileType::Wind {
            return false;
        }

        if field.bakaze.clone() as u8 == tile.number || field.zikaze.clone() as u8 == tile.number {
            true
        } else {
            false
        }
    }
}
