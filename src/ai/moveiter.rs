use std::collections::HashSet;

use super::super::board::{Arrow, BoardEvent, BoardEventProcessor, Boardable, OctiMove, Position};
use super::board::Board;

use super::super::global::ARROWS_PER_OCTI;

pub struct OctiMoveIterator<'a, T: Iterator<Item = Position>, U: Iterator<Item = Position>> {
    arr_octi_move_iterator: ArrowOctiMoveIterator<'a, T>,
    move_octi_move_iterator: MoveOctiMoveIterator<'a, U>,
}

pub struct ArrowOctiMoveIterator<'a, T: Iterator<Item = Position>> {
    board: &'a Board,
    pos: Position,
    positions: T,
    check_stack: Vec<Arrow>,
}

pub struct MoveOctiMoveIterator<'a, T: Iterator<Item = Position>> {
    board: &'a Board,
    pos: Position,
    positions: T,
    check_stack: Vec<Vec<Arrow>>,
}

pub fn new_octi_move_iterator<'a>(
    board: &'a Board,
) -> OctiMoveIterator<'a, impl Iterator<Item = Position> + 'a, impl Iterator<Item = Position> + 'a>
{
    OctiMoveIterator {
        arr_octi_move_iterator: new_arrow_octi_move_iterator(board),
        move_octi_move_iterator: new_move_octi_move_iterator(board),
    }
}

pub fn new_arrow_octi_move_iterator<'a>(
    board: &'a Board,
) -> ArrowOctiMoveIterator<'a, impl Iterator<Item = Position> + 'a> {
    let turn = board.turn();
    ArrowOctiMoveIterator {
        board,
        pos: Position::default(),
        positions: board
            .octis()
            .filter(move |x| x.team() == turn)
            .map(|x| x.pos()),
        check_stack: vec![],
    }
}

pub fn new_move_octi_move_iterator<'a>(
    board: &'a Board,
) -> MoveOctiMoveIterator<'a, impl Iterator<Item = Position> + 'a> {
    let turn = board.turn();

    MoveOctiMoveIterator {
        board,
        pos: Position::default(),
        positions: board
            .octis()
            .filter(move |x| x.team() == turn)
            .map(|x| x.pos()),
        check_stack: vec![],
    }
}

impl<'a, T: Iterator<Item = Position>, U: Iterator<Item = Position>> Iterator
    for OctiMoveIterator<'a, T, U>
{
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        self.arr_octi_move_iterator
            .next()
            .or_else(|| self.move_octi_move_iterator.next())
    }
}

impl<'a, T: Iterator<Item = Position>> Iterator for ArrowOctiMoveIterator<'a, T> {
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        loop {
            if self.check_stack.len() == 0 {
                self.pos = self.positions.next()?;
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

impl<'a, T: Iterator<Item = Position>> Iterator for MoveOctiMoveIterator<'a, T> {
    type Item = OctiMove;

    fn next(&mut self) -> Option<OctiMove> {
        'main: loop {
            if self.check_stack.len() == 0 {
                self.pos = self.positions.next()?;
                self.check_stack = (0..ARROWS_PER_OCTI)
                    .map(|i| vec![Arrow::new(i).unwrap()])
                    .collect::<Vec<_>>();
            }

            let chain = self.check_stack.pop().unwrap();

            let consider = OctiMove::Move(self.pos, chain.clone());
            if let Ok(move_events) = self.board.move_events(&consider) {
                let len = chain.len();
                // len <= 3 impossible repetition
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
            return Some(consider);
        }
    }
}
