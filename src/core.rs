use crate::calculator;
use crate::calculator::result::{Points, ScoreResult};
use crate::constants::hand::WinningHand;
use crate::core::CalcError::{HandValidationError, NoYakuError};
use crate::finder::finder;
use crate::finder::result::FoundResult;
use crate::parser::{Input, ValidationError};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CalcError {
    HandValidationError(ValidationError),
    NoYakuError(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Output {
    winning_hand: WinningHand,
    found_result: FoundResult,
    score_result: ScoreResult,
}

type Outputs = Vec<Output>;

impl Input {
    /// calculate the score of the hand
    ///
    /// outputs [Output] if success
    ///
    /// outputs [CalcError] if fails
    pub fn calc_hand(self) -> Result<Output, CalcError> {
        let hands = self.parse_hand();
        if hands.is_err() {
            return Err(HandValidationError(hands.unwrap_err()));
        }

        let mut outputs: Outputs = vec![];
        for hand in hands.unwrap() {
            let found = finder::Finder::find_hand(&hand);
            if !found.is_valid_hora() {
                continue;
            }
            let score = calculator::score::calc_score(
                &found,
                &hand.field,
                &hand.winning_hand,
                &hand.status,
            );
            outputs.push(Output {
                winning_hand: hand.winning_hand,
                found_result: found,
                score_result: score,
            });
        }

        if outputs.len() == 0 {
            return Err(NoYakuError("No yaku found".to_string()));
        }

        Ok(get_highest(outputs))
    }
}

fn get_highest(results: Outputs) -> Output {
    let mut result_sum: HashMap<u32, Output> = HashMap::new();

    for result in results {
        let sum = match result.score_result.points {
            Points::ChildTumo(x, y) => x * 2 + y,
            Points::DealerTumo(x) | Points::Ron(x) => x,
        };

        result_sum.insert(sum, result);
    }

    let highest = result_sum.into_iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();

    highest.1
}
