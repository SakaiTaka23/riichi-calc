use crate::calculator::result::Points;

pub fn calc_actual_points(points: &Points, honba: u8) -> Points {
    match points {
        Points::ChildTumo(x, y) => {
            Points::ChildTumo(x + 100 * honba as u32, y + 100 * honba as u32)
        }
        Points::DealerTumo(x) => Points::DealerTumo(x + 100 * honba as u32),
        Points::Ron(x) => Points::Ron(x + 300 * honba as u32),
    }
}
