use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::{is_starting_position, winner};

use super::super::board::{BoardEventProcessor, Boardable, OctiMove};

use super::board::Board;

pub type Priority = u32;

pub fn get_contexts_sorted(
    board: &Board,
    octi_moves: Vec<OctiMove>,
    priority_eval_data: &PriorityEvalData,
) -> Vec<OctiMoveContext> {
    let mut contexts = octi_moves
        .into_iter()
        .map(|x| priority_eval(board, x, priority_eval_data))
        .collect::<Vec<_>>();

    contexts.sort();
    contexts.reverse();
    contexts
}

// assigns priority to moves (returns board after move in order to only calculate next_octis once)
fn priority_eval(
    board: &Board,
    octi_move: OctiMove,
    priority_eval_data: &PriorityEvalData,
) -> OctiMoveContext {
    let mut next_board = board.clone();
    next_board.make_move(&octi_move);

    let mut priority = u32::MIN;
    let game_winner = winner(&next_board);
    if game_winner.is_some() {
        let game_winner = game_winner.unwrap();
        if game_winner == board.turn() {
            priority = u32::MAX;
        } else {
            priority = u32::MIN;
        }
    } else {
        match octi_move {
            OctiMove::Arrow(pos, _) => {
                let octi = board.get_octi_by_pos(&pos).unwrap();
                let team = octi.team();

                if !is_starting_position(pos, team) {
                    priority += priority_eval_data.has_moved_value;
                }

                // prioritize giving arrows to octis that already have arrows
                priority += octi.arr_count() * priority_eval_data.arrow_value;
            }
            OctiMove::Move(pos, _) => {
                // movement is much more prioritized in branch seeking than arrow placements
                priority += priority_eval_data.mov_value;

                let octi = board.get_octi_by_pos(&pos).unwrap();
                let team = octi.team();

                if !is_starting_position(pos, team) {
                    priority += priority_eval_data.has_moved_value;
                }

                let enemy_octis_count_before = board.octis().filter(|x| x.team() != team).count() as u32;
                let enemy_octis_count_after = (&next_board).octis().filter(|x| x.team() != team).count() as u32;

                priority +=
                    (enemy_octis_count_before - enemy_octis_count_after) * priority_eval_data.kill_value;
            }
        }
    }

    OctiMoveContext {
        octi_move,
        board: next_board,
        priority,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriorityEvalData {
    has_moved_value: u32,
    mov_value: u32,
    kill_value: u32,
    arrow_value: u32,
}

// an octi move with context (board state)
#[derive(Clone)]
pub struct OctiMoveContext {
    octi_move: OctiMove,
    board: Board,
    priority: Priority,
}

impl PriorityEvalData {
    pub fn default() -> Result<PriorityEvalData, Box<dyn Error>> {
        let reader = BufReader::new(File::open("./src/ai/data/default_priority_eval_data.json")?);
        Ok(serde_json::from_reader(reader)?)
    }
}

impl OctiMoveContext {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn octi_move(self) -> OctiMove {
        self.octi_move
    }
}

impl PartialEq for OctiMoveContext {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl Eq for OctiMoveContext {}

impl PartialOrd for OctiMoveContext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for OctiMoveContext {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
