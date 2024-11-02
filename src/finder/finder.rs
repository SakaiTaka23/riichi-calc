use crate::finder::dora::dora_count;
use crate::finder::ii_han::ii_han_yaku;
use crate::finder::result::{FoundResult, FoundYaku, FoundYakuman};
use crate::finder::roku_han::roku_han_yaku;
use crate::finder::ryan_han::ryan_han_yaku;
use crate::finder::san_han::san_han_yaku;
use crate::finder::yakuman::yakuman_yaku;
use crate::parser::ParsedHand;

pub struct Finder {}

impl Finder {
    pub fn find_hand(parsed_hand: &ParsedHand) -> FoundResult {
        let yakuman = yakuman_yaku(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);
        let is_yakuman = if yakuman.len() > 0 { true } else { false };

        if is_yakuman {
            return FoundResult::FoundYakuman(FoundYakuman {
                yakuman
            });
        }

        let dora = dora_count(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);
        let ii_han = ii_han_yaku(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);
        let ryan_han = ryan_han_yaku(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);
        let san_han = san_han_yaku(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);
        let roku_han = roku_han_yaku(&parsed_hand.field, &parsed_hand.winning_hand, &parsed_hand.status);

        FoundResult::FoundYaku(FoundYaku {
            dora,
            ii_han,
            ryan_han,
            san_han,
            roku_han,
        })
    }
}
