use std::cmp::min;

use lifegame_core::{CELL_ALIVE, CELL_DEAD};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Cell, Row, Table, Widget},
    Frame,
};

use crate::app::{App, AppState};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(1), Constraint::Min(1)])
        .split(frame.size());
    let (description, world) = (layout[0], layout[1]);
    frame.render_widget(
        Block::default().title(format!(
            "Lifegame (gen={}) {}{}[<q>: quit]",
            app.gen,
            if app.state == AppState::Pause {
                "[<s>: start] "
            } else {
                "[<s>: pause] "
            },
            if app.can_reset() {
                "[<left><up><down><right>: pan] [<r>: reset] "
            } else {
                ""
            }
        )),
        description,
    );
    frame.render_widget(TableWorld::new(app, world.width, world.height), world);
}

struct TableWorld<'a> {
    /// application status
    app: &'a App,
    /// area width
    width: u16,
    /// area height
    height: u16,
}

impl<'a> TableWorld<'a> {
    fn new(app: &'a App, width: u16, height: u16) -> Self {
        Self { app, width, height }
    }

    fn make_rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::with_capacity(self.height as usize);
        for iy in
            self.app.rendering_iy..min(self.app.ny, self.app.rendering_iy + self.height as usize)
        {
            let mut row: Vec<_> = Vec::with_capacity(self.width as usize);
            for ix in
                self.app.rendering_ix..min(self.app.nx, self.app.rendering_ix + self.width as usize)
            {
                row.push(match self.app.world.get_present_cell(ix, iy) {
                    CELL_ALIVE => Cell::from(" ").style(Style::default().bg(Color::Blue)),
                    CELL_DEAD => Cell::from(" ").style(Style::default()),
                });
            }
            rows.push(Row::new(row));
        }
        rows
    }

    fn make_widths(&self) -> Vec<Constraint> {
        let mut widths = Vec::with_capacity(self.width as usize);
        for _ in 0..self.width {
            widths.push(Constraint::Length(1));
        }
        widths
    }
}

impl Widget for TableWorld<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Table::new(self.make_rows(), self.make_widths())
            .column_spacing(0)
            .render(area, buf);
    }
}
