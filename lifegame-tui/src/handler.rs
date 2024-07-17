use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Run/Pause lifegame
        KeyCode::Char('s') => {
            app.toggle();
        }
        // Reset lifegame
        KeyCode::Char('r') => {
            app.reset()?;
        }
        // Pan rendering area to left
        KeyCode::Left => {
            app.rendering_ix = app.rendering_ix.saturating_sub(1);
        }
        // Pan rendering area to right
        KeyCode::Right => {
            app.rendering_ix = app.rendering_ix.saturating_add(1);
        }
        // Pan rendering area to left
        KeyCode::Up => {
            app.rendering_iy = app.rendering_iy.saturating_sub(1);
        }
        // Pan rendering area to right
        KeyCode::Down => {
            app.rendering_iy = app.rendering_iy.saturating_add(1);
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
