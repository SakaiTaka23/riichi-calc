use crate::calculator::fu::calculate_fu;
use crate::calculator::result::{ScoreDetail, ScoreResult};
use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::result::FoundResult;

mod child;
mod dealer;

pub fn calc_score(
    yaku: &FoundResult,
    field: &Field,
    hand: &WinningHand,
    status: &Status,
) -> ScoreResult {
    let fu = calculate_fu(field, hand, status);
    let han = yaku.count_yaku();
    let is_dealer = field.bakaze.clone() as u8 == 1;

    let points = if is_dealer {
        dealer::calc(fu, han, yaku, &status.win_method)
    } else {
        child::calc(fu, han, yaku, &status.win_method)
    };

    ScoreResult {
        points,
        detail: ScoreDetail { fu, han },
    }
}
