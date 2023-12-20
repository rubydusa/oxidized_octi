use super::super::board::{Arrow, BoardEventProcessor, Boardable, OctiMove, Position};
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
    check_stack: Vec<Vec<(Arrow, bool)>>,
}

pub fn new_octi_move_iterator(
    board: &Board,
) -> OctiMoveIterator<impl Iterator<Item = Position> + '_, impl Iterator<Item = Position> + '_> {
    OctiMoveIterator {
        arr_octi_move_iterator: new_arrow_octi_move_iterator(board),
        move_octi_move_iterator: new_move_octi_move_iterator(board),
    }
}

pub fn new_arrow_octi_move_iterator(
    board: &Board,
) -> ArrowOctiMoveIterator<impl Iterator<Item = Position> + '_> {
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

pub fn new_move_octi_move_iterator(
    board: &Board,
) -> MoveOctiMoveIterator<impl Iterator<Item = Position> + '_> {
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
            if self.check_stack.is_empty() {
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
            if self.check_stack.is_empty() {
                self.pos = self.positions.next()?;
                self.check_stack = (0..ARROWS_PER_OCTI)
                    .map(|i| {
                        let arr = Arrow::new(i).unwrap();
                        vec![
                            vec![(arr, false)],
                            vec![(arr, true)]
                        ]
                     })
                    .flatten()
                    .collect::<Vec<_>>();
            }

            let chain = self.check_stack.pop().unwrap();

            let consider = OctiMove::Move(self.pos, chain.clone());
            if let Err(_) = self.board.move_events(&consider) {
                continue 'main;
            }

            for i in 0..ARROWS_PER_OCTI {
                let mut c = chain.clone();
                c.push((Arrow::new(i).unwrap(), false));
                self.check_stack.push(c);
                let mut c = chain.clone();
                c.push((Arrow::new(i).unwrap(), true));
                self.check_stack.push(c);
            }
            return Some(consider);
        }
    }
}
