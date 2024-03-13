use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.fill_playfield();
        }
        KeyCode::Left => {
            app.clear_falling();
            app.current_tetromino = app.spawn_tetromino_random(app.x, app.y);
        }
        KeyCode::Up => {
            app.reset_tetromino();
        }
        KeyCode::Char('w') => {
            app.swap_tetromino();
        }
        KeyCode::Char('a') => {
            app.move_tetromino(-1, 0, app.current_tetromino);
        }
        KeyCode::Char('s') => {
            app.tick_count_target = 0;
        }
        KeyCode::Char('d') => {
            app.move_tetromino(1, 0, app.current_tetromino);
        }
        KeyCode::Char('q') => {
            if app.current_rotation > 0 {
                app.current_rotation -= 1;
            } else {
                app.current_rotation = 3;
            }
            app.clear_falling();
            app.move_tetromino(0, 0, app.current_tetromino);
        }
        KeyCode::Char('e') => {
            if app.current_rotation < 3 {
                app.current_rotation += 1;
            } else {
                app.current_rotation = 0;
            }
            app.clear_falling();
            app.move_tetromino(0, 0, app.current_tetromino);
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
