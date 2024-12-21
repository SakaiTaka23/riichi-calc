use crate::calculator::result::Points;
use crate::constants::status::WinMethod;
use crate::finder::result::FoundResult;

pub fn calc(fu: u8, han: u8, yaku: &FoundResult, win_method: &WinMethod) -> Points {
    match yaku {
        FoundResult::FoundYaku(_) => match win_method {
            WinMethod::Tumo => tumo_calc(fu, han),
            WinMethod::Ron => ron_calc(fu, han),
        },
        FoundResult::FoundYakuman(_) => yakuman_calc(han, win_method.clone()),
    }
}

const YAKUMAN_SCORE_BASE: u32 = 48000;
fn yakuman_calc(han: u8, win_method: WinMethod) -> Points {
    let han: u32 = u32::from(han);
    let score_sum: u32 = YAKUMAN_SCORE_BASE * han;
    match win_method {
        WinMethod::Tumo => Points::DealerTumo(score_sum / 3),
        WinMethod::Ron => Points::Ron(score_sum),
    }
}

fn tumo_calc(fu: u8, han: u8) -> Points {
    match han {
        0..=4 => {}
        5 => return Points::DealerTumo(4000),
        6 | 7 => return Points::DealerTumo(6000),
        8 | 9 | 10 => return Points::DealerTumo(8000),
        11 | 12 => return Points::DealerTumo(12000),
        13..=u8::MAX => return Points::DealerTumo(16000),
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
        _ => Points::DealerTumo(0),
    }
}

fn ron_calc(fu: u8, han: u8) -> Points {
    match han {
        0..=4 => {}
        5 => return Points::Ron(12000),
        6 | 7 => return Points::Ron(18000),
        8 | 9 | 10 => return Points::Ron(24000),
        11 | 12 => return Points::Ron(36000),
        13..=u8::MAX => return Points::Ron(48000),
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
        _ => Points::Ron(0),
    }
}

fn tumo_20fu(han: u8) -> Points {
    let score = match han {
        2 => 700,
        3 => 1300,
        4 => 2600,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn tumo_25fu(han: u8) -> Points {
    let score = match han {
        3 => 1600,
        4 => 3200,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

const DEALER_25_FU_RON_BASE: u32 = 2400;
fn ron_25fu(han: u8) -> Points {
    let score = match han {
        2 => DEALER_25_FU_RON_BASE,
        3 => DEALER_25_FU_RON_BASE * 2,
        4 => DEALER_25_FU_RON_BASE * 4,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_30fu(han: u8) -> Points {
    let score = match han {
        1 => 500,
        2 => 1000,
        3 => 2000,
        4 => 3900,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_30fu(han: u8) -> Points {
    let score = match han {
        1 => 1500,
        2 => 2900,
        3 => 5800,
        4 => 11600,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_40fu(han: u8) -> Points {
    let score = match han {
        1 => 700,
        2 => 1300,
        3 => 2600,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_40fu(han: u8) -> Points {
    let score = match han {
        1 => 2000,
        2 => 3900,
        3 => 7700,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_50fu(han: u8) -> Points {
    let score = match han {
        1 => 800,
        2 => 1600,
        3 => 3200,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_50fu(han: u8) -> Points {
    let score = match han {
        1 => 2400,
        2 => 4800,
        3 => 9600,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_60fu(han: u8) -> Points {
    let score = match han {
        1 => 1000,
        2 => 2000,
        3 => 3900,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_60fu(han: u8) -> Points {
    let score = match han {
        1 => 2900,
        2 => 5800,
        3 => 11600,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_70fu(han: u8) -> Points {
    let score = match han {
        1 => 1200,
        2 => 2300,
        3 => 4000,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_70fu(han: u8) -> Points {
    let score = match han {
        1 => 3400,
        2 => 6800,
        3 => 12000,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_80fu(han: u8) -> Points {
    let score = match han {
        1 => 1300,
        2 => 2600,
        3 => 4000,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_80fu(han: u8) -> Points {
    let score = match han {
        1 => 3900,
        2 => 7700,
        3 => 12000,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_90fu(han: u8) -> Points {
    let score = match han {
        1 => 1500,
        2 => 2900,
        3 => 4000,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_90fu(han: u8) -> Points {
    let score = match han {
        1 => 4400,
        2 => 8700,
        3 => 12000,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_100fu(han: u8) -> Points {
    let score = match han {
        1 => 1600,
        2 => 3200,
        3 => 4000,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_100fu(han: u8) -> Points {
    let score = match han {
        1 => 4800,
        2 => 9600,
        3 => 12000,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

fn tumo_110fu(han: u8) -> Points {
    let score = match han {
        1 => 1800,
        2 => 3600,
        3 => 4000,
        4 => 4000,
        _ => unreachable!(),
    };
    Points::DealerTumo(score)
}

fn ron_110fu(han: u8) -> Points {
    let score = match han {
        1 => 5300,
        2 => 10600,
        3 => 12000,
        4 => 12000,
        _ => unreachable!(),
    };
    Points::Ron(score)
}

#[cfg(test)]
mod table_test {
    use crate::calculator::result::Points;
    use crate::calculator::result::Points::{DealerTumo, Ron};
    use crate::calculator::score::dealer::{ron_calc, tumo_calc};
    use std::collections::HashMap;

    #[test]
    fn test_tumo() {
        const MANGAN_TUMO: Points = DealerTumo(4000);

        let tumo_20: Vec<(u8, Points)> = vec![
            (2, DealerTumo(700)),
            (3, DealerTumo(1300)),
            (4, DealerTumo(2600)),
        ];

        let tumo_25: Vec<(u8, Points)> = vec![(3, DealerTumo(1600)), (4, DealerTumo(3200))];

        let tumo_30: Vec<(u8, Points)> = vec![
            (1, DealerTumo(500)),
            (2, DealerTumo(1000)),
            (3, DealerTumo(2000)),
            (4, DealerTumo(3900)),
        ];

        let tumo_40: Vec<(u8, Points)> = vec![
            (1, DealerTumo(700)),
            (2, DealerTumo(1300)),
            (3, DealerTumo(2600)),
            (4, MANGAN_TUMO),
        ];

        let tumo_50: Vec<(u8, Points)> = vec![
            (1, DealerTumo(800)),
            (2, DealerTumo(1600)),
            (3, DealerTumo(3200)),
            (4, MANGAN_TUMO),
        ];

        let tumo_60: Vec<(u8, Points)> = vec![
            (1, DealerTumo(1000)),
            (2, DealerTumo(2000)),
            (3, DealerTumo(3900)),
            (4, MANGAN_TUMO),
        ];

        let tumo_70: Vec<(u8, Points)> = vec![
            (1, DealerTumo(1200)),
            (2, DealerTumo(2300)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_80: Vec<(u8, Points)> = vec![
            (1, DealerTumo(1300)),
            (2, DealerTumo(2600)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_90: Vec<(u8, Points)> = vec![
            (1, DealerTumo(1500)),
            (2, DealerTumo(2900)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_100: Vec<(u8, Points)> = vec![
            (1, DealerTumo(1600)),
            (2, DealerTumo(3200)),
            (3, MANGAN_TUMO),
            (4, MANGAN_TUMO),
        ];

        let tumo_110: Vec<(u8, Points)> =
            vec![(2, DealerTumo(3600)), (3, MANGAN_TUMO), (4, MANGAN_TUMO)];

        let table: HashMap<u8, Vec<(u8, Points)>> = HashMap::from([
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
        const MANGAN_RON: Points = Ron(12000);

        let ron_25: Vec<(u8, Points)> = vec![(2, Ron(2400)), (3, Ron(4800)), (4, Ron(9600))];

        let ron_30: Vec<(u8, Points)> = vec![
            (1, Ron(1500)),
            (2, Ron(2900)),
            (3, Ron(5800)),
            (4, Ron(11600)),
        ];

        let ron_40: Vec<(u8, Points)> = vec![
            (1, Ron(2000)),
            (2, Ron(3900)),
            (3, Ron(7700)),
            (4, MANGAN_RON),
        ];

        let ron_50: Vec<(u8, Points)> = vec![
            (1, Ron(2400)),
            (2, Ron(4800)),
            (3, Ron(9600)),
            (4, MANGAN_RON),
        ];

        let ron_60: Vec<(u8, Points)> = vec![
            (1, Ron(2900)),
            (2, Ron(5800)),
            (3, Ron(11600)),
            (4, MANGAN_RON),
        ];

        let ron_70: Vec<(u8, Points)> = vec![
            (1, Ron(3400)),
            (2, Ron(6800)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_80: Vec<(u8, Points)> = vec![
            (1, Ron(3900)),
            (2, Ron(7700)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_90: Vec<(u8, Points)> = vec![
            (1, Ron(4400)),
            (2, Ron(8700)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_100: Vec<(u8, Points)> = vec![
            (1, Ron(4800)),
            (2, Ron(9600)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let ron_110: Vec<(u8, Points)> = vec![
            (1, Ron(5300)),
            (2, Ron(10600)),
            (3, MANGAN_RON),
            (4, MANGAN_RON),
        ];

        let table: HashMap<u8, Vec<(u8, Points)>> = HashMap::from([
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
