use std::collections::HashSet;

use super::super::board::{Arrow, BoardEvent, BoardEventProcessor, OctiMove, Position};
use super::board::Board;

use super::super::global::ARROWS_PER_OCTI;

pub struct OctiMoveIterator<'a> {
    arr_octi_move_iterator: ArrowOctiMoveIterator<'a>,
    mov_octi_move_iterator: MoveOctiMoveIterator<'a>,
}

impl<'a> OctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> OctiMoveIterator {
        OctiMoveIterator {
            arr_octi_move_iterator: ArrowOctiMoveIterator::new(board),
            mov_octi_move_iterator: MoveOctiMoveIterator::new(board),
        }
    }
}

impl<'a> Iterator for OctiMoveIterator<'a> {
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        let mut octi_move = self.mov_octi_move_iterator.next();
        if octi_move.is_none() {
            octi_move = self.arr_octi_move_iterator.next();
        }
        octi_move
    }
}

pub struct ArrowOctiMoveIterator<'a> {
    board: &'a Board,
    pos: Position,
    positions: Vec<Position>,
    check_stack: Vec<Arrow>,
}

impl<'a> ArrowOctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> ArrowOctiMoveIterator<'a> {
        ArrowOctiMoveIterator {
            board,
            pos: Position::default(),
            positions: board.octis().map(|x| x.pos()).collect(),
            check_stack: vec![],
        }
    }
}

impl<'a> Iterator for ArrowOctiMoveIterator<'a> {
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        loop {
            if self.check_stack.len() == 0 {
                self.pos = self.positions.pop()?;
                self.check_stack = (0..ARROWS_PER_OCTI)
                    .map(|x| Arrow::new(x).unwrap())
                    .collect::<Vec<_>>();
            }
            let arr = self.check_stack.pop();

            let octi_move = OctiMove::Arrow(self.pos, arr.unwrap());
            if self.board.is_move_valid(&octi_move) {
                return Some(octi_move);
            }
        }
    }
}

pub struct MoveOctiMoveIterator<'a> {
    board: &'a Board,
    pos: Position,
    positions: Vec<Position>,
    check_stack: Vec<Vec<Arrow>>,
}

impl<'a> MoveOctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> MoveOctiMoveIterator<'a> {
        MoveOctiMoveIterator {
            board,
            pos: Position::default(),
            positions: board.octis().map(|x| x.pos()).collect(),
            check_stack: vec![],
        }
    }
}

impl<'a> Iterator for MoveOctiMoveIterator<'a> {
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        'main: loop {
            if self.check_stack.len() == 0 {
                self.pos = self.positions.pop()?;
                self.check_stack = (0..ARROWS_PER_OCTI)
                    .map(|i| vec![Arrow::new(i).unwrap()])
                    .collect::<Vec<_>>();
            }

            let chain = self.check_stack.pop().unwrap();

            let consider = &OctiMove::Move(self.pos, chain);
            if let Ok(move_events) = self.board.move_events(consider) {
                let len = chain.len();
                if len > 4 {
                    let mut already_checked = HashSet::<Board>::with_capacity(len);
                    let mut local_board = self.board.clone();

                    for events in move_events.split(|x| matches!(x, BoardEvent::Div)) {
                        local_board.process_events(events);
                        if already_checked.contains(&local_board) {
                            continue 'main;
                        } else {
                            already_checked.insert(local_board.clone());
                        }
                    }
                }
            } else {
                continue 'main;
            }

            for i in 0..ARROWS_PER_OCTI {
                let mut c = chain.clone();
                c.push(Arrow::new(i).unwrap());
                self.check_stack.push(c);
            }
            return Some(OctiMove::Move(self.pos, chain));
        }
    }
}
