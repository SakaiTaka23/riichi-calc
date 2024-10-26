use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::Tile;

pub fn find_dora(dora: &Vec<Tile>, hand: &Hand) -> u8 {
    let mut dora_count = 0;
    for mentsu in hand {
        match mentsu {
            Mentsu::Koutsu(tile, _) => {
                if dora.contains(&tile) {
                    dora_count += 3;
                }
            }
            Mentsu::Shuntsu(tile, _) => {
                let tile1 = Tile { number: tile.number, tile_type: tile.tile_type };
                let tile2 = Tile { number: tile.number + 1, tile_type: tile.tile_type };
                let tile3 = Tile { number: tile.number + 2, tile_type: tile.tile_type };
                dora_count += if dora.contains(&tile1) { 1 } else { 0 };
                dora_count += if dora.contains(&tile2) { 1 } else { 0 };
                dora_count += if dora.contains(&tile3) { 1 } else { 0 };
            }
            Mentsu::Kantsu(tile, _) => {
                if dora.contains(&tile) {
                    dora_count += 4;
                }
            }
            Mentsu::Janto(tile) => {
                if dora.contains(&tile) {
                    dora_count += 2;
                }
            }
        }
    }

    dora_count
}
