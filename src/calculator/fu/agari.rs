use crate::calculator::fu::FuBase;
use crate::constants::field::Field;
use crate::constants::hand::{Hand, Mentsu, WinningHand};
use crate::constants::status::{Status, WinMethod};

pub struct Agari {}

impl FuBase for Agari {
    fn validate(_: &Field, hand: &WinningHand, status: &Status) -> u8 {
        match status.win_method {
            WinMethod::Ron => {
                if Self::is_menzen(&hand.hand) { 10 } else { 0 }
            }
            WinMethod::Tumo => { 2 }
        }
    }
}

impl Agari {
    fn is_menzen(hand: &Hand) -> bool {
        for mentsu in hand {
            match mentsu {
                Mentsu::Koutsu(_, x)
                | Mentsu::Shuntsu(_, x)
                | Mentsu::Kantsu(_, x) => { if *x { return false; } }
                Mentsu::Janto(_) => {}
            }
        }

        true
    }
}
