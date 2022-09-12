use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::super::board::{ArrowStatus, BoardEventProcessor, Boardable, OctiMove, Team};
use super::super::global::ARROWS_PER_OCTI;

use super::board::Board;
use super::matrix::Matrix;
use super::moveiter::new_move_octi_move_iterator;
use super::{team_index, winner, TEAMS};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Score(i32),
    Win(Team),
}

pub fn board_eval(board: &Board, eval_data: &EvalData) -> Value {
    let game_winner = winner(board);
    match game_winner {
        Some(game_winner) => return Value::Win(game_winner),
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
        let arrow_values = &eval_data.arrow_values[team_index(team)];

        for (i, arrow) in octi.arr_iter() {
            if *arrow == ArrowStatus::Active {
                *eval += arrow_values[i];
            }
        }

        let position_matricies = &eval_data.position_matricies[team_index(team)];
        *eval += position_matricies.get(&pos).unwrap();
    }

    let cur_team = board.turn();
    for team in [Team::Red, Team::Green] {
        let mut board = board.clone();
        board.set_turn(team);

        for octi_mov in new_move_octi_move_iterator(&board) {
            match &octi_mov {
                OctiMove::Move(pos, arrs) => {
                    let mut board_clone = board.clone();

                    let octi = board_clone.get_octi_by_pos(&pos).unwrap();
                    let team = octi.team();
                    let octi_id = octi.id();
                    let preivous_pos = octi.pos();

                    board_clone.make_move(&octi_mov);

                    let new_pos = board_clone.get_octi_by_id(&octi_id).unwrap().pos();

                    let eval = match team {
                        Team::Red => &mut red_eval,
                        Team::Green => &mut green_eval,
                    };
                    // if there is a winner
                    // check whether the winner is the original turn team from the original board state
                    // and also that this is a move done by the same team
                    // (opponent team theoretically can do a move which will result in the win of the other, but an optimal opponent won't)
                    if let Some(game_winner) = winner(&board_clone) {
                        if game_winner == cur_team && game_winner == team {
                            return Value::Win(game_winner);
                        }
                    }

                    let abs_dif = (new_pos - preivous_pos).abs();
                    let move_matrix = if abs_dif.x() <= 1 && abs_dif.y() <= 1 && arrs.len() == 1 {
                        &eval_data.simple_move_matricies[team_index(team)]
                    } else {
                        &eval_data.jump_move_matricies[team_index(team)]
                    };

                    *eval += move_matrix.get(&new_pos).unwrap();
                }
                _ => panic!("MoveOctiMoveIterator returned non mov octi move"),
            }
        }
    }

    let final_eval = red_eval - green_eval;
    Value::Score(final_eval)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EvalData {
    octi_value: i32,
    arrow_values: [[i32; ARROWS_PER_OCTI]; TEAMS],
    position_matricies: [Matrix<i32>; TEAMS],
    simple_move_matricies: [Matrix<i32>; TEAMS],
    jump_move_matricies: [Matrix<i32>; TEAMS],
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match self {
                Value::Win(team) => match team {
                    Team::Red => match other {
                        Value::Win(Team::Red) => Ordering::Equal,
                        _ => Ordering::Greater,
                    }
                    Team::Green => match other {
                        Value::Win(Team::Green) => Ordering::Equal,
                        _ => Ordering::Less,
                    },
                },
                Value::Score(score) => match other {
                    Value::Win(other_team) => match other_team {
                        Team::Red => Ordering::Less,
                        Team::Green => Ordering::Greater,
                    },
                    Value::Score(other_score) => score.cmp(other_score),
                }
            }
        )
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl EvalData {
    pub fn default() -> Result<EvalData, Box<dyn Error>> {
        let reader = BufReader::new(File::open("./src/ai/data/default_eval_data.json")?);
        Ok(serde_json::from_reader(reader)?)
    }
}
