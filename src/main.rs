mod cell;
mod grid;
mod organism;
mod utils;
mod organism_manager;
mod brain;
#[cfg(feature = "tuning")]
mod tuner;

use ::rand::{Rng, rngs::StdRng};
use grid::Grid;
use organism_manager::OrganismManager;
use macroquad::prelude::*;

#[cfg(feature = "tuning")]
const TUNING: bool = true;
#[cfg(not(feature = "tuning"))]
const TUNING: bool = false;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}
impl Direction {
    pub fn random_direction(rng: &mut StdRng) -> Direction {
        match (*rng).gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::None,
        }
    }
    pub fn x_offset(&self) -> f32 {
        match self {
            Direction::Up => 0.0,
            Direction::Down => 0.0,
            Direction::Left => -1.0,
            Direction::Right => 1.0,
            Direction::None => 0.0,
        }
    }
    pub fn y_offset(&self) -> f32 {
        match self {
            Direction::Up => -1.0,
            Direction::Down => 1.0,
            Direction::Left => 0.0,
            Direction::Right => 0.0,
            Direction::None => 0.0,
        }
    }
}

fn window_conf() -> Conf {
    let screen_size = Grid::screen_size();
    Conf {
        window_title: "Life Engine".to_owned(),
        window_width: screen_size.0, // Set your desired width here
        window_height: screen_size.1, // Set your desired height here
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if TUNING {
        #[cfg(feature = "tuning")]
        tuner::main();
        return;
    }

    let mut organism_manager = OrganismManager::new();

    organism_manager.init();

    loop {
        clear_background(BLACK);

        organism_manager.update();

        next_frame().await;
    }
}
