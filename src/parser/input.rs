use crate::constants::field::{Bakaze, Zikaze};
use crate::constants::hand::Mentsu;
use crate::constants::status::{RiichiStatus, SpecialWin, WinMethod};
use crate::constants::tiles::pi::Tile;

pub struct Input {
    pub pi_input: PiInput,
    pub field_input: FieldInput,
    pub status_input: StatusInput,
}

pub struct PiInput {
    pub hand: Vec<Tile>,
    pub naki: Mentsu,
    pub hora: Tile,
    pub method: String,
}

pub struct FieldInput {
    pub zikaze: Zikaze,
    pub bakaze: Bakaze,
    pub dora: Vec<Tile>,
}

pub struct StatusInput {
    pub riichi: RiichiStatus,
    pub win_method: WinMethod,
    pub special_win: Vec<SpecialWin>,
}
