use std::io;

use crate::board::{Arrow, BoardEventProcessor, OctiMove, Position};

mod ai;
mod board;
mod game;
mod global;
mod ui;

fn main() -> Result<(), io::Error> {
    ui::run()
    //run();
    //Ok(())
}

fn run() {
    let mut board = ai::board::Board::new(&board::Board::default());
    board.make_move(&OctiMove::Arrow(
        Position::new(1, 5),
        Arrow::new(2).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(1, 1),
        Arrow::new(6).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(2, 5),
        Arrow::new(4).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(2, 1),
        Arrow::new(4).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(2, 5),
        Arrow::new(0).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(3, 1),
        Arrow::new(6).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(3, 5),
        Arrow::new(2).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(4, 1),
        Arrow::new(6).unwrap(),
    ));
    board.make_move(&OctiMove::Arrow(
        Position::new(4, 5),
        Arrow::new(2).unwrap(),
    ));

    ai::minimax(&board, 4);
    println!("blab");
}
