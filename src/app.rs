use std::{
	error,
	vec,
};

use rand::{
	prelude::SliceRandom,
	Rng,
};
use ratatui::layout::Rect;

use crate::tetromino::{
	Tetromino,
	TETROMINO_SHAPES,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

//----------[ Structs ]----------//
/// Application.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlayFieldCell {
	pub falling: bool,
	pub landed: bool,
}

#[derive(Debug)]
pub struct App {
	/// Is the application running?
	pub running: bool,
	/// counter
	pub playfield: Vec<Vec<PlayFieldCell>>,

	pub current_tetromino: Tetromino,
	pub swap_tetromino: Tetromino,
	pub tetromino_queue: Vec<Tetromino>,

	pub start_x: usize,
	pub start_y: usize,
	pub x: usize,
	pub y: usize,

	pub current_rotation: usize,

	pub tick_count: u32,
	pub tick_count_target: u32,
	pub default_tick_count_target: u32,

	pub grace_period: bool,

	pub paused: bool,

	pub buttons: Vec<Rect>,

	pub score: u32,
	pub high_score: u32,
	pub level: u32,
}

impl Default for App {
	fn default() -> Self {
		Self {
			running: true,
			playfield: vec![vec![PlayFieldCell::default(); 18]; 26],
			current_tetromino: TETROMINO_SHAPES[rand::thread_rng().gen_range(0..7)],
			swap_tetromino: Tetromino {
				rotations: [
					[
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
					],
					[
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
					],
					[
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
					],
					[
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
						[false, false, false, false],
					],
				],
			},
			tetromino_queue: vec![],
			start_x: 7,
			start_y: 4,
			x: 7,
			y: 4,
			current_rotation: 0,
			tick_count: 0,
			tick_count_target: 0,
			default_tick_count_target: 15,
			grace_period: false,
			paused: false,
			buttons: vec![],
			score: 0,
			high_score: 0,
			level: 1,
		}
	}
}
//-------------------------------//

impl App {
	//----------[ Loop Functions ]----------//
	/// Constructs a new instance of [`App`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Handles the tick event of the terminal.
	pub fn tick(&mut self) {
		if self.paused {
			return;
		}

		if self.tetromino_queue.len() < 7 {
			self.populate_tetromino_queue();
		}

		self.tick_count += 1;
		if self.tick_count > self.tick_count_target {
			self.score += self.check_for_line_clear().pow(2) * 100 * self.level;
			self.check_for_next_level();
			if self.has_landed_cells_at_offset(0, 1) {
				if self.grace_period {
					self.reset_tetromino();
				}
				self.grace_period = !self.grace_period;
			}

			self.tick_count = 0;

			if !self.has_landed_cells_at_offset(0, 1) {
				self.move_tetromino(0, 1, self.current_tetromino);
				self.grace_period = false;
			}
		}
		self.tick_count_target = self.default_tick_count_target;
	}

	/// Set running to false to quit the application.
	pub fn quit(&mut self) {
		self.running = false;
	}
	//--------------------------------------//

	//----------[ Tetromino Movement ]----------//
	/// Randomly spawns a tetromino at the specified position on the playfield.
	pub fn spawn_tetromino(
		&mut self,
		start_x: usize,
		start_y: usize,
		tetromino: Tetromino,
	) -> Tetromino {
		self.check_for_game_over();

		for y in 0..tetromino.rotations[self.current_rotation].len() {
			for x in 0..tetromino.rotations[self.current_rotation][y].len() {
				if tetromino.rotations[self.current_rotation][y][x] {
					self.playfield[start_y + y][start_x + x].falling = true;
				}
			}
		}
		tetromino
	}

	pub fn populate_tetromino_queue(&mut self) {
		let mut tetromino_order: Vec<usize> = (0..7).collect();
		tetromino_order.shuffle(&mut rand::thread_rng());

		for tetromino in tetromino_order {
			self.tetromino_queue.push(TETROMINO_SHAPES[tetromino]);
		}
	}

	/// prepares for next tetromino.
	pub fn reset_tetromino(&mut self) {
		self.land_tetromino();
		self.clear_falling();
		self.x = self.start_x;
		self.y = self.start_y;
		self.current_rotation = 0;
		self.current_tetromino = self.spawn_tetromino(self.x, self.y, self.tetromino_queue[0]);
		self.tetromino_queue.remove(0);
	}

	/// Places the tetromino on the playfield.
	pub fn land_tetromino(&mut self) {
		self.current_tetromino.rotations[self.current_rotation]
			.iter()
			.enumerate()
			.for_each(|(y, row)| {
				row.iter().enumerate().for_each(|(x, &cell)| {
					if cell {
						self.playfield[self.y + y][self.x + x].landed = true;
					}
				});
			});
	}

	/// Moves the tetromino.
	pub fn move_tetromino(&mut self, move_x: i32, move_y: i32, tetromino: Tetromino) {
		self.clear_falling();
		let new_x = (self.x as i32 + move_x) as usize;
		let new_y = (self.y as i32 + move_y) as usize;

		for y in 0..4 {
			for x in 0..4 {
				if tetromino.rotations[self.current_rotation][y][x] {
					let playfield_x = new_x + x;
					let playfield_y = new_y + y;
					if playfield_y < self.playfield.len() && playfield_x < self.playfield[0].len() {
						self.playfield[playfield_y][playfield_x].falling = true;
					}
				}
			}
		}
		self.x = new_x;
		self.y = new_y;
	}

	/// Instanly moves the tetromino as far down as possible.
	pub fn drop_tetromino(&mut self) {
		let mut min_drops = 20;
		for y in 0..4 {
			for x in 0..4 {
				if self.current_tetromino.rotations[self.current_rotation][y][x] {
					let mut drops = 0;
					let mut y_temp = y;
					while self.y + y_temp < self.playfield.len()
						&& self.x + x < self.playfield[y_temp].len()
						&& !self.playfield[self.y + y_temp][self.x + x].landed
					{
						y_temp += 1;
						drops += 1;
					}
					if drops < min_drops {
						min_drops = drops;
					}
				}
			}
		}
		self.move_tetromino(0, min_drops - 1, self.current_tetromino);
		self.reset_tetromino();
	}

	/// Swap current and swap tetromino, and spawn new one if current tetromino is all false.
	pub fn swap_tetromino(&mut self) {
		std::mem::swap(&mut self.current_tetromino, &mut self.swap_tetromino);
		self.clear_falling();
		self.x = self.start_x;
		self.y = self.start_y;
		self.current_rotation = 0;
		if !self.current_tetromino.rotations[self.current_rotation]
			.iter()
			.any(|row| row.iter().any(|&cell| cell))
		{
			self.current_tetromino = self.spawn_tetromino(self.x, self.y, self.tetromino_queue[0]);
			self.tetromino_queue.remove(0);
		}
	}
	//------------------------------------------//

	//----------[ Checks ]----------//
	/// Check if there are landed cells at the specified offset from the tetromino's position.
	pub fn has_landed_cells_at_offset(&self, x_offset: i32, y_offset: i32) -> bool {
		for y in 0..4 {
			for x in 0..4 {
				if self.current_tetromino.rotations[self.current_rotation][y][x] {
					let check_y = (self.y + y) as i32 + y_offset;
					let check_x = (self.x + x) as i32 + x_offset;

					let out_of_bounds = check_y >= self.playfield.len() as i32
						|| check_y < 0 || !(4..=13).contains(&check_x);

					if out_of_bounds || self.playfield[check_y as usize][check_x as usize].landed {
						return true;
					}
				}
			}
		}
		false
	}

	pub fn check_for_line_clear(&mut self) -> u32 {
		let mut lines_cleared = 0;
		let mut lines_to_be_cleared: Vec<usize> = vec![];

		for y in 0..self.playfield.len() {
			let mut remove_row = true;
			for x in 4..14 {
				if !self.playfield[y][x].landed {
					remove_row = false;
					break;
				}
			}
			if remove_row {
				lines_to_be_cleared.push(y);
				lines_cleared += 1;
			}
		}
		for row in lines_to_be_cleared {
			self.playfield.remove(row);
			self.playfield.insert(0, vec![PlayFieldCell::default(); 18]);
		}

		lines_cleared
	}

	pub fn check_for_game_over(&self) -> bool {
		if self.playfield[self.start_y + 1_usize][self.start_x + 1_usize].landed {
			// path of home directory
			let mut path = dirs::home_dir().unwrap();
			path.push(".tetrs_highscore");

			let file = match std::fs::read_to_string(&path) {
				Ok(contents) => contents,
				Err(_) => {
					std::fs::write(&path, "0").expect("Failed to create file");
					String::from("0")
				}
			};
			let highscore = file.parse::<u32>().unwrap_or(0);

			if self.score > highscore {
				std::fs::write(&path, self.score.to_string()).expect("Failed to write file");
			}
			return true;
		}
		false
	}
	pub fn check_for_highscore(&mut self) {
		// path of home directory
		let mut path = dirs::home_dir().unwrap();
		path.push(".tetrs_highscore");

		let file = match std::fs::read_to_string(&path) {
			Ok(contents) => contents,
			Err(_) => {
				std::fs::write(&path, "0").expect("Failed to create file");
				String::from("0")
			}
		};
		let high_score = file.parse::<u32>().unwrap_or(0);
		self.high_score = high_score;
	}

	pub fn check_for_next_level(&mut self) {
		if self.score > 500 * self.level.pow(3){
			self.level += 1;
			self.default_tick_count_target -= 1;
		}
	}
	//-----------------------------------------//

	//----------[ Rendering ]----------//
	/// Clears the falling cells from the playfield.
	pub fn clear_falling(&mut self) {
		self.playfield.iter_mut().for_each(|row| {
			row.iter_mut().for_each(|cell| {
				if cell.falling {
					cell.falling = false;
				}
			});
		});
	}

	/// Returns the playfield as a string.
	pub fn playfield_string(&self) -> String {
		let mut result = String::new();
		for row_index in 4..self.playfield.len() {
			let row = &self.playfield[row_index];
			for col_index in row.iter().skip(4) {
				let cell = col_index;
				result.push_str(if cell.landed {
					"██"
				} else if cell.falling {
					"▒▒"
				} else {
					"  "
				});
			}
			result.push('\n');
		}
		result
	}

	/// Returns a tetromino as a string.
	pub fn tetromino_string(&self, tetromino: Tetromino) -> String {
		let mut result = String::new();
		for (_, row) in tetromino.rotations[1]
			.iter()
			.enumerate()
			.take_while(|(_, row)| !row.iter().all(|&cell| !cell))
		{
			for &cell in row.iter().skip(1).take(2) {
				result.push_str(if cell { "██" } else { "  " });
			}
			result.push('\n');
		}
		result
	}

	/// Returns the tetromino queue as a string.
	pub fn tetromino_queue_string(&self) -> String {
		let mut result = String::new();
		for tetromino in &self.tetromino_queue {
			result += &self.tetromino_string(*tetromino);
			result.push('\n');
		}
		result
	}
	//---------------------------------//
}
