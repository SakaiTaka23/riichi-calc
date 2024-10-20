use crate::constants::found_yaku::FoundYaku;
use crate::finder::ii_han::ii_han_yaku;
use crate::finder::roku_han::roku_han_yaku;
use crate::finder::ryan_han::ryan_han_yaku;
use crate::finder::san_han::san_han_yaku;
use crate::finder::yakuman::yakuman_yaku;
use crate::parser::input::ParsedHand;

pub struct Finder {
    pub parsed_hand: ParsedHand,
}

impl Finder {
    pub fn find_hand(&self) -> FoundYaku {
        let ii_han = ii_han_yaku(&self.parsed_hand.field, &self.parsed_hand.winning_hand, &self.parsed_hand.status);
        let ryan_han = ryan_han_yaku(&self.parsed_hand.field, &self.parsed_hand.winning_hand, &self.parsed_hand.status);
        let san_han = san_han_yaku(&self.parsed_hand.field, &self.parsed_hand.winning_hand, &self.parsed_hand.status);
        let roku_han = roku_han_yaku(&self.parsed_hand.field, &self.parsed_hand.winning_hand, &self.parsed_hand.status);
        let yakuman = yakuman_yaku(&self.parsed_hand.field, &self.parsed_hand.winning_hand, &self.parsed_hand.status);

        FoundYaku {
            ii_han,
            ryan_han,
            san_han,
            roku_han,
            yakuman,
        }
    }
}
