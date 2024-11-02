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

const YAKUMAN_SCORE_BASE: u32 = 32000;
fn yakuman_calc(han: u8, win_method: WinMethod) -> ScoreResult {
    let han: u32 = u32::from(han);
    let score_sum: u32 = YAKUMAN_SCORE_BASE * han;
    match win_method {
        WinMethod::Tumo => {
            ScoreResult::ChildTumo(score_sum / 4, score_sum / 2)
        }
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
        _ => ScoreResult::ChildTumo(0, 0),
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
        2 => (400, 700),
        3 => (700, 1300),
        4 => (1300, 2600),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn tumo_25fu(han: u8) -> ScoreResult {
    let score = match han {
        3 => (800, 1600),
        4 => (1600, 3200),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

const CHILD_25_FU_RON_BASE: u32 = 1600;
fn ron_25fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => CHILD_25_FU_RON_BASE,
        3 => CHILD_25_FU_RON_BASE * 2,
        4 => CHILD_25_FU_RON_BASE * 3,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (300, 500),
        2 => (500, 1000),
        3 => (1000, 2000),
        4 => (2000, 3900),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1000,
        2 => 2000,
        3 => 3900,
        4 => 7700,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (400, 700),
        2 => (700, 1300),
        3 => (1300, 2600),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1300,
        2 => 2600,
        3 => 5200,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (400, 800),
        2 => (800, 1600),
        3 => (1600, 3200),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1600,
        2 => 3200,
        3 => 6400,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (500, 1000),
        2 => (1000, 2000),
        3 => (2000, 3900),
        4 => (2000, 400),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2000,
        2 => 3900,
        3 => 7700,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (600, 1200),
        2 => (1200, 2300),
        3 => (2000, 4000),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2300,
        2 => 4500,
        3 => 8000,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (700, 1300),
        2 => (1300, 2600),
        3 => (2000, 4000),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2600,
        2 => 5200,
        3 => 8000,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (800, 1500),
        2 => (1500, 2900),
        3 => (2000, 4000),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2900,
        2 => 5800,
        3 => 8000,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (800, 1600),
        2 => (1600, 3200),
        3 => (2000, 4000),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3200,
        2 => 6400,
        3 => 8000,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}

fn tumo_110fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => (1800, 3600),
        3 => (2000, 4000),
        4 => (2000, 4000),
        _ => (0, 0),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_110fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3600,
        2 => 7100,
        3 => 8000,
        4 => 8000,
        _ => 0,
    };
    ScoreResult::Ron(score)
}
