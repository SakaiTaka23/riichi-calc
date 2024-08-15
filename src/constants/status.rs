use crate::constants::tiles::Tile;

/// ドラ length 1~4
pub type Dora = Vec<Tile>;

#[derive(PartialEq, Eq)]
pub enum RiichiStatus {
    /// リーチなし or ダマ
    NoRiichi,
    /// リーチ 
    /// - takes uradora info as vector 
    /// - length: 1~4
    Riichi(Vec<Tile>),
    /// ダブルリーチ
    /// - takes uradora info as vector 
    /// - length: 1~4
    DoubleRiichi(Vec<Tile>),
}

#[derive(PartialEq, Eq)]
pub enum WinMethod {
    /// ロン
    Ron,
    /// ツモ
    Tumo,
}

#[derive(PartialEq, Hash, Eq)]
pub enum SpecialWin {
    /// 一発
    /// - cannot be combined with `Rinshan`
    /// - must call `Riichi` or `DoubleRiichi` (cannot be Dama)
    Ipatu,
    /// 搶槓
    /// - cannot be combined with `Rinshan`
    /// - must be `Ron`
    Chakan,
    /// 嶺上開花
    /// - cannot be combined with `Chakan`
    /// - must be `Tumo`
    Rinshan,
    /// 海底自摸
    /// - can only exist without `Hotei`
    /// - must be `Tumo`
    Haitei,
    /// 河底撈魚
    /// - can only exist without `Haitei`
    /// - must be `Ron`
    Hotei,
    /// 第一自摸
    /// - will be Tenho or Chiho depending on the dealer
    /// - input only if no other player has 鳴き
    /// - must be `NoRiichi`
    /// - must be `Tumo`
    /// - cannot be combined with any other `SpecialWin`
    DaiichiTumo,
}
