#[cfg(test)]
mod unit_tests {
    use crate::{app::App, tetromino::TETROMINO_SHAPES};

    #[test]
    fn test_default_app() {
        let app = App::default();
        assert!(app.running);
        assert_eq!(app.playfield.len(), 24);
        assert_eq!(app.playfield[0].len(), 14);
        // Add more assertions for other fields if needed
    }

    #[test]
    fn test_tick() {
        let mut app = App::default();
        app.tick();
        // Add assertions to test the behavior of the tick method
    }

    #[test]
    fn test_quit() {
        let mut app = App::default();
        app.quit();
        assert!(!app.running);
    }

    #[test]
    fn test_move_tetromino() {
        let mut app = App::default();
        let initial_x = app.x;
        let initial_y = app.y;
        let tetromino = TETROMINO_SHAPES[4]; // You may need to modify this to get a non-empty tetromino
        app.move_tetromino(1, 1, tetromino);
        assert_eq!(app.x, initial_x + 1);
        assert_eq!(app.y, initial_y + 1);
    }

    #[test]
    fn test_reset_tetromino() {
        let mut app = App::default();
        let initial_tetromino = app.current_tetromino;
        app.reset_tetromino();
        assert_ne!(app.current_tetromino, initial_tetromino);
        // Add more assertions as needed
    }

    #[test]
    fn test_swap_tetromino() {
        let mut app = App::default();
        let current_tetromino = app.current_tetromino;
        let swap_tetromino = app.swap_tetromino;
        app.swap_tetromino();
        assert_eq!(app.current_tetromino, swap_tetromino);
        assert_eq!(app.swap_tetromino, current_tetromino);
    }

    #[test]
    fn test_playfield_string() {
        let app = App::default();
        // Set up playfield with some landed and falling cells
        // For example:
        // app.playfield[4][4].landed = true;
        // app.playfield[5][5].falling = true;
        let _playfield_string = app.playfield_string();
        // Add assertions to check the generated string
    }

    // Add more test functions for other methods as needed
}
