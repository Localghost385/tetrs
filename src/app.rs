use std::error;

use rand::Rng;

use crate::tetromino::{Tetromino, TETROMINO_SHAPES};

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

    pub x: usize,
    pub y: usize,

    pub current_rotation: usize,

    pub tick_count: u32,
    pub tick_count_target: u32,

    pub grace_period: bool,

    pub paused: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            playfield: vec![vec![PlayFieldCell::default(); 18]; 24],
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
            x: 7,
            y: 4,
            current_rotation: 0,
            tick_count: 0,
            tick_count_target: 5,
            grace_period: false,
            paused: false,
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
        self.check_for_line_clear();
        self.tick_count += 1;
        if self.tick_count > self.tick_count_target {
            if self.is_tetromino_at_bottom() || self.has_landed_cell_below() {
                if self.grace_period {
                    self.reset_tetromino();
                }
                self.grace_period = !self.grace_period;
            }
            self.tick_count = 0;
            if !self.is_tetromino_at_bottom() && !self.has_landed_cell_below() {
                self.move_tetromino(0, 1, self.current_tetromino);
            }
        }

        self.tick_count_target = 5;
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
        for y in 0..tetromino.rotations[self.current_rotation].len() {
            for x in 0..tetromino.rotations[self.current_rotation][y].len() {
                if tetromino.rotations[self.current_rotation][y][x] {
                    self.playfield[start_y + y][start_x + x].falling = true;
                }
            }
        }
        tetromino
    }

    /// prepares for next tetromino.
    pub fn reset_tetromino(&mut self) {
        self.land_tetromino();
        self.clear_falling();
        self.x = 7;
        self.y = 3;
        self.current_rotation = 0;
        self.current_tetromino = self.spawn_tetromino(
            self.x,
            self.y,
            TETROMINO_SHAPES[rand::thread_rng().gen_range(0..TETROMINO_SHAPES.len())],
        );
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

    pub fn check_for_line_clear(&mut self) -> u32 {
        let mut lines_cleared = 0;

        for y in (0..self.playfield.len()).rev() {
            let mut remove_row = true;
            for x in 4..15 {
                if !self.playfield[y][x].landed {
                    remove_row = false;
                    break;
                }
            }
            if remove_row {
                self.playfield.remove(y);
                self.playfield.insert(0, vec![PlayFieldCell::default(); 18]);
                lines_cleared += 1;
            }
        }
        lines_cleared
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
        self.x = 7;
        self.y = 3;
        self.current_rotation = 0;
        if !self.current_tetromino.rotations[self.current_rotation]
            .iter()
            .any(|row| row.iter().any(|&cell| cell))
        {
            self.current_tetromino = self.spawn_tetromino(
                self.x,
                self.y,
                TETROMINO_SHAPES[rand::thread_rng().gen_range(0..TETROMINO_SHAPES.len())],
            );
        }
    }
    //------------------------------------------//

    //----------[ Boundary checking ]----------//
    /// check if tetromino is at the bottom.
    pub fn is_tetromino_at_bottom(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_tetromino.rotations[self.current_rotation][y][x]
                    && (self.y + y + 1 >= self.playfield.len()
                        || self.playfield[self.y + y + 1][self.x + x].landed)
                {
                    return true;
                }
            }
        }
        false
    }

    /// check if tetromino has a landed cell below it
    pub fn has_landed_cell_below(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_tetromino.rotations[self.current_rotation][y][x]
                    && (self.y + y >= self.playfield.len()
                        || self.playfield[self.y + y][self.x + x].landed)
                {
                    return true;
                }
            }
        }
        false
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
        for y in 0..4 {
            for x in 1..3 {
                result.push_str(if tetromino.rotations[1][y][x] {
                    "██"
                } else {
                    "  "
                });
            }
            result.push('\n');
        }
        result
    }
    //---------------------------------//
}
