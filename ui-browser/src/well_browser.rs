use crate::Well;
use crate::well::Direction;


pub struct WellBrowser {
    well: Well,
}

pub trait Browser {
    /*
    pub is implied in traits
     */
    fn new() -> Self;

    fn run(&mut self) -> ();
    fn move_tetromino(&mut self, direction: Direction) -> ();
    fn render_tetromino(&mut self, erase: bool) -> ();
    fn render_falling_blocks(&mut self) -> ();
    fn render_edges_and_stuck_pieces(&mut self) -> ();
    fn render_score(&mut self, score: i32);
    fn render_game_status(&mut self, status: &str);

    fn record_high_score(&mut self) -> ();
    fn get_high_score(&self) -> i32;
    fn set_high_score(&self, high_score: i32) -> ();

    fn log_grid(&self) -> ();
    fn quit(&mut self) -> ();
}

impl Browser for WellBrowser {
    fn new() -> Self {
        todo!()
    }

    fn run(&mut self) -> () {
        todo!()
    }

    fn move_tetromino(&mut self, direction: Direction) -> () {
        todo!()
    }

    fn render_tetromino(&mut self, erase: bool) -> () {
        todo!()
    }

    fn render_falling_blocks(&mut self) -> () {
        todo!()
    }

    fn render_edges_and_stuck_pieces(&mut self) -> () {
        todo!()
    }

    fn render_score(&mut self, score: i32) {
        todo!()
    }

    fn render_game_status(&mut self, status: &str) {
        todo!()
    }

    fn record_high_score(&mut self) -> () {
        todo!()
    }

    fn get_high_score(&self) -> i32 {
        todo!()
    }

    fn set_high_score(&self, high_score: i32) -> () {
        todo!()
    }

    fn log_grid(&self) -> () {
        todo!()
    }

    fn quit(&mut self) -> () {
        todo!()
    }
}
