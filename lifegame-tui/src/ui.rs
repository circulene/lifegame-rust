use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Row, Table},
    Frame,
};

use crate::app::App;

fn make_rows(app: &App) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::with_capacity(app.ny);
    for irow in 0..app.ny {
        let mut row: Vec<String> = Vec::with_capacity(app.nx);
        for icol in 0..app.nx {
            row.push(match app.world.cell(icol, irow) {
                lifegame_core::Cell::Alive => "■".to_string(),
                lifegame_core::Cell::Dead => "□".to_string(),
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

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let rows = make_rows(app);
    let widths = make_widths(app);
    let table = Table::new(rows, widths)
        .column_spacing(0)
        .style(Style::new().blue())
        .block(Block::new().title("Lifegame"));

    frame.render_widget(table, frame.size())
}
