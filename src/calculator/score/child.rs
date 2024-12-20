use crate::calculator::result::ScoreResult;
use crate::constants::status::WinMethod;
use crate::finder::result::FoundResult;

pub fn calc(fu: u8, yaku: &FoundResult, win_method: &WinMethod) -> ScoreResult {
    let han = yaku.count_yaku();
    match yaku {
        FoundResult::FoundYaku(_) => match win_method {
            WinMethod::Tumo => tumo_calc(fu, han),
            WinMethod::Ron => ron_calc(fu, han),
        },
        FoundResult::FoundYakuman(_) => yakuman_calc(han, win_method.clone()),
    }
}

const YAKUMAN_SCORE_BASE: u32 = 32000;
const MANGAN_RON: u32 = 8000;
const MANGAN_TUMO: (u32, u32) = (2000, 4000);
fn yakuman_calc(han: u8, win_method: WinMethod) -> ScoreResult {
    let han: u32 = u32::from(han);
    let score_sum: u32 = YAKUMAN_SCORE_BASE * han;
    match win_method {
        WinMethod::Tumo => ScoreResult::ChildTumo(score_sum / 4, score_sum / 2),
        WinMethod::Ron => ScoreResult::Ron(score_sum),
    }
}

fn tumo_calc(fu: u8, han: u8) -> ScoreResult {
    match han {
        0..=4 => {}
        5 => return ScoreResult::ChildTumo(2000, 4000),
        6 | 7 => return ScoreResult::ChildTumo(3000, 6000),
        8 | 9 | 10 => return ScoreResult::ChildTumo(4000, 8000),
        11 | 12 => return ScoreResult::ChildTumo(6000, 12000),
        13..=u8::MAX => return ScoreResult::ChildTumo(8000, 16000),
    }

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
        _ => unreachable!(),
    }
}

fn ron_calc(fu: u8, han: u8) -> ScoreResult {
    match han {
        0..=4 => {}
        5 => return ScoreResult::Ron(8000),
        6 | 7 => return ScoreResult::Ron(12000),
        8 | 9 | 10 => return ScoreResult::Ron(16000),
        11 | 12 => return ScoreResult::Ron(24000),
        13..=u8::MAX => return ScoreResult::Ron(36000),
    }

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
        _ => unreachable!(),
    }
}

fn tumo_20fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => (400, 700),
        3 => (700, 1300),
        4 => (1300, 2600),
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn tumo_25fu(han: u8) -> ScoreResult {
    let score = match han {
        3 => (800, 1600),
        4 => (1600, 3200),
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

const CHILD_25_FU_RON_BASE: u32 = 1600;
fn ron_25fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => CHILD_25_FU_RON_BASE,
        3 => CHILD_25_FU_RON_BASE * 2,
        4 => CHILD_25_FU_RON_BASE * 4,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (300, 500),
        2 => (500, 1000),
        3 => (1000, 2000),
        4 => (2000, 3900),
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_30fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1000,
        2 => 2000,
        3 => 3900,
        4 => 7700,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (400, 700),
        2 => (700, 1300),
        3 => (1300, 2600),
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_40fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1300,
        2 => 2600,
        3 => 5200,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (400, 800),
        2 => (800, 1600),
        3 => (1600, 3200),
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_50fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 1600,
        2 => 3200,
        3 => 6400,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (500, 1000),
        2 => (1000, 2000),
        3 => (2000, 3900),
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_60fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2000,
        2 => 3900,
        3 => 7700,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (600, 1200),
        2 => (1200, 2300),
        3 => MANGAN_TUMO,
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_70fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2300,
        2 => 4500,
        3 => MANGAN_RON,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (700, 1300),
        2 => (1300, 2600),
        3 => MANGAN_TUMO,
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_80fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2600,
        2 => 5200,
        3 => MANGAN_RON,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (800, 1500),
        2 => (1500, 2900),
        3 => MANGAN_TUMO,
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_90fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 2900,
        2 => 5800,
        3 => MANGAN_RON,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => (800, 1600),
        2 => (1600, 3200),
        3 => MANGAN_TUMO,
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_100fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3200,
        2 => 6400,
        3 => MANGAN_RON,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

fn tumo_110fu(han: u8) -> ScoreResult {
    let score = match han {
        2 => (1800, 3600),
        3 => MANGAN_TUMO,
        4 => MANGAN_TUMO,
        _ => unreachable!(),
    };
    ScoreResult::ChildTumo(score.0, score.1)
}

fn ron_110fu(han: u8) -> ScoreResult {
    let score = match han {
        1 => 3600,
        2 => 7100,
        3 => MANGAN_RON,
        4 => MANGAN_RON,
        _ => unreachable!(),
    };
    ScoreResult::Ron(score)
}

#[cfg(test)]
mod table_test {
    use crate::calculator::result::ScoreResult;
    use crate::calculator::result::ScoreResult::{ChildTumo, Ron};
    use crate::calculator::score::child::{ron_calc, tumo_calc};
    use std::collections::HashMap;

    #[test]
    fn test_tumo() {
        const MANGAN_TUMO: ScoreResult = ChildTumo(2000, 4000);

        let tumo_20: Vec<(u8, ScoreResult)> = vec![
            (2, ChildTumo(400, 700)),
            (3, ChildTumo(700, 1300)),
            (4, ChildTumo(1300, 2600)),
        ];

        let tumo_25: Vec<(u8, ScoreResult)> =
            vec![(3, ChildTumo(800, 1600)), (4, ChildTumo(1600, 3200))];

        let tumo_30: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(300, 500)),
            (2, ChildTumo(500, 1000)),
            (3, ChildTumo(1000, 2000)),
            (4, ChildTumo(2000, 3900)),
        ];

        let tumo_40: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(400, 700)),
            (2, ChildTumo(700, 1300)),
            (3, ChildTumo(1300, 2600)),
            (4, MANGAN_TUMO),
        ];

        let tumo_50: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(400, 800)),
            (2, ChildTumo(800, 1600)),
            (3, ChildTumo(1600, 3200)),
            (4, MANGAN_TUMO),
        ];

        let tumo_60: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(500, 1000)),
            (2, ChildTumo(1000, 2000)),
            (3, ChildTumo(2000, 3900)),
            (4, MANGAN_TUMO),
        ];

        let tumo_70: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(600, 1200)),
            (2, ChildTumo(1200, 2300)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_80: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(700, 1300)),
            (2, ChildTumo(1300, 2600)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_90: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(800, 1500)),
            (2, ChildTumo(1500, 2900)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_100: Vec<(u8, ScoreResult)> = vec![
            (1, ChildTumo(800, 1600)),
            (2, ChildTumo(1600, 3200)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_110: Vec<(u8, ScoreResult)> = vec![
            (2, ChildTumo(1800, 3600)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let table: HashMap<u8, Vec<(u8, ScoreResult)>> = HashMap::from([
            (20, tumo_20),
            (25, tumo_25),
            (30, tumo_30),
            (40, tumo_40),
            (50, tumo_50),
            (60, tumo_60),
            (70, tumo_70),
            (80, tumo_80),
            (90, tumo_90),
            (100, tumo_100),
            (110, tumo_110),
        ]);

        for (fu, expected) in table {
            for (han, expected_score) in expected {
                assert_eq!(
                    tumo_calc(fu, han),
                    expected_score,
                    "fu: {}, han: {}",
                    fu,
                    han
                );
            }
        }
    }

    #[test]
    fn test_ron() {
        const MANGAN_RON: ScoreResult = Ron(8000);

        let ron_25: Vec<(u8, ScoreResult)> = vec![(2, Ron(1600)), (3, Ron(3200)), (4, Ron(6400))];

        let ron_30: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(1000)),
            (2, Ron(2000)),
            (3, Ron(3900)),
            (4, Ron(7700)),
        ];

        let ron_40: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(1300)),
            (2, Ron(2600)),
            (3, Ron(5200)),
            (4, MANGAN_RON),
        ];

        let ron_50: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(1600)),
            (2, Ron(3200)),
            (3, Ron(6400)),
            (4, MANGAN_RON),
        ];

        let ron_60: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(2000)),
            (2, Ron(3900)),
            (3, Ron(7700)),
            (4, MANGAN_RON),
        ];

        let ron_70: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(2300)),
            (2, Ron(4500)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_80: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(2600)),
            (2, Ron(5200)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_90: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(2900)),
            (2, Ron(5800)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_100: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(3200)),
            (2, Ron(6400)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_110: Vec<(u8, ScoreResult)> = vec![
            (1, Ron(3600)),
            (2, Ron(7100)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let table: HashMap<u8, Vec<(u8, ScoreResult)>> = HashMap::from([
            (25, ron_25),
            (30, ron_30),
            (40, ron_40),
            (50, ron_50),
            (60, ron_60),
            (70, ron_70),
            (80, ron_80),
            (90, ron_90),
            (100, ron_100),
            (110, ron_110),
        ]);

        for (fu, expected) in table {
            for (han, expected_score) in expected {
                assert_eq!(
                    ron_calc(fu, han),
                    expected_score,
                    "fu: {}, han: {}",
                    fu,
                    han
                );
            }
        }
    }
}
