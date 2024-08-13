use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::pi::{Tile, TileType};
use crate::parser::pi_input::PiHandColor;
use std::collections::HashMap;

fn create_hand(mut colors: PiHandColor, head: &Tile) -> Option<Hand> {
    let mut _hands: Vec<Hand> = Vec::new();
    // TODO 雀頭を抜き出す
    let pi_count = match head.tile_type {
        TileType::Dragon => {
            remove_two_occurrences(&mut colors.dragon, head.number);
            colors.dragon.len()
        }
        TileType::Manzu => {
            remove_two_occurrences(&mut colors.manzu, head.number);
            colors.manzu.len()
        }
        TileType::Pinzu => {
            remove_two_occurrences(&mut colors.pinzu, head.number);
            colors.pinzu.len()
        }
        TileType::Souzu => {
            remove_two_occurrences(&mut colors.souzu, head.number);
            colors.souzu.len()
        }
        TileType::Wind => {
            remove_two_occurrences(&mut colors.wind, head.number);
            colors.wind.len()
        }
    };
    if pi_count % 3 != 0 {
        return None;
    }

    // TODO 暗刻を抜き出す
    // TODO それでできるか検証 暗刻なしでも検証が必要

    None
}

fn create_mentu_suhai(hand: Vec<u8>) -> Option<Vec<Mentsu>> {
    // TODO 暗刻を抜き出す
    // TODO 全ての組み合わせでできるか検証

    None
}

fn create_mentu_zihai(hand: Vec<u8>) -> Option<Vec<Mentsu>> {
    None
}

fn remove_two_occurrences(vec: &mut Vec<u8>, target: u8) {
    let mut count = 0;
    vec.retain(|&x| {
        if x == target && count < 2 {
            count += 1;
            false
        } else {
            true
        }
    });
}

fn extract_anko(tiles: &mut Vec<u8>) -> Vec<u8> {
    let mut counts = HashMap::new();

    // タイルごとの出現回数をカウントする
    for &tile in tiles.iter() {
        *counts.entry(tile).or_insert(0) += 1;
    }

    // 暗刻（3つ以上のタイル）を抽出する
    let mut anko = Vec::new();
    for (&tile, &count) in counts.iter() {
        if count >= 3 {
            anko.push(tile);
        }
    }

    // 抽出した暗刻を元のベクタから削除する
    for &tile in anko.iter() {
        tiles.retain(|&x| x != tile);  // 暗刻が含まれるタイルを削除
    }

    anko
}
