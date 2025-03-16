mod cell;
mod grid;
mod organism;
mod utils;

use grid::Grid;
use organism::Organism;
use macroquad::prelude::*;

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
    let mut grid = Grid::new();

    let mut organisms: Vec<Organism> = Vec::new(); 

    organisms.push(Organism::new(50, 50, vec![(0, 0, cell::Cell::Mover), (0, 1, cell::Cell::Mouth), (1, 0, cell::Cell::Producer)]));

    loop {
        clear_background(BLACK);
        grid.draw_organisms(&mut organisms);
        for organism in organisms.iter_mut() {
            organism.update(&mut grid);
        }

        next_frame().await;
    }
}
