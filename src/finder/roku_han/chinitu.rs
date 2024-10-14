use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::utils::{check_kuisagari, split_colors};

pub struct Chinitu {}

impl YakuBase for Chinitu {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<(String, u8)> {
        let (manzu, pinzu, sozu, wind, dragon) = split_colors(&hand.hand);
        let non_empty_count = [!manzu.is_empty(), !pinzu.is_empty(), !sozu.is_empty(), !wind.is_empty(), !dragon.is_empty()]
            .iter().filter(|&&x| x).count();

        if non_empty_count != 1 {
            return None;
        }
        if !(wind.is_empty() && dragon.is_empty()) {
            return None;
        }

        check_kuisagari(&hand.hand, "清一色".to_string(), 6)
    }
}
