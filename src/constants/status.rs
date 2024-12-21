use crate::constants::tiles::Tile;
use std::collections::HashSet;

/// Struct to contain information about the status of the hand that won
///
/// # Fields
///
/// [riichi](RiichiStatus) - whether the hand is riichi or not
/// [win_method](WinMethod) - the method of winning
/// [special_win](SpecialWin) - any special status in the winning hand in a hashset
///
#[derive(Debug, Clone)]
pub struct Status {
    pub riichi: RiichiStatus,
    pub win_method: WinMethod,
    pub special_win: HashSet<SpecialWin>,
}

///
/// # Types
///
/// - [NoRiichi](RiichiStatus::NoRiichi)
/// - [Riichi](RiichiStatus::Riichi)
/// - [DoubleRiichi](RiichiStatus::DoubleRiichi)
///
#[derive(Debug, Clone, PartialEq, Eq)]
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

///
/// # Types
///
/// - [Ron](WinMethod::Ron)
/// - [Tumo](WinMethod::Tumo)
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinMethod {
    /// ロン
    Ron,
    /// ツモ
    Tumo,
}

///
/// # Types
///
/// - [Ipatu](SpecialWin::Ipatu)
/// - [Chankan](SpecialWin::Chankan)
/// - [Rinshan](SpecialWin::Rinshan)
/// - [Haitei](SpecialWin::Haitei)
/// - [Hotei](SpecialWin::Hotei)
/// - [DaiichiTumo](SpecialWin::DaiichiTumo)
///
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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
