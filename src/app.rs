use std::error;

use rand::Rng;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayFieldCell {
    pub falling: bool,
    pub landed: bool,
}

impl Default for PlayFieldCell {
    fn default() -> Self {
        Self {
            landed: false,
            falling: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Tetromino {
    pub rotations: [[[bool; 4]; 4]; 4],
}

/// pre-defined tetromino shapes
pub const TETROMINO_SHAPES: [Tetromino; 7] = [
    Tetromino {
        rotations: [
            [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            [
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
    },
    Tetromino {
        rotations: [
            [
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
        ],
    },
    // Additional Tetromino shapes
    // ...
];
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub playfield: [[PlayFieldCell; 10]; 20],

    pub current_tetromino: Tetromino,

    pub x: usize,
    pub y: usize,

    pub current_rotation: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            playfield: [[PlayFieldCell::default(); 10]; 20],

            current_tetromino: TETROMINO_SHAPES[0],

            x: 0,
            y: 0,

            current_rotation: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// randomly spawns a tetromino at the specified position on the playfield
    pub fn spawn_tetromino_random(&mut self, start_x: usize, start_y: usize) -> Tetromino {
        let index = rand::thread_rng().gen_range(0..TETROMINO_SHAPES.len());
        let tetromino = TETROMINO_SHAPES[index];
        for y in 0..tetromino.rotations[self.current_rotation].len() {
            for x in 0..tetromino.rotations[self.current_rotation][y].len() {
                if tetromino.rotations[self.current_rotation][y][x] {
                    self.playfield[start_y + y][start_x + x].falling = true;
                }
            }
        }
        tetromino
    }

    pub fn move_tetromino(&mut self, x: i32, y: i32, tetromino: Tetromino) {
        let mut in_range: bool = true;
        self.clear_falling();
        if x < 0 && self.x as i32 + x < 0 {
            in_range = false;
        } else if x > 0 && self.x as i32 + x > 9 {
            in_range = false;
        }
        if in_range {
            self.x = (self.x as i32 + x) as usize;
            self.y = (self.y as i32 + y) as usize;
        }
        for y in 0..tetromino.rotations[self.current_rotation].len() {
            for x in 0..tetromino.rotations[self.current_rotation][y].len() {
                if tetromino.rotations[self.current_rotation][y][x] {
                    self.playfield[self.y + y][self.x + x].falling = true;
                }
            }
        }
    }

    pub fn clear_falling(&mut self) {
        for y in 0..self.playfield.len() {
            for x in 0..self.playfield[y].len() {
                if self.playfield[y][x].falling {
                    self.playfield[y][x].falling = false;
                }
            }
        }
    }

    pub fn land_tetromino(&mut self) {
        for y in 0..self.current_tetromino.rotations[self.current_rotation].len() {
            for x in 0..self.current_tetromino.rotations[self.current_rotation][y].len() {
                if self.current_tetromino.rotations[self.current_rotation][y][x] {
                    self.playfield[self.y + y][self.x + x].landed = true;
                }
            }
        }
    }

    /// fill the bottom row of playfield and some in next row
    pub fn fill_playfield(&mut self) {
        for i in 0..10 {
            self.playfield[19][i].landed = true;
        }
        for i in 0..10 {
            self.playfield[18][i].landed = true;
        }
        //have a chance to fill next row
        for i in 0..10 {
            if rand::random() {
                self.playfield[17][i].landed = true;
            }
        }
    }

    /// Returns the playfield as a string.
    pub fn playfield_string(&self) -> String {
        let mut result = String::new();
        for row in &self.playfield {
            for cell in row {
                result.push_str(if cell.landed {
                    "██"
                } else if cell.falling {
                    "██"
                } else {
                    "  "
                });
            }
            result.push('\n');
        }
        result
    }
}
