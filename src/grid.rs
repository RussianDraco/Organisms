use crate::organism::Organism;

use crate::cell::Cell;
use std::collections::LinkedList;
use macroquad::prelude::*;
use ::rand::prelude::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: f32 = 7.0;

pub struct Grid {
    pub cells: [[Cell; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
        };
        grid
    }

    pub fn screen_size() -> (i32, i32) {
        ((WIDTH as f32 * CELL_SIZE) as i32, (HEIGHT as f32 * CELL_SIZE) as i32)
    }

    pub fn draw_organisms(&mut self, organisms: &mut LinkedList<Organism>) {
        for organism in organisms.iter() {
            self.cells[organism.y][organism.x] = Cell::Organism;
        }
    }

    pub fn draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = match self.cells[y][x] {
                    Cell::Empty => DARKGRAY,
                    Cell::Food => GREEN,
                    Cell::Organism => RED,
                };
                draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
            }
        }
    }
}
