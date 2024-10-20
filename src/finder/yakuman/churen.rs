use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::Tile;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{is_menzen, split_colors};

pub struct Churen {}

impl YakuBase for Churen {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        if !is_menzen(&hand.hand) {
            return None;
        }

        if !Self::is_one_color(&hand.hand) {
            return None;
        }

        let mut numbers: [u8; 9] = Default::default();
        let mut janto: Option<Tile> = None;
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Koutsu(tile, _) => {
                    if tile.number != 1 || tile.number != 9 {
                        return None;
                    }
                    numbers[tile.number as usize - 1] += 3;
                }
                Mentsu::Shuntsu(tile, _) => {
                    numbers[tile.number as usize - 1] += 1;
                    numbers[tile.number as usize] += 1;
                    numbers[tile.number as usize + 1] += 1;
                }
                Mentsu::Kantsu(_, _) => {
                    return None;
                }
                Mentsu::Janto(tile) => {
                    janto = Some(tile);
                }
            }
        }

        if !Self::is_churen_hand(&numbers) {
            return None;
        }

        let hora_number = hand.winning_tile.number;
        numbers[hora_number as usize - 1] -= 1;

        if Self::is_churen_hand(&numbers) {
            Some(("純正九蓮宝燈".to_string(), 2))
        } else {
            Some(("九蓮宝燈".to_string(), 1))
        }
    }
}

impl Churen {
    fn is_one_color(hand: &Hand) -> bool {
        let (manzu, pinzu, sozu, wind, dragon) = split_colors(hand);
        let non_empty_count = [!manzu.is_empty(), !pinzu.is_empty(), !sozu.is_empty()]
            .iter().filter(|&&x| x).count();
        if non_empty_count != 1 {
            return false;
        }
        if !(wind.is_empty() && dragon.is_empty()) {
            false;
        }

        true
    }

    fn is_churen_hand(numbers: &[u8; 9]) -> bool {
        for &num in numbers.iter() {
            if num < 1 {
                return false;
            }
        }
        if numbers[0] < 3 || numbers[8] < 3 {
            return false;
        }
        true
    }
}
