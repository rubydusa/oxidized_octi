use std::io;

mod board;
mod game;
mod global;
mod ui;

fn main() -> Result<(), io::Error> {
    ui::run()
}
