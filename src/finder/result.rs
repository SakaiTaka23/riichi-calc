pub enum FoundResult {
    FoundYaku(FoundYaku),
    FoundYakuman(FoundYakuman),
}

pub struct FoundYaku {
    pub dora: Vec<(String, u8)>,
    pub ii_han: Vec<(String, u8)>,
    pub ryan_han: Vec<(String, u8)>,
    pub san_han: Vec<(String, u8)>,
    pub roku_han: Vec<(String, u8)>,
}

pub struct FoundYakuman {
    pub yakuman: Vec<(String, u8)>,
}

impl FoundResult {
    pub fn count_yaku(&self) -> u8 {
        match self {
            FoundResult::FoundYaku(yaku) => {
                yaku.dora.len() as u8 +
                    yaku.ii_han.len() as u8 +
                    yaku.ryan_han.len() as u8 +
                    yaku.san_han.len() as u8 +
                    yaku.roku_han.len() as u8
            }
            FoundResult::FoundYakuman(yakuaman) => {
                yakuaman.yakuman.len() as u8
            }
        }
    }
}
