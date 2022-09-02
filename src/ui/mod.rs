mod board;
mod layouts;
mod symbols;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Borders, Block, Paragraph},
    Frame, Terminal, style::{Style, Modifier, Color},
};

use super::game::{Action, Game};
use super::board::Board;

struct App {
    input: String,
    message: String,
    game: Game,
}

impl App {
    fn board_state(&self) -> &Board {
        self.game.state()
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            message: String::from("Press Esc to quit"),
            game: Game::default(),
        }
    }
}

fn game_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let mut app = App::default();

    loop {
        terminal.draw(|f| render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    if app.input.is_empty() {
                        continue;
                    }

                    let input: String = app.input.drain(..).collect();
                    let action = input.parse::<Action>();
                    match action {
                        Ok(action) => match app.game.process_action(action) {
                            Err(message) => { app.message = message; },
                            _ => { app.message.clear() }
                        },
                        Err(message) => { app.message = message; }
                    }
                },
                KeyCode::Char(c) => {
                    app.input.push(c);
                },
                KeyCode::Backspace => {
                    app.input.pop();
                },
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {},
            }
        }
    }
}

fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
    let board_ui = board::BoardUI::new(app.board_state());
    let layout = build_layout(f.size().width, &board_ui);

    f.render_widget(board_ui, layout[0]);

    let input = Paragraph::new(app.input.as_ref())
    .block(Block::default().borders(Borders::ALL).title("Move"));

    f.render_widget(input, layout[1]);
    
    let message = Paragraph::new(app.message.as_ref())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(message, layout[2]);
    f.set_cursor(
        layout[1].x + app.input.len() as u16 + 1,
        layout[1].y + 1,
    );
}

fn build_layout(width: u16, board_ui: &board::BoardUI) -> Vec<Rect> {
    let (board_width, board_height) = (board_ui.width(), board_ui.height());
    
    let x = (width - board_width) / 2;
    let mut vstack = layouts::VStackLayout::new(x, 0, board_width);

    vstack.push(board_height);
    vstack.margin(2);
    vstack.push(3);
    vstack.margin(1);
    vstack.push(2);
    vstack.layout()
}

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
