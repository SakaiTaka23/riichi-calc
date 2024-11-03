use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::is_same_wind;

pub struct Pinfu;

impl YakuBase for Pinfu {
    fn validate(field: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, open) => { if open { return None; } }
                Mentsu::Janto(tile) => { if !Self::is_valid_janto(&tile, field) { return None; } }
                _ => { return None }
            }
        }

        if !Self::is_riyanmen(&hand) {
            return None;
        }
        Some(("平和".to_string(), 1))
    }
}

impl Pinfu {
    fn is_valid_janto(tile: &Tile, field: &Field) -> bool {
        match tile.tile_type {
            TileType::Manzu | TileType::Pinzu | TileType::Souzu => { true }
            TileType::Wind => {
                if is_same_wind(tile.number, &field.bakaze) || is_same_wind(tile.number, &field.zikaze) {
                    return false;
                }
                true
            }
            TileType::Dragon => { false }
        }
    }

    fn is_riyanmen(WinningHand { hand, winning_tile, .. }: &WinningHand) -> bool {
        let start_number = hand.into_iter()
            .filter(|m| match m {
                Mentsu::Shuntsu(_, _) => true,
                _ => false
            })
            .filter(|m| m.tile().tile_type == winning_tile.tile_type)
            .into_iter()
            .map(|m| m.tile().number)
            .collect::<Vec<u8>>();

        match winning_tile.number {
            1 => { start_number.contains(&1) }
            2 => { start_number.contains(&2) }
            3 => { start_number.contains(&1) | start_number.contains(&3) }
            4 => { start_number.contains(&2) | start_number.contains(&4) }
            5 => { start_number.contains(&3) | start_number.contains(&5) }
            6 => { start_number.contains(&4) | start_number.contains(&6) }
            7 => { start_number.contains(&5) | start_number.contains(&7) }
            8 => { start_number.contains(&6) }
            9 => { start_number.contains(&7) }
            _ => { false }
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::field::{Field, Wind};
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::pinfu::Pinfu;
    use crate::finder::test_utils::{random_field, random_janto, random_shuntu, random_shuntu_number, random_status, random_tile};

    #[test]
    fn valid_pinfu() {
        let hand = [
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_janto(true),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), Some(("平和".to_string(), 1)), "{:?}", hand);
    }

    #[test]
    fn aranmen_machi() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: 1 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Janto(Tile { tile_type: TileType::Souzu, number: 1 }),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), Some(("平和".to_string(), 1)), "{:?}", hand);
    }

    #[test]
    fn head_is_otakaze() {
        let field = Field { zikaze: Wind::East, bakaze: Wind::South, dora: vec![random_tile()] };
        let hand = [
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            Mentsu::Janto(Tile { tile_type: TileType::Wind, number: 3 }),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), Some(("平和".to_string(), 1)), "{:?}", hand);
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::field::{Field, Wind};
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ii_han::pinfu::Pinfu;
    use crate::finder::test_utils::{random_field, random_janto, random_shuntu, random_shuntu_number, random_status, random_tile, random_tile_type};
    use rand::{random, Rng};

    #[test]
    fn not_menzen() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: random_tile_type(), number: random_shuntu_number() }, true),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_janto(true),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn has_anko() {
        let hand = [
            Mentsu::Koutsu(Tile { tile_type: random_tile_type(), number: random_shuntu_number() }, false),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_janto(true),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn has_jihai() {
        let hand = [
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 1 }, random()),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_janto(true),
        ];
        let winning_tile = hand[1].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn kanchan_machi() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            random_janto(true),
        ];
        let mut winning_tile = hand[0].tile();
        winning_tile.number += 1;
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn penchan_machi() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: 1 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            random_janto(true),
        ];
        let mut winning_tile = Tile { tile_type: TileType::Souzu, number: 3 };
        winning_tile.number += 1;
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);

        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: 7 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            random_janto(true),
        ];
        let mut winning_tile = Tile { tile_type: TileType::Souzu, number: 7 };
        winning_tile.number += 1;
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn tanki_machi() {
        let hand = [
            Mentsu::Shuntsu(Tile { tile_type: TileType::Souzu, number: 1 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Pinzu, number: random_shuntu_number() }, false),
            Mentsu::Janto(Tile { tile_type: TileType::Souzu, number: 9 }),
        ];
        let winning_tile = hand[4].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let field = random_field();
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn head_is_yakuhai_wind() {
        let field = Field { zikaze: Wind::East, bakaze: Wind::South, dora: vec![random_tile()] };
        let hand = [
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            Mentsu::Janto(Tile { tile_type: TileType::Wind, number: 2 }),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }

    #[test]
    fn head_is_yakuhai_dragon() {
        let field = Field { zikaze: Wind::East, bakaze: Wind::South, dora: vec![random_tile()] };
        let hand = [
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            random_shuntu(false),
            Mentsu::Janto(Tile { tile_type: TileType::Dragon, number: rand::thread_rng().gen_range(1..=3) }),
        ];
        let winning_tile = hand[0].tile();
        let winning_hand = WinningHand { hand, winning_tile, red_tile: 0 };
        let status = random_status();
        assert_eq!(Pinfu::validate(&field, &winning_hand, &status), None, "{:?}", hand);
    }
}
