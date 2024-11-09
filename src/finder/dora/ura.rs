use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::{RiichiStatus, Status};
use crate::finder::dora::dora_finder::find_dora;
use crate::finder::finder_base::YakuBase;

pub struct Ura;

impl YakuBase for Ura {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> Option<(String, u8)> {
        match &status.riichi {
            RiichiStatus::NoRiichi => {
                None
            }
            RiichiStatus::Riichi(dora) |
            RiichiStatus::DoubleRiichi(dora) => {
                let dora_count = find_dora(&dora, &hand.hand);
                Some(("裏ドラ".to_string(), dora_count))
            }
        }
    }
}

#[cfg(test)]
mod count {
    use crate::constants::hand::{Mentsu, WinningHand};
    use crate::constants::status::RiichiStatus;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::dora::ura::Ura;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_status};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref WINNING_HAND: WinningHand = from_hand([
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 1 }, false),
            Mentsu::Shuntsu(Tile { tile_type: TileType::Manzu, number: 4 }, false),
            Mentsu::Koutsu(Tile { tile_type: TileType::Wind, number: 3 }, false),
            Mentsu::Kantsu(Tile { tile_type: TileType::Pinzu, number: 2 }, false),
            Mentsu::Janto(Tile { tile_type: TileType::Manzu, number: 9 }),
        ]);
    }

    #[test]
    fn one_in_shuntu() {
        let mut status = random_status();
        status.riichi = RiichiStatus::Riichi(vec![
            Tile { tile_type: TileType::Manzu, number: 2 },
        ]);

        assert_eq!(Ura::validate(&random_field(), &WINNING_HAND, &status), Some(("裏ドラ".to_string(), 1)));
    }

    #[test]
    fn should_print_when_riichi_in_zero() {
        let mut status = random_status();
        status.riichi = RiichiStatus::Riichi(vec![
            Tile { tile_type: TileType::Souzu, number: 2 },
        ]);

        assert_eq!(Ura::validate(&random_field(), &WINNING_HAND, &status), Some(("裏ドラ".to_string(), 0)));
    }

    #[test]
    fn should_print_when_double_riichi_in_zero() {
        let mut status = random_status();
        status.riichi = RiichiStatus::DoubleRiichi(vec![
            Tile { tile_type: TileType::Souzu, number: 2 },
        ]);

        assert_eq!(Ura::validate(&random_field(), &WINNING_HAND, &status), Some(("裏ドラ".to_string(), 0)));
    }

    #[test]
    fn should_not_print_in_no_riichi() {
        let mut status = random_status();
        status.riichi = RiichiStatus::NoRiichi;

        assert_eq!(Ura::validate(&random_field(), &WINNING_HAND, &status), None);
    }
}
