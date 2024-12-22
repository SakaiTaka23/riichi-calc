use riichi_calc::constants::field::{Field, Wind};
use riichi_calc::constants::hand::Mentsu;
use riichi_calc::constants::status::{RiichiStatus, Status, WinMethod};
use riichi_calc::constants::tiles::{Tile, TileType};
use riichi_calc::parser::{Input, PiInput};

fn main() {
    // 111m 3333m(kan) 777p WWW 88p
    // riichi pinfu dora ura
    let pi_input = PiInput {
        hand: vec![
            Tile {
                number: 1,
                tile_type: TileType::Manzu,
            },
            Tile {
                number: 1,
                tile_type: TileType::Manzu,
            },
            Tile {
                number: 1,
                tile_type: TileType::Manzu,
            },
            Tile {
                number: 7,
                tile_type: TileType::Pinzu,
            },
            Tile {
                number: 7,
                tile_type: TileType::Pinzu,
            },
            Tile {
                number: 7,
                tile_type: TileType::Pinzu,
            },
            Tile {
                number: 3,
                tile_type: TileType::Wind,
            },
            Tile {
                number: 3,
                tile_type: TileType::Wind,
            },
            Tile {
                number: 8,
                tile_type: TileType::Pinzu,
            },
            Tile {
                number: 8,
                tile_type: TileType::Pinzu,
            },
        ],
        naki: vec![Mentsu::Kantsu(
            Tile {
                number: 3,
                tile_type: TileType::Manzu,
            },
            false,
        )],
        hora: Tile {
            number: 3,
            tile_type: TileType::Wind,
        },
    };
    let field = Field {
        honba: 2,
        zikaze: Wind::East,
        bakaze: Wind::East,
        dora: vec![Tile {
            number: 1,
            tile_type: TileType::Manzu,
        }],
    };
    let status = Status {
        riichi: RiichiStatus::Riichi(vec![Tile {
            number: 2,
            tile_type: TileType::Manzu,
        }]),
        win_method: WinMethod::Tumo,
        special_win: Default::default(),
    };

    let input = Input::new(pi_input, field, status);
    let result = input.calc_hand();
    if result.is_err() {
        panic!("invalid input");
    }
    let result = result.unwrap();
    println!("{:#?}", result);
}
