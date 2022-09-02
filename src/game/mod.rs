mod parse;

use super::board::Board;
use super::board::BoardEventProcessor;
use super::board::OctiMove;

// Aliases

type GameHistory = Vec<OctiMove>;

// Enums

pub enum Action {
    Start,
    End,
    Forward(usize),
    Backward(usize),
    OctiMove(OctiMove),
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
        }
    }

    pub fn overwrite_history(&mut self) {
        self.history.truncate(self.cursor);
    }

    pub fn move_cursor_forward(&mut self, by: usize) {
        let new_cursor = std::cmp::min(self.cursor + by, self.history.len());
        for i in self.cursor..new_cursor {
            self.state.make_move(&self.history[i]);
        }

        self.cursor = new_cursor;
    }

    pub fn move_cursor_backwords(&mut self, by: usize) {
        let new_cursor = std::cmp::max(self.cursor.saturating_sub(by), 0);

        self.state = self.start.clone();
        for i in 0..new_cursor {
            self.state.make_move(&self.history[i]);
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
