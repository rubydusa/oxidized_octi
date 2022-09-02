use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;

use super::super::board::{Board, Boardable, Position, Team};
use super::symbols;

const CELL_WIDTH: u16 = 11;
const CELL_HEIGHT: u16 = 6;

pub struct BoardUI<'a>(&'a Board);

impl<'a> BoardUI<'a> {
    pub fn new(board: &'a Board) -> BoardUI<'a> {
        BoardUI(board)
    }

    pub fn height(&self) -> u16 {
        self.0.bounds().height() as u16 * CELL_HEIGHT + 1
    }

    pub fn width(&self) -> u16 {
        self.0.bounds().width() as u16 * CELL_WIDTH + 1
    }
}

impl<'a> Widget for BoardUI<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let bounds = self.0.bounds();
        let (lu, rd) = (bounds.lu(), bounds.rd());
        let (x1, x2, y1, y2) = (lu.x(), rd.x(), lu.y(), rd.y());
        // normalize
        let (x1, x2, y1, y2) = (x1 - x1, x2 - x1, y1 - y1, y2 - y1);

        let horizontal_line = symbols::HORIZONTAL_LINE.repeat((CELL_WIDTH - 1) as usize);

        let right_down_edge_line_span =
            Span::raw([symbols::RIGHT_DOWN_EDGE, &horizontal_line].concat());
        let left_border_line_span = Span::raw([symbols::LEFT_BORDER, &horizontal_line].concat());
        let up_border_line_span = Span::raw([symbols::UP_BORDER, &horizontal_line].concat());
        let intersection_line_span = Span::raw([symbols::INTERSECTION, &horizontal_line].concat());

        let empty_line = Span::raw(
            [
                symbols::VERTICAL_LINE,
                &" ".repeat((CELL_WIDTH - 1) as usize),
            ]
            .concat(),
        );

        for x in x1..=x2 {
            for y in y1..=y2 {
                let x_buf = x as u16 * CELL_WIDTH + area.x;
                let y_buf = y as u16 * CELL_HEIGHT + area.y;

                let top = {
                    if x == 0 && y == 0 {
                        &right_down_edge_line_span
                    } else if x == 0 && y != 0 {
                        &left_border_line_span
                    } else if x != 0 && y == 0 {
                        &up_border_line_span
                    } else {
                        &intersection_line_span
                    }
                };

                buf.set_span(x_buf, y_buf, top, CELL_WIDTH);

                if let Some(octi) = self.0.get_octi_by_pos(&Position::new(x, y)) {
                    let team = octi.team();
                    let team_style = match team {
                        Team::Red => Style::default().fg(Color::Red),
                        Team::Green => Style::default().fg(Color::Green),
                    };

                    buf.set_span(
                        x_buf,
                        y_buf + 1,
                        &Span::raw(format!(
                            "{}  {} {} {}  ",
                            symbols::VERTICAL_LINE,
                            symbols::arrow_symbol(octi, 3),
                            symbols::arrow_symbol(octi, 2),
                            symbols::arrow_symbol(octi, 1),
                        )),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 2,
                        &Spans::from(vec![
                            Span::raw([symbols::VERTICAL_LINE, "   "].concat()),
                            Span::styled(symbols::OCTI_TOP, team_style),
                            Span::raw("  "),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 3,
                        &Spans::from(vec![
                            Span::raw(format!(
                                "{} {}",
                                symbols::VERTICAL_LINE,
                                symbols::arrow_symbol(octi, 4)
                            )),
                            Span::styled(symbols::OCTI_LEFT_SIDE, team_style),
                            Span::raw(match team {
                                Team::Red => symbols::RED_OCTI_ARROW,
                                Team::Green => symbols::GREEN_OCTI_ARROW,
                            }),
                            Span::styled(symbols::OCTI_RIGHT_SIDE, team_style),
                            Span::raw(format!("{} ", symbols::arrow_symbol(octi, 0))),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 4,
                        &Spans::from(vec![
                            Span::raw([symbols::VERTICAL_LINE, "   "].concat()),
                            Span::styled(symbols::OCTI_BOTTOM, team_style),
                            Span::raw("  "),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_span(
                        x_buf,
                        y_buf + 5,
                        &Span::raw(format!(
                            "│  {} {} {}  ",
                            symbols::arrow_symbol(octi, 5),
                            symbols::arrow_symbol(octi, 6),
                            symbols::arrow_symbol(octi, 7),
                        )),
                        CELL_WIDTH,
                    );
                } else {
                    for oy in 1..=5 {
                        buf.set_span(x_buf, y_buf + oy, &empty_line, CELL_WIDTH);
                    }
                }
            }
        }

        let (min_x, min_y) = (area.x, area.y);
        let (max_x, max_y) = (
            (x2 + 1) as u16 * CELL_WIDTH + min_x,
            (y2 + 1) as u16 * CELL_HEIGHT + min_y,
        );

        for x in min_x..max_x {
            let symbol = {
                if x == min_x {
                    symbols::RIGHT_UP_EDGE
                } else if (x - min_x) % CELL_WIDTH == 0 {
                    symbols::DOWN_BORDER
                } else {
                    symbols::HORIZONTAL_LINE
                }
            };
            buf.get_mut(x, max_y).set_symbol(symbol);
        }
        for y in min_y..max_y {
            let symbol = {
                if y == min_y {
                    symbols::LEFT_DOWN_EDGE
                } else if (y - min_y) % CELL_HEIGHT == 0 {
                    symbols::RIGHT_BORDER
                } else {
                    symbols::VERTICAL_LINE
                }
            };
            buf.get_mut(max_x, y).set_symbol(symbol);
        }
        buf.get_mut(max_x, max_y).set_symbol(symbols::LEFT_UP_EDGE);
    }
} 
