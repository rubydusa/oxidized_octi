mod board;
mod eval;
mod matrix;
mod moveiter;
mod priority;

use std::cmp::Ordering;
use std::error::Error;

use super::board::{OctiMove, Position, Team};

use board::Board;
use eval::{board_eval, EvalData, Value};
use priority::{get_contexts_sorted, Priority, PriorityEvalData};

const BOARD_WIDTH: usize = 6;
const BOARD_HEIGHT: usize = 7;
const TEAMS: usize = 2;

pub fn minimax(board: &Board, depth: u32) -> Result<MinimaxResult, Box<dyn Error>> {
    if depth == 0 {
        Err("Minimax cannot be depth 0")?;
    }

    let eval_data = EvalData::default()?;
    let priority_eval_data = PriorityEvalData::default()?;
    let res = _minimax(
        board,
        depth,
        BoardScore(i32::MIN, u32::MAX),
        BoardScore(i32::MAX, u32::MAX),
        &eval_data,
        &priority_eval_data,
    );

    if octi_move.is_none() {
        Err("No possible moves from current state")?;
    }

    Ok(octi_move.unwrap())
}

fn _minimax(
    board: &Board,
    depth: u32,
    alpha: BoardScore,
    beta: BoardScore,
    eval_data: &EvalData,
    priority_eval_data: &PriorityEvalData,
) -> MinimaxResult {
    if depth == 0 || winner(board).is_some() {
        return (BoardScore(board_eval(board, eval_data), depth), None);
    }

    let (mut alpha, mut beta) = (alpha, beta);
    let turn = board.turn();
    let mut value = match turn {
        Team::Red => BoardScore(i32::MIN, u32::MAX),
        Team::Green => BoardScore(i32::MAX, u32::MAX),
    };
    let mut value_move = None;

    let all_contexts = get_contexts_sorted(
        board,
        OctiMoveIterator::new(board).collect(),
        priority_eval_data,
    );

    for context in all_contexts {
        // destructure prioritized
        let context = context.context;
        if value_move.is_none() {
            value_move = Some(context.octi_move.clone());
        }

        let res = _minimax(
            &context.board,
            depth - 1,
            alpha,
            beta,
            eval_data,
            priority_eval_data,
        );
        match turn {
            Team::Red => {
                if eval > value || eval.higher_priority(&value) {
                    value = eval;
                    value_move = Some(context.octi_move);
                }
                if value > alpha || value.higher_priority(&alpha) {
                    alpha = value;
                }
                if value >= beta {
                    break;
                }
            }
            Team::Green => {
                if eval < value || eval.higher_priority(&value) {
                    value = eval;
                    value_move = Some(context.octi_move);
                }
                if value < beta || value.higher_priority(&beta) {
                    beta = value;
                }
                if value <= alpha {
                    break;
                }
            }
        }
    }

    MinimaxResult::new(value, value_move)
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

fn team_win_value(team: Team) -> i32 {
    match team {
        Team::Red => i32::MAX,
        Team::Green => i32::MIN,
    }
}

fn team_index(team: Team) -> usize {
    match team {
        Team::Red => 0,
        Team::Green => 1,
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
struct MinimaxResult(BoardScore, Option<OctiMove>);

// first is eval value
// second is depth (reversed - lower depth means it reaches this score in more moves)

#[derive(Copy, Clone, Eq, PartialEq)]
struct BoardScore(Value, Priority);

impl MinimaxResult {
    fn new(board_score: BoardScore, octi_move: Option<OctiMove>) -> MinimaxResult {
        MinimaxResult(board_score, octi_move)
    }

    fn score(&self) -> BoardScore {
        self.0
    }

    fn octi_move(&self) -> Option<OctiMove> {
        self.1
    }
}

impl BoardScore {
    fn new(value: Value, prioirty: Priority) -> BoardScore {
        BoardScore(value, prioirty)
    }

    fn value(&self) -> Value {
        self.0
    }

    fn prioirty(&self) -> Priority {
        self.1
    }

    fn higher_priority(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 < other.1
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
