pub mod board;

mod eval;
mod matrix;
mod moveiter;
mod priority;

use std::cmp::Ordering;
use std::error::Error;

use super::board::{Boardable, OctiMove, Position, Team};

use board::Board;
use eval::{board_eval, EvalData, Value};
use moveiter::new_octi_move_iterator;
use priority::{get_contexts_sorted, PriorityEvalData};

const BOARD_WIDTH: usize = 6;
const BOARD_HEIGHT: usize = 7;
const TEAMS: usize = 2;
const RED_INDEX: usize = 0;
const GREEN_INDEX: usize = 1;

pub fn minimax(board: &Board, depth: u32) -> Result<MinimaxResult, Box<dyn Error>> {
    if depth == 0 {
        Err("Minimax cannot be depth 0")?;
    }

    let eval_data = EvalData::default()?;
    let priority_eval_data = PriorityEvalData::default()?;
    let result = _minimax(
        board,
        0,
        depth,
        BoardScore(Value::Win(Team::Green), u32::MAX),
        BoardScore(Value::Win(Team::Red), u32::MAX),
        &eval_data,
        &priority_eval_data,
    );

    Ok(result)
}

fn _minimax(
    board: &Board,
    depth: u32,
    target_depth: u32,
    alpha: BoardScore,
    beta: BoardScore,
    eval_data: &EvalData,
    priority_eval_data: &PriorityEvalData,
) -> MinimaxResult {
    if depth == target_depth || winner(board).is_some() {
        return MinimaxResult(BoardScore(board_eval(board, eval_data), depth), None);
    }

    let (mut alpha, mut beta) = (alpha, beta);
    let turn = board.turn();
    let mut value = match turn {
        Team::Red => BoardScore(Value::Win(Team::Green), u32::MAX),
        Team::Green => BoardScore(Value::Win(Team::Red), u32::MAX),
    };
    let mut value_move = None;

    let all_contexts = get_contexts_sorted(
        board,
        new_octi_move_iterator(board).collect(),
        priority_eval_data,
    );

    for context in all_contexts {
        // don't return none in case all moves are absolute worse
        if value_move.is_none() {
            value_move = Some(context.clone().octi_move());
        }
        // destructure prioritized
        let result = _minimax(
            context.board(),
            depth + 1,
            target_depth,
            alpha,
            beta,
            eval_data,
            priority_eval_data,
        );

        let eval = result.score();

        match turn {
            Team::Red => {
                if eval > value || eval.same_lower_depth(&value) {
                    value = eval;
                    value_move = Some(context.octi_move());
                }
                if value > alpha || value.same_lower_depth(&alpha) {
                    alpha = value;
                }
                if value >= beta {
                    break;
                }
            }
            Team::Green => {
                if eval < value || eval.same_lower_depth(&value) {
                    value = eval;
                    value_move = Some(context.octi_move());
                }
                if value < beta || value.same_lower_depth(&beta) {
                    beta = value;
                }
                if value <= alpha {
                    break;
                }
            }
        }
    }

    MinimaxResult(value, value_move)
}

fn winner(board: &Board) -> Option<Team> {
    let mut saw_red = false;
    let mut saw_green = false;

    for octi in board.octis() {
        let pos = octi.pos();
        let team = octi.team();
        match team {
            Team::Red => {
                if is_starting_position(pos, Team::Green) {
                    return Some(Team::Red);
                }
                saw_red = true;
            }
            Team::Green => {
                if is_starting_position(pos, Team::Red) {
                    return Some(Team::Green);
                }
                saw_green = true;
            }
        }
    }

    if saw_red && !saw_green {
        Some(Team::Red)
    } else if !saw_red && saw_green {
        Some(Team::Green)
    } else {
        None
    }
}

// helper functions for submodules
fn is_starting_position(pos: Position, team: Team) -> bool {
    match team {
        Team::Red => pos.y() == 5 && pos.x() > 0 && pos.x() < 5,
        Team::Green => pos.y() == 1 && pos.x() > 0 && pos.x() < 5,
    }
}

fn team_index(team: Team) -> usize {
    match team {
        Team::Red => RED_INDEX,
        Team::Green => GREEN_INDEX,
    }
}

/**
 * BoardScore exists so if two board states have the same evalutaion,
 * it will prioritize the board state that takes more moves to reach
 *
 * This is in order to make the ai to "not give up" if it realizes it loses anyways
 *
 * e.g. maximizing player is making a turn
 *
 * if there are two board states in the tree that are a win to the minimizing player,
 * it will make the move that results in a lose in more turns
 *
 * if there are two board states in the tree that are a win to the maximizng player,
 * it will still make the move that results in the game ending in more turns, but is still guaranteed a win
 *
 * theoretical edge case is if there is a board evaluation which is exactly the integer max limit or the integer min limit
 * and is not a win to either sides
 *
 * this would actually worsen the evaluation process but if there is a board evaluation of a non-terminal position that is equal
 * to the evaluation of a terminal position it should be considerd a bug
 *
 */

#[derive(Clone)]
pub struct MinimaxResult(BoardScore, Option<OctiMove>);

// first is eval value
// second is depth

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoardScore(Value, u32);

impl MinimaxResult {
    pub fn score(&self) -> BoardScore {
        self.0
    }

    pub fn octi_move(self) -> Option<OctiMove> {
        self.1
    }
}

impl BoardScore {
    pub fn value(&self) -> Value {
        self.0
    }

    pub fn depth(&self) -> u32 {
        self.1
    }

    fn same_lower_depth(&self, other: &BoardScore) -> bool {
        self.0 == other.0 && other.1 < self.1
    }
}

impl PartialOrd for BoardScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for BoardScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
