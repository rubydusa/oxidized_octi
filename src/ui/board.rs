use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;

use super::super::board::{Arrow, Board, Boardable, Octi, Position, Team};
use super::symbols;

const CELL_WIDTH: u16 = 11;
const CELL_HEIGHT: u16 = 6;

pub struct BoardUI<'a>(pub &'a Board);

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
            for y in y1..y2 {
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
                            "│  {} {} {}  ",
                            arrow_symbol(octi, 3, "╲", " "),
                            arrow_symbol(octi, 2, "▕▏", "  "),
                            arrow_symbol(octi, 1, "╱", " "),
                        )),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 2,
                        &Spans::from(vec![
                            Span::raw("│   "),
                            Span::styled("╱▔▔╲", team_style),
                            Span::raw("  "),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 3,
                        &Spans::from(vec![
                            Span::raw(format!("│ {}", arrow_symbol(octi, 4, "──", "  "))),
                            Span::styled("▏", team_style),
                            Span::raw(match team {
                                Team::Red => "╱╲",
                                Team::Green => "╲╱",
                            }),
                            Span::styled("▕", team_style),
                            Span::raw(format!("{} ", arrow_symbol(octi, 0, "──", "  "))),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_spans(
                        x_buf,
                        y_buf + 4,
                        &Spans::from(vec![
                            Span::raw("│   "),
                            Span::styled("╲▁▁╱", team_style),
                            Span::raw("  "),
                        ]),
                        CELL_WIDTH,
                    );

                    buf.set_span(
                        x_buf,
                        y_buf + 5,
                        &Span::raw(format!(
                            "│  {} {} {}  ",
                            arrow_symbol(octi, 5, "╱", " "),
                            arrow_symbol(octi, 6, "▕▏", "  "),
                            arrow_symbol(octi, 7, "╲", " "),
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
    }
}

fn arrow_symbol(
    octi: &Octi,
    arr: usize,
    if_exists: &'static str,
    if_not: &'static str,
) -> &'static str {
    if octi.has_arr(&Arrow::new(arr).unwrap()) {
        if_exists
    } else {
        if_not
    }
}

