use crate::calculator::fu::calculate_fu;
use crate::calculator::result::ScoreResult;
use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::result::FoundResult;

mod dealer;
mod child;

pub fn calc_score(yaku: &FoundResult, field: &Field, hand: &WinningHand, status: &Status) -> ScoreResult {
    let fu = calculate_fu(field, hand, status);
    let is_dealer = field.bakaze.clone() as u8 == 1;

    if is_dealer {
        dealer::calc(fu, yaku, &status.win_method)
    } else {
        child::calc(fu, yaku, &status.win_method)
    }
}
