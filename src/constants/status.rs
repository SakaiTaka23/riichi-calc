use crate::constants::tiles::pi::Tile;

pub enum RiichiStatus {
    NoRiichi,
    Riichi(Vec<Tile>),
    DoubleRiichi(Vec<Tile>),
}

pub enum WinMethod {
    Ron,
    Tumo,
}

#[derive(PartialEq)]
pub enum SpecialWin {
    Ipatu,
    Chakan,
    Rinshan,
    Haitei,
    Hotei,
    DaiichiTumo,
}
