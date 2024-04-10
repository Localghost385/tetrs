use crate::{
	app::{
		App,
		AppResult,
	},
	tetromino::TETROMINO_SHAPES,
};
use crossterm::event::{
	KeyCode,
	KeyEvent,
	KeyModifiers,
};
use rand::Rng;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
	if app.paused && (key_event.code != KeyCode::Char('p')) {
		// Exit application on `ESC`
		if key_event.code == KeyCode::Esc {
			app.quit();
		}
		// Exit application on `Ctrl-C`
		else if key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C') {
			if key_event.modifiers == KeyModifiers::CONTROL {
				app.quit();
			}
		}
		return Ok(()); // Return early if not paused and not 'p' key
	}

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
		// Toggle pause
		KeyCode::Char('p') => {
			app.paused = !app.paused;
		}
		// Counter handlers
		KeyCode::Right => {}
		KeyCode::Left => {
			app.clear_falling();
			app.current_tetromino = app.spawn_tetromino(
				app.x,
				app.y,
				TETROMINO_SHAPES[rand::thread_rng().gen_range(0..TETROMINO_SHAPES.len())],
			);
		}
		KeyCode::Up => {
			app.reset_tetromino();
		}
		KeyCode::Char(' ') => {
			app.drop_tetromino();
		}
		KeyCode::Char('w') => {
			app.swap_tetromino();
		}
		KeyCode::Char('a') => {
			if !app.has_landed_cells_at_offset(-1, 0) {
				app.move_tetromino(-1, 0, app.current_tetromino);
			}
		}
		KeyCode::Char('s') => {
			app.tick_count_target = 0;
		}
		KeyCode::Char('d') => {
			if !app.has_landed_cells_at_offset(1, 0) {
				app.move_tetromino(1, 0, app.current_tetromino);
			}
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
