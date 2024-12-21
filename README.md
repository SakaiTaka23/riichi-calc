# Riichi Calc

Library to calculate the score of a hand in riichi mahjong

## Example

```rust
let input = Input::new(
pi_input, // infomation about the pi in hand
field, // information about the field
status // player's status
);

// calculation function
let result = input.calc_hand();
if result.is_err() {
panic ! ("invalid input");
}
let result = result.unwrap();
```

expected output

```txt
Output {
    winning_hand: WinningHand {
        hand: [
            Janto(
                Tile {
                    number: 9,
                    tile_type: Souzu,
                },
            ),
            Shuntsu(
                Tile {
                    number: 5,
                    tile_type: Souzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 1,
                    tile_type: Manzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 5,
                    tile_type: Manzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 2,
                    tile_type: Pinzu,
                },
                false,
            ),
        ],
        winning_tile: Tile {
            number: 5,
            tile_type: Souzu,
        },
        red_tile: 0,
    },
    found_result: FoundYaku(
        FoundYaku {
            dora: [
                (
                    "ドラ",
                    1,
                ),
                (
                    "裏ドラ",
                    1,
                ),
            ],
            ii_han: [
                (
                    "立直",
                    1,
                ),
                (
                    "平和",
                    1,
                ),
            ],
            ryan_han: [],
            san_han: [],
            roku_han: [],
        },
    ),
    score_result: ScoreResult {
        points: Ron(
            11600,
        ),
        actual_points: Ron(
            12200,
        ),
        detail: ScoreDetail {
            han: 4,
            fu: 30,
        },
    },
}

```
