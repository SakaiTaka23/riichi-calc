#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileType {
    Manzu,
    Pinzu,
    Souzu,
    Wind,
    Dragon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub number: u8,
    pub tile_type: TileType,
}
