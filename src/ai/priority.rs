use std::collections::BinaryHeap;

use super::{is_starting_position, winner};

use super::super::board::{BoardEventProcessor, Boardable, OctiMove};

use super::board::Board;

pub type Priority = u32;

// assigns priority to moves (returns board after move in order to only calculate next_octis once)
pub fn priority_eval(
    board: &Board,
    octi_move: &OctiMove,
    priority_eval_data: &PriorityEvalData,
) -> (Board, u32) {
    let mut next_board = board.clone();
    next_board.make_move(octi_move);

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
            OctiMove::Arr(pos, _) => {
                let pos = *pos;

                let octi = board.get(pos).unwrap();
                let team = octi.team();

                if !is_starting_position(pos, team) {
                    priority += priority_eval_data.has_moved_value;
                }

                // prioritize giving arrows to octis that already have arrows
                priority += octi.count_arrows() * priority_eval_data.arrow_value;
            }
            OctiMove::Mov(pos, _) => {
                let pos = *pos;
                // movement is much more prioritized in branch seeking than arrow placements
                priority += priority_eval_data.mov_value;

                let octi = board.get(pos).unwrap();
                let team = octi.team();

                if !is_starting_position(pos, team) {
                    priority += priority_eval_data.has_moved_value;
                }

                let octis_count_before = board.into_iter().count() as u32;
                let octis_count_after = (&next_board).into_iter().count() as u32;

                priority +=
                    (octis_count_before - octis_count_after) * priority_eval_data.kill_value;
            }
        }
    }

    (next_board, priority)
}

pub fn get_contexts_sorted(
    board: &Board,
    octi_moves: Vec<OctiMove>,
    priority_eval_data: &PriorityEvalData,
) -> BinaryHeap<OctiMoveContextPrioritized> {
    let mut heap = BinaryHeap::new();
    for octi_move in octi_moves.into_iter() {
        let (board, priority) = priority_eval(board, &octi_move, priority_eval_data);

        heap.push(OctiMoveContextPrioritized {
            priority,
            context: OctiMoveContext { octi_move, board },
        });
    }
    heap
}

#[derive(Clone)]
pub struct PriorityEvalData {
    has_moved_value: u32,
    mov_value: u32,
    kill_value: u32,
    arrow_value: u32,
}

// an octi move with context (board state)
pub struct OctiMoveContext {
    pub octi_move: OctiMove,
    pub board: Board, // octis AFTER octi_move
}

pub struct OctiMoveContextPrioritized {
    pub priority: Priority,
    pub context: OctiMoveContext,
}

impl PartialEq for OctiMoveContextPrioritized {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl Eq for OctiMoveContextPrioritized {}

impl PartialOrd for OctiMoveContextPrioritized {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for OctiMoveContextPrioritized {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
