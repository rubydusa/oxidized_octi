mod board;
mod symbols;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{self, Constraint, Direction, Layout, Rect},
    widgets::Block,
    Frame, Terminal,
};

use super::board::Board;

pub fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    game_loop(&mut terminal)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn game_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let board = Board::default();

    loop {
        terminal.draw(|f| render(f, &board))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn render<B: Backend>(f: &mut Frame<B>, board: &Board) {
    /*
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
                     Constraint::Length(1),
                     Constraint::Length(3),
        ].as_ref())
        .split(f.size());
    */

    // f.render_widget(board::draw_board(board), f.size());
    f.render_widget(board::BoardUI(board), f.size())
}
