use crate::calculator::fu::FuBase;
use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;

pub struct Machi {}

impl FuBase for Machi {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> u8 {
        if Self::is_tanki(hand) || Self::is_kanchan(hand) || Self::is_penchan(hand) {
            2
        } else {
            0
        }
    }
}

impl Machi {
    fn is_tanki(hand: &WinningHand) -> bool {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Janto(tile) => {
                    return if tile == hand.winning_tile {
                        true
                    } else {
                        false
                    }
                }
                _ => { continue }
            }
        }

        false
    }

    fn is_kanchan(hand: &WinningHand) -> bool {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(tile, _) => {
                    return if tile.tile_type == hand.winning_tile.tile_type && tile.number == hand.winning_tile.number - 1 {
                        true
                    } else {
                        continue
                    }
                }
                _ => { continue }
            }
        }

        false
    }

    fn is_penchan(hand: &WinningHand) -> bool {
        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => {
                    return if Self::is_penchan3(hand) || Self::is_penchan7(hand) {
                        true
                    } else {
                        continue
                    }
                }
                _ => { continue }
            }
        }

        false
    }

    fn is_penchan3(hand: &WinningHand) -> bool {
        let winning_tile = hand.winning_tile.clone();
        if winning_tile.number != 3 {
            return false;
        }

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(tile, _) => {
                    return if tile.tile_type == winning_tile.tile_type && tile.number == 1 {
                        true
                    } else {
                        continue
                    }
                }
                _ => { continue }
            }
        }

        false
    }

    fn is_penchan7(hand: &WinningHand) -> bool {
        let winning_tile = hand.winning_tile.clone();
        if winning_tile.number != 7 {
            return false;
        }

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(tile, _) => {
                    return if tile.tile_type == winning_tile.tile_type && tile.number == 7 {
                        true
                    } else {
                        continue
                    }
                }
                _ => { continue }
            }
        }

        false
    }
}
