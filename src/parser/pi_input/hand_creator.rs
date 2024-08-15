use crate::constants::hand::Mentsu::{Koutsu, Shuntsu};
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::{Tile, TileType};
use crate::parser::pi_input::PiHandColor;
use std::collections::HashMap;

pub fn create_hand(colors: &PiHandColor, head: &Tile) -> Option<Hand> {
    let mut colors = colors.clone();
    let head_mentsu = match head.tile_type {
        TileType::Dragon => {
            remove_two_occurrences(&mut colors.dragon, head.number);
            create_mentu_zihai(&colors.dragon, &TileType::Dragon)
        }
        TileType::Manzu => {
            remove_two_occurrences(&mut colors.manzu, head.number);
            create_mentu_suhai(&colors.manzu, &TileType::Manzu)
        }
        TileType::Pinzu => {
            remove_two_occurrences(&mut colors.pinzu, head.number);
            create_mentu_suhai(&colors.pinzu, &TileType::Pinzu)
        }
        TileType::Souzu => {
            remove_two_occurrences(&mut colors.souzu, head.number);
            create_mentu_suhai(&colors.souzu, &TileType::Souzu)
        }
        TileType::Wind => {
            remove_two_occurrences(&mut colors.wind, head.number);
            create_mentu_zihai(&colors.wind, &TileType::Wind)
        }
    };
    if head_mentsu.is_none() {
        return None;
    }
    let mut hands: Vec<Mentsu> = head_mentsu?;

    if !colors.dragon.is_empty() && head.tile_type != TileType::Dragon {
        let result = create_mentu_zihai(&colors.dragon, &TileType::Dragon);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }
    if !colors.manzu.is_empty() && head.tile_type != TileType::Manzu {
        create_mentu_suhai(&colors.manzu, &TileType::Manzu);
    }
    if !colors.pinzu.is_empty() && head.tile_type != TileType::Pinzu {
        create_mentu_suhai(&colors.pinzu, &TileType::Pinzu);
    }
    if !colors.souzu.is_empty() && head.tile_type != TileType::Souzu {
        create_mentu_suhai(&colors.souzu, &TileType::Souzu);
    }
    if !colors.wind.is_empty() && head.tile_type != TileType::Wind {
        create_mentu_zihai(&colors.wind, &TileType::Wind);
    }

    if hands.len() == 5 {
        let result: Hand = hands.try_into().expect("unknown error");

        return Some(result);
    }

    None
}

fn create_mentu_suhai(hand: &Vec<u8>, tile_type: &TileType) -> Option<Vec<Mentsu>> {
    let anko_candidate = extract_anko(hand);
    let anko_combinations = generate_combinations(&anko_candidate);

    for ankos in anko_combinations {
        let mut result: Vec<Mentsu> = Vec::new();
        let mut hand = hand.clone();
        for anko in &ankos {
            // TODO 4枚構成に対応できないから専用の関数が必要
            hand.retain(|&x| x != *anko);
        }
        for _ in 0..hand.len() % 3 {
            let min = hand.iter().min()?;
            if hand.contains(&(min + 1)) && hand.contains(&(min + 2)) {
                result.push(Shuntsu(Tile { number: *min, tile_type: tile_type.clone() }, false));
            } else {
                continue;
            }
        }
        if result.len() == hand.len() / 3 {
            for anko in &ankos {
                result.push(Koutsu(Tile { number: *anko, tile_type: tile_type.clone() }, false));
            }
            return Some(result);
        }
    }


    None
}

fn create_mentu_zihai(hand: &Vec<u8>, tile_type: &TileType) -> Option<Vec<Mentsu>> {
    let anko_candidate = extract_anko(hand);
    if hand.len() % 3 != anko_candidate.len() {
        return None;
    }

    let mut result: Vec<Mentsu> = Vec::new();
    for anko in anko_candidate {
        result.push(Mentsu::Koutsu(Tile { number: anko, tile_type: tile_type.clone() }, false));
    }

    Some(result)
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

fn extract_anko(tiles: &Vec<u8>) -> Vec<u8> {
    let mut counts = HashMap::new();

    for &tile in tiles.iter() {
        *counts.entry(tile).or_insert(0) += 1;
    }

    let mut anko = Vec::new();
    for (&tile, &count) in counts.iter() {
        if count >= 3 {
            anko.push(tile);
        }
    }

    let mut tilesc = tiles.clone();
    for &tile in anko.iter() {
        tilesc.retain(|&x| x != tile);
    }

    anko
}

fn generate_combinations(candidates: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut combinations = Vec::new();
    combinations.push(Vec::new());
    let n = candidates.len();

    for i in 0..(1 << n) {
        let mut combination = Vec::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                combination.push(candidates[j]);
            }
        }
        combinations.push(combination);
    }
    combinations
}
