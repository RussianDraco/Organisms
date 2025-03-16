mod cell;
mod grid;
mod organism;

use grid::Grid;
use organism::Organism;
use std::collections::LinkedList;
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

    let mut organisms: LinkedList<Organism> = LinkedList::new(); 

    grid.draw_organisms(organisms);

    loop {
        clear_background(BLACK);
        grid.draw();

        next_frame().await;
    }
}
