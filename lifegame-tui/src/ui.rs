use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Row, Table, Widget},
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
    frame.render_widget(
        Block::default().title(format!(
            "Lifegame (gen={}) {}{}[<q>: quit]",
            app.gen,
            if app.state == AppState::Pause {
                "[<s>: start] "
            } else {
                "[<s>: pause ]"
            },
            if app.can_reset() { "[<r>: reset] " } else { "" }
        )),
        layout[0],
    );
    frame.render_widget(TableWorld::new(app), layout[1]);
}

struct TableWorld<'a> {
    rows: Vec<Row<'a>>,
    widths: Vec<Constraint>,
}

impl<'a> TableWorld<'a> {
    fn new(app: &'a App) -> Self {
        Self {
            rows: Self::make_rows(app),
            widths: Self::make_widths(app),
        }
    }

    fn make_rows(app: &App) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::with_capacity(app.ny);
        for irow in 0..app.ny {
            let mut row: Vec<String> = Vec::with_capacity(app.nx);
            for icol in 0..app.nx {
                row.push(match app.world.cell(icol, irow) {
                    lifegame_core::Cell::Alive => "█".to_string(),
                    lifegame_core::Cell::Dead => " ".to_string(),
                });
            }
            rows.push(Row::new(row));
        }
        rows
    }

    fn make_widths(app: &App) -> Vec<Constraint> {
        let mut widths = Vec::with_capacity(app.nx);
        for _ in 0..app.nx {
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
        Table::new(self.rows, self.widths)
            .column_spacing(0)
            .style(Style::new().blue())
            .render(area, buf);
    }
}
