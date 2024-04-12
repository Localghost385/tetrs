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
	MouseButton,
	MouseEvent,
	MouseEventKind,
};
use rand::Rng;

/// handles the mouse events
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) {
	if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
		let mut button_index: u16 = 99;
		for (index, button) in app.buttons.iter().enumerate() {
			if mouse_event.column >= button.x
				&& mouse_event.column < button.x + button.width
				&& mouse_event.row >= button.y
				&& mouse_event.row < button.y + button.height
			{
				button_index = index as u16;
				break; // Exit the loop after finding the matching button
			}
		}
		match button_index {
			0 => {
				if app.current_rotation > 0 {
					app.current_rotation -= 1;
				} else {
					app.current_rotation = 3;
				}
				app.clear_falling();
				app.move_tetromino(0, 0, app.current_tetromino);
			}
			1 => {
				if !app.has_landed_cells_at_offset(-1, 0) {
					app.move_tetromino(-1, 0, app.current_tetromino);
				}
			}
			2 => {
				if app.current_rotation < 3 {
					app.current_rotation += 1;
				} else {
					app.current_rotation = 0;
				}
				app.clear_falling();
				app.move_tetromino(0, 0, app.current_tetromino);
			}
			3 => {
				if !app.has_landed_cells_at_offset(1, 0) {
					app.move_tetromino(1, 0, app.current_tetromino);
				}
			}
			4 => {
				app.tick_count_target = 0;
			}
			5 => {
				app.drop_tetromino();
			}
			6 => {
				app.paused = !app.paused;
			}
			7 => {
				app.swap_tetromino();
			}
			_ => {}
		}
	}
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
	if app.paused && (key_event.code != KeyCode::Char('p')) {
		// Exit application on `ESC`
		if key_event.code == KeyCode::Esc
			|| (key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C'))
				&& key_event.modifiers == KeyModifiers::CONTROL
		{
			app.quit();
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
		KeyCode::Right => {
			app.playfield[app.start_y + 1_usize][app.start_x + 1_usize].landed =
				!app.playfield[app.start_y + 1_usize][app.start_x + 1_usize].landed;
		}
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
