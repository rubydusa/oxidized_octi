mod parse;

use std::error::Error;

use super::ai::board;
use super::ai::minimax;
use super::board::{Board, BoardEventProcessor, OctiMove};

// Aliases

type GameHistory = Vec<OctiMove>;

// Enums

pub enum Action {
    Start,
    End,
    Forward(usize),
    Backward(usize),
    OctiMove(OctiMove),
    AI(u32),
    Ovewrite
}

// Structs

pub struct Game {
    state: Board,
    start: Board,
    history: GameHistory,
    cursor: usize,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            state: board.clone(),
            start: board,
            history: Vec::new(),
            cursor: 0,
        }
    }

    // Getters

    pub fn state(&self) -> &Board {
        &self.state
    }

    pub fn start(&self) -> &Board {
        &self.start
    }

    pub fn history(&self) -> &GameHistory {
        &self.history
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    // Operations

    pub fn make_move(&mut self, octi_move: OctiMove) -> Result<(), String> {
        if self.cursor < self.history.len() {
            return Err(String::from("Cannot make a move when cursor not on head"));
        };

        self.state.make_move(&octi_move)?;

        self.history.push(octi_move);
        self.cursor += 1;

        Ok(())
    }

    pub fn ai(&mut self, depth: u32) -> Result<(), Box<dyn Error>> {
        let board = board::Board::new(self.state());
        let octi_move = minimax(&board, depth)?
            .octi_move()
            .ok_or("No possible moves from possition")?;
        self.make_move(octi_move)?;
        Ok(())
    }

    pub fn process_action(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Start => {
                self.cursor_start();
                Ok(())
            }
            Action::End => {
                self.cursor_end();
                Ok(())
            }
            Action::Forward(steps) => {
                self.move_cursor_forward(steps);
                Ok(())
            }
            Action::Backward(steps) => {
                self.move_cursor_backwords(steps);
                Ok(())
            }
            Action::OctiMove(octi_move) => self.make_move(octi_move),
            Action::AI(depth) => self.ai(depth).map_err(|e| e.to_string()),
            Action::Ovewrite => {
                self.overwrite_history();
                Ok(())
            }
        }
    }

    pub fn overwrite_history(&mut self) {
        self.history.truncate(self.cursor);
    }

    pub fn move_cursor_forward(&mut self, by: usize) {
        let new_cursor = std::cmp::min(self.cursor + by, self.history.len());
        for i in self.cursor..new_cursor {
            self.state.make_move(&self.history[i]).unwrap();
        }

        self.cursor = new_cursor;
    }

    pub fn move_cursor_backwords(&mut self, by: usize) {
        let new_cursor = std::cmp::max(self.cursor.saturating_sub(by), 0);

        self.state = self.start.clone();
        for i in 0..new_cursor {
            self.state.make_move(&self.history[i]).unwrap();
        }

        self.cursor = new_cursor;
    }

    pub fn cursor_end(&mut self) {
        self.move_cursor_forward(self.history.len());
    }

    pub fn cursor_start(&mut self) {
        self.move_cursor_backwords(self.history.len());
    }

    pub fn set_cursor(&mut self, to: usize) {
        self.cursor_start();
        self.move_cursor_forward(to)
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new(Board::default())
    }
}
