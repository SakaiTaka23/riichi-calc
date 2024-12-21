#[derive(Debug)]
pub struct ScoreResult {
    pub points: Points,
    pub detail: ScoreDetail,
}

#[derive(Debug, PartialEq)]
pub enum Points {
    ChildTumo(u32, u32),
    DealerTumo(u32),
    Ron(u32),
}

#[derive(Debug)]
pub struct ScoreDetail {
    pub han: u8,
    pub fu: u8,
}
