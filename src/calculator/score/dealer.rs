use crate::calculator::result::ScoreResult;
use crate::constants::status::WinMethod;
use crate::finder::result::FoundResult;

pub fn calc(fu: u8, yaku: &FoundResult, win_method: &WinMethod) -> ScoreResult {
    let han = yaku.count_yaku();
    match yaku {
        FoundResult::FoundYaku(_) => {
            match win_method {
                WinMethod::Tumo => tumo_calc(fu, han),
                WinMethod::Ron => ron_calc(fu, han),
            }
        }
        FoundResult::FoundYakuman(_) => {
            yakuman_calc(han, win_method.clone())
        }
    }
}

const YAKUMAN_SCORE_BASE: u32 = 48000;
fn yakuman_calc(han: u8, win_method: WinMethod) -> ScoreResult {
    let han: u32 = u32::from(han);
    let score_sum: u32 = YAKUMAN_SCORE_BASE * han;
    match win_method {
        WinMethod::Tumo => ScoreResult::DealerTumo(score_sum / 3),
        WinMethod::Ron => ScoreResult::Ron(score_sum),
    }
}

fn tumo_calc(fu: u8, han: u8) -> ScoreResult {
    match fu {
        20 => tumo_20fu(han),
        25 => tumo_25fu(han),
        30 => tumo_30fu(han),
        40 => tumo_40fu(han),
        50 => tumo_50fu(han),
        60 => tumo_60fu(han),
        70 => tumo_70fu(han),
        80 => tumo_80fu(han),
        90 => tumo_90fu(han),
        100 => tumo_100fu(han),
        110 => tumo_110fu(han),
        _ => ScoreResult::DealerTumo(0),
    }
}

fn ron_calc(fu: u8, han: u8) -> ScoreResult {
    match fu {
        25 => ron_25fu(han),
        30 => ron_30fu(han),
        40 => ron_40fu(han),
        50 => ron_50fu(han),
        60 => ron_60fu(han),
        70 => ron_70fu(han),
        80 => ron_80fu(han),
        90 => ron_90fu(han),
        100 => ron_100fu(han),
        110 => ron_110fu(han),
        _ => ScoreResult::Ron(0),
    }
}

fn tumo_20fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => 700,
        3 => 1300,
        4 => 2600,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn tumo_25fu(han: u8) -> ScoreResult {
    let score = match han {
        3 => 1600,
        4 => 3200,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

const DEALER_25_FU_RON_BASE: u32 = 2400;
fn ron_25fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => DEALER_25_FU_RON_BASE,
        3 => DEALER_25_FU_RON_BASE * 2,
        4 => DEALER_25_FU_RON_BASE * 4,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 500,
        2 => 1000,
        3 => 2000,
        4 => 3900,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1500,
        2 => 2900,
        3 => 5800,
        4 => 11600,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 700,
        2 => 1300,
        3 => 2600,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2000,
        2 => 3900,
        3 => 7700,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 800,
        2 => 1600,
        3 => 3200,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2400,
        2 => 4800,
        3 => 9600,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1000,
        2 => 2000,
        3 => 3900,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2900,
        2 => 5800,
        3 => 11600,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1200,
        2 => 2300,
        3 => 4000,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3400,
        2 => 6800,
        3 => 12000,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1300,
        2 => 2600,
        3 => 4000,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3900,
        2 => 7700,
        3 => 12000,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1500,
        2 => 2900,
        3 => 4000,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 4400,
        2 => 8700,
        3 => 12000,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1600,
        2 => 3200,
        3 => 4000,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 4800,
        2 => 9600,
        3 => 12000,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_110fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1800,
        2 => 3600,
        3 => 4000,
        4 => 4000,
        _ => 0,
    };
    ScoreResult::DealerTumo(score)
}

fn ron_110fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 5300,
        2 => 10600,
        3 => 12000,
        4 => 12000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}
