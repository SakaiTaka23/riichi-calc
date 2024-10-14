#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    /// 萬子
    Manzu,
    /// 筒子
    Pinzu,
    /// 索子
    Souzu,
    /// 風牌
    Wind,
    /// 三元牌
    Dragon,
}

/// represents a single tile
///
/// # Fields
/// - `number`: the number of tile [number]
/// - `tile_type`: types in the enum [tile_type]
///
/// [number]: #structfield.number
/// [tile_type]: #structfield.tile_type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    /// - 1~9 for number tile types
    /// - 1~4 for wind tile type `order`: (東, 南, 西, 北)
    /// - 1~3 for dragon tile type `order`: (白, 發, 中)
    pub number: u8,
    /// types in the enum [TileType]
    pub tile_type: TileType,
}
