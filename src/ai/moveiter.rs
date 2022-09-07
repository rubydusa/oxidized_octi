use std::collections::HashSet;

use super::super::board::{Arrow, BoardEventProcessor, Boardable, OctiMove, Position};
use super::board::Board;

use super::super::global::ARROWS_PER_OCTI;

pub struct OctiMoveIterator<'a> {
    arr_octi_move_iterator: ArrOctiMoveIterator<'a>,
    mov_octi_move_iterator: MovOctiMoveIterator<'a>,
}

impl<'a> OctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> OctiMoveIterator {
        OctiMoveIterator {
            arr_octi_move_iterator: ArrOctiMoveIterator::new(board),
            mov_octi_move_iterator: MovOctiMoveIterator::new(board),
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

pub struct ArrOctiMoveIterator<'a> {
    board: &'a Board,
    pos: Position,
    positions: Vec<Position>,
    check_stack: Vec<Arrow>,
}

impl<'a> ArrOctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> ArrOctiMoveIterator<'a> {
        ArrOctiMoveIterator {
            board,
            pos: Position::default(),
            positions: board.octis().map(|x| x.pos()).collect(),
            check_stack: vec![],
        }
    }
}

impl<'a> Iterator for ArrOctiMoveIterator<'a> {
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

pub struct MovOctiMoveIterator<'a> {
    board: &'a Board,
    pos: Position,
    positions: Vec<Position>,
    check_stack: Vec<Vec<Arrow>>,
}

impl<'a> MovOctiMoveIterator<'a> {
    pub fn new(board: &'a Board) -> MovOctiMoveIterator<'a> {
        MovOctiMoveIterator {
            board,
            pos: Position::default(),
            positions: board.octis().map(|x| x.pos()).collect(),
            check_stack: vec![],
        }
    }
}

impl<'a> Iterator for MovOctiMoveIterator<'a> {
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
            if self.board.is_move_valid(consider) {
                // no possible illegal repetition for chain moves under 4 moves
                let len = chain.len();
                if len > 4 {
                    let mut already_checked = HashSet::<Board>::with_capacity(len);

                    for i in 1..=len {
                        let mut local_board = self.board.clone();
                        let consider = &OctiMove::Move(self.pos, chain[..i].to_vec());
                        local_board.make_move(consider);

                        if already_checked.contains(&local_board) {
                            continue 'main;
                        } else {
                            already_checked.insert(local_board);
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
