#[derive(Debug, PartialEq)]
pub enum ScoreResult {
    ChildTumo(u32, u32),
    DealerTumo(u32),
    Ron(u32),
}
