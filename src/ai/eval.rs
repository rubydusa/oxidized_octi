use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::super::board::{ArrowStatus, Boardable, Team};
use super::super::global::ARROWS_PER_OCTI;

use super::board::Board;
use super::matrix::Matrix;
use super::{team_win_value, winner, TEAMS};

pub type Value = i32;

pub fn board_eval(board: &Board, eval_data: &EvalData) -> Value {
    let game_winner = winner(board);
    match game_winner {
        Some(game_winner) => return team_win_value(game_winner),
        None => (),
    }

    let mut red_eval = 0;
    let mut green_eval = 0;

    // add arrow values, intrinsic octi values and position values
    for octi in board.octis() {
        let pos = octi.pos();
        let team = octi.team();

        let eval = match team {
            Team::Red => &mut red_eval,
            Team::Green => &mut green_eval,
        };

        *eval += eval_data.octi_value;
        let arrow_values = &eval_data.arrow_values[team as usize];

        for (i, arrow) in octi.arr_iter() {
            if *arrow == ArrowStatus::Active {
                *eval += arrow_values[i];
            }
        }

        let position_matricies = &eval_data.position_matricies[team as usize];
        *eval += position_matricies.get(&pos).unwrap();
    }

    let cur_team = board.turn();
    for team in [Team::Red, Team::Green] {
        let mut board = board.clone();
        board.set_turn(team);

        for octi_mov in MovOctiMoveIterator::new(&board) {
            let mut board_clone = board.clone();
            let new_pos = board_clone.make_move(&octi_mov);

            match octi_mov {
                OctiMove::Mov(pos, arrs) => {
                    let octi = board.get(pos).unwrap();
                    let team = octi.team();
                    let eval = match team {
                        Team::Red => &mut red_eval,
                        Team::Green => &mut green_eval,
                    };
                    // if there is a winner
                    // check whether the winner is the original turn team from the original board state
                    // and also that this is a move done by the same team
                    // (opponent team theoretically can do a move which will result in the win of the other, but an optimal opponent won't)
                    let game_winner = winner(&board_clone);
                    if game_winner.is_some() {
                        let game_winner = game_winner.unwrap();
                        if game_winner == cur_team && game_winner == team {
                            return team_win_value(game_winner);
                        }
                    }

                    let (x, y) = pos;
                    let (nx, ny) = new_pos;
                    let (adx, ady) = ((x - nx).abs(), (y - ny).abs()); // absolute difference x, absolute difference y
                                                                       // simple move
                    if adx <= 1 && ady <= 1 && arrs.len() == 1 {
                        let simple_move_matricies = &eval_data.simple_move_matricies[team as usize];
                        *eval += simple_move_matricies.get(nx as usize, ny as usize).unwrap();
                    } else {
                        let jump_move_matricies = &eval_data.jump_move_matricies[team as usize];
                        *eval += jump_move_matricies.get(nx as usize, ny as usize).unwrap();
                    }
                }
                _ => panic!("MovOctiMoveIterator returned non mov octi move"),
            }
        }
    }

    let final_eval = red_eval - green_eval;
    if final_eval == i32::MAX || final_eval == i32::MIN {
        panic!("board evaluation that results in an integer limit");
    } else {
        final_eval
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EvalData {
    octi_value: Value,
    arrow_values: [[Value; ARROWS_PER_OCTI]; TEAMS],
    position_matricies: [Matrix<Value>; TEAMS],
    simple_move_matricies: [Matrix<Value>; TEAMS],
    jump_move_matricies: [Matrix<Value>; TEAMS],
}

impl EvalData {
    pub fn default() -> Result<EvalData, Box<dyn Error>> {
        let reader = BufReader::new(File::open("./src/ai/data/default_eval_data.json")?);
        Ok(serde_json::from_reader(reader)?)
    }
}
