use crate::constants::hand::Mentsu::{Janto, Koutsu, Shuntsu};
use crate::constants::hand::{Hand, Mentsu};
use crate::constants::tiles::{Tile, TileType};
use crate::parser::pi_input::PiHandColor;
use std::collections::HashMap;

pub fn create_hand(colors: &mut PiHandColor, head: &Tile, naki: &Vec<Mentsu>) -> Option<Hand> {
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
    let mut hands: Vec<Mentsu> = vec![Janto(Tile { number: head.number, tile_type: head.tile_type.clone() })];
    hands.append(head_mentsu.clone().as_mut()?);

    if !colors.dragon.is_empty() && head.tile_type != TileType::Dragon {
        let result = create_mentu_zihai(&colors.dragon, &TileType::Dragon);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }
    if !colors.manzu.is_empty() && head.tile_type != TileType::Manzu {
        let result = create_mentu_suhai(&colors.manzu, &TileType::Manzu);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }
    if !colors.pinzu.is_empty() && head.tile_type != TileType::Pinzu {
        let result = create_mentu_suhai(&colors.pinzu, &TileType::Pinzu);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }
    if !colors.souzu.is_empty() && head.tile_type != TileType::Souzu {
        let result = create_mentu_suhai(&colors.souzu, &TileType::Souzu);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }
    if !colors.wind.is_empty() && head.tile_type != TileType::Wind {
        let result = create_mentu_zihai(&colors.wind, &TileType::Wind);
        if result.is_some() {
            hands.append(&mut result?);
        }
    }

    hands.append(&mut naki.clone());
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
        for _ in 0..hand.len() / 3 {
            let read_hand = hand.clone();
            let min = read_hand.iter().min()?;
            if read_hand.contains(&(min + 1)) && read_hand.contains(&(min + 2)) {
                result.push(Shuntsu(Tile { number: *min, tile_type: tile_type.clone() }, false));

                // for i in 0..3 {
                //     let index = read_hand.iter().position(|x| *x == *min + i)?;
                //     hand.remove(index);
                // }
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
    if hand.len() / 3 != anko_candidate.len() {
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

mod valid_hand_test {
    #[test]
    fn all_shuntsu_pinfu_iipeco() {
        use crate::constants::hand::Mentsu::{Janto, Shuntsu};
        use crate::constants::tiles::{Tile, TileType};
        use crate::parser::pi_input::hand_creator::create_hand;
        use crate::parser::pi_input::PiHandColor;

        let colors = &PiHandColor {
            dragon: vec![],
            manzu: vec![1, 2, 3, 1, 2, 3],
            pinzu: vec![4, 5, 6, 9, 9],
            souzu: vec![6, 7, 8],
            wind: vec![],
        };
        let head = Tile { number: 9, tile_type: TileType::Pinzu };
        let hand = [
            Janto(Tile { number: 9, tile_type: TileType::Pinzu }),
            Shuntsu(Tile { number: 4, tile_type: TileType::Pinzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
            Shuntsu(Tile { number: 6, tile_type: TileType::Souzu }, false),
        ];
        assert_eq!(create_hand(&mut colors.clone(), &head, &vec![]).unwrap(), hand);
    }

    #[test]
    fn anko_bukumi_ryuiso_with_naki() {
        use crate::constants::hand::Mentsu::{Janto, Koutsu, Shuntsu};
        use crate::constants::tiles::{Tile, TileType};
        use crate::parser::pi_input::hand_creator::create_hand;
        use crate::parser::pi_input::PiHandColor;

        let colors = &PiHandColor {
            dragon: vec![2, 2, 2],
            manzu: vec![],
            pinzu: vec![],
            souzu: vec![1, 2, 3, 6, 6, 6, 8, 8],
            wind: vec![],
        };
        let head = Tile { number: 8, tile_type: TileType::Souzu };
        let naki = vec![Shuntsu(Tile { number: 1, tile_type: TileType::Souzu }, false), ];

        let hand = [
            Janto(Tile { number: 8, tile_type: TileType::Souzu }),
            Shuntsu(Tile { number: 1, tile_type: TileType::Souzu }, false),
            Koutsu(Tile { number: 6, tile_type: TileType::Souzu }, false),
            Koutsu(Tile { number: 2, tile_type: TileType::Dragon }, false),
            Shuntsu(Tile { number: 1, tile_type: TileType::Souzu }, false),
        ];
        assert_eq!(create_hand(&mut colors.clone(), &head, &naki).unwrap(), hand);
    }
}

mod private_fn_test {
    #[test]
    fn shuntsu_from_suhai() {
        use crate::constants::hand::Mentsu::Shuntsu;
        use crate::constants::tiles::{Tile, TileType};
        use crate::parser::pi_input::hand_creator::create_mentu_suhai;

        let hand: Vec<u8> = vec![1, 2, 3];
        let hand = create_mentu_suhai(&hand, &TileType::Manzu).unwrap();
        assert_eq!(hand, vec![
            Shuntsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
        ]);
    }

    #[test]
    fn anko_from_suhai() {
        use crate::constants::hand::Mentsu::Koutsu;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType;
        use crate::parser::pi_input::hand_creator::create_mentu_suhai;

        let hand: Vec<u8> = vec![1, 1, 1];
        let hand = create_mentu_suhai(&hand, &TileType::Manzu).unwrap();
        assert_eq!(hand, vec![
            Koutsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
        ]);
    }

    #[test]
    fn shuntsu_anko_combination_from_suhai() {
        use crate::constants::hand::Mentsu::Koutsu;
        use crate::constants::hand::Mentsu::Shuntsu;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType;
        use crate::parser::pi_input::hand_creator::create_mentu_suhai;

        let hand: Vec<u8> = vec![1, 1, 1, 4, 5, 6];
        let hand = create_mentu_suhai(&hand, &TileType::Manzu).unwrap();
        assert_eq!(hand, vec![
            Shuntsu(Tile { number: 4, tile_type: TileType::Manzu }, false),
            Koutsu(Tile { number: 1, tile_type: TileType::Manzu }, false),
        ]);
    }

    #[test]
    fn shuntsu_anko_combination_from_suhai_reverse() {
        use crate::constants::hand::Mentsu::Koutsu;
        use crate::constants::hand::Mentsu::Shuntsu;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType;
        use crate::parser::pi_input::hand_creator::create_mentu_suhai;

        let hand: Vec<u8> = vec![4, 5, 6, 7, 7, 7];
        let hand = create_mentu_suhai(&hand, &TileType::Manzu).unwrap();
        assert_eq!(hand, vec![
            Shuntsu(Tile { number: 4, tile_type: TileType::Manzu }, false),
            Koutsu(Tile { number: 7, tile_type: TileType::Manzu }, false),
        ]);
    }

    #[test]
    fn anko_from_zihai() {
        use crate::constants::hand::Mentsu::Koutsu;
        use crate::constants::tiles::Tile;
        use crate::constants::tiles::TileType;
        use crate::parser::pi_input::hand_creator::create_mentu_suhai;

        let hand: Vec<u8> = vec![1, 1, 1];
        let hand = create_mentu_suhai(&hand, &TileType::Dragon).unwrap();
        assert_eq!(hand, vec![
            Koutsu(Tile { number: 1, tile_type: TileType::Dragon }, false),
        ]);
    }
}
