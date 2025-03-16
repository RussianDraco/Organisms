mod cell;
mod grid;
mod organism;
mod utils;
mod organism_manager;

use grid::Grid;
use organism_manager::OrganismManager;
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
    let mut organism_manager = OrganismManager::new();

    organism_manager.init();

    loop {
        clear_background(BLACK);

        organism_manager.draw_organisms();
        organism_manager.update();

        next_frame().await;
    }
}
