use crate::constants::tiles::Tile;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Status {
    pub riichi: RiichiStatus,
    pub win_method: WinMethod,
    pub special_win: HashSet<SpecialWin>,
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
pub enum WinMethod {
    /// ロン
    Ron,
    /// ツモ
    Tumo,
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum SpecialWin {
    /// 一発
    /// - cannot be combined with `Rinshan`
    /// - must call `Riichi` or `DoubleRiichi` (cannot be Dama)
    Ipatu,
    /// 搶槓
    /// - cannot be combined with `Rinshan`
    /// - must be `Ron`
    Chankan,
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
