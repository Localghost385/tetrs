
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
