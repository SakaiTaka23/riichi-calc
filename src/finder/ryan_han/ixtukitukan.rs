use crate::constants::field::Field;
use crate::constants::hand::Mentsu::Shuntsu;
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct Ixtukitukan {}

const MANZU1: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Manzu }, false);
const MANZU4: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Manzu }, false);
const MANZU7: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Manzu }, false);

const PINZU1: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Pinzu }, false);
const PINZU4: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Pinzu }, false);
const PINZU7: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Pinzu }, false);

const SOZU1: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Souzu }, false);
const SOZU4: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Souzu }, false);
const SOZU7: Mentsu = Shuntsu(Tile { number: 0, tile_type: TileType::Souzu }, false);

impl YakuBase for Ixtukitukan {
    fn validate(_: &Field, hand: &Hand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, _, _) = split_colors(hand);

        if (manzu.contains(&MANZU1) && manzu.contains(&MANZU4) && manzu.contains(&MANZU7)) ||
            (pinzu.contains(&PINZU1) && pinzu.contains(&PINZU4) && pinzu.contains(&PINZU7)) ||
            (sozu.contains(&SOZU1) && sozu.contains(&SOZU4) && sozu.contains(&SOZU7))
        {
            return check_kuisagari(hand, "一気通貫".to_string(), 2);
        }

        None
    }
}

