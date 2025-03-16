use crate::{cell::Cell, grid};
use rand::prelude::*;
use crate::utils::{WIDTH, HEIGHT};

pub struct Organism {
    pub x: usize,
    pub y: usize,
    pub cells: Vec<(i32, i32, Cell)>,
    pub energy: i32,

    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Organism {
    pub fn new(x: usize, y: usize, cells: Vec<(i32, i32, Cell)>) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for (dx, dy, _) in cells.iter() {
            if *dx < min_x {
                min_x = *dx;
            }
            if *dx > max_x {
                max_x = *dx;
            }
            if *dy < min_y {
                min_y = *dy;
            }
            if *dy > max_y {
                max_y = *dy;
            }
        }

        Organism { x, y, cells, energy: 0, 
            min_x: min_x.abs() as usize, max_x: max_x as usize, min_y: min_y.abs() as usize, max_y: max_y as usize}
    }

    pub fn move_org(&mut self, dx: i32, dy: i32) {
        self.x = (self.x as i32 + dx) as usize;
        self.y = (self.y as i32 + dy) as usize;
        if self.x >= WIDTH - self.max_x {
            self.x = WIDTH - self.max_x - 1;
        }
        if self.y >= HEIGHT - self.max_y {
            self.y = HEIGHT - self.max_y - 1;
        }
        if self.x <= self.min_x {
            self.x = self.min_x + 1;
        }
        if self.y <= self.min_y {
            self.y = self.min_y + 1;
        }
    }

    pub fn random_movement(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);
        self.move_org(dx, dy);
    }

    pub fn update(&mut self, grid: &mut grid::Grid) {
        let mut will_move: bool = false;
        for (dx, dy, cell) in self.cells.iter() {
            let x = (self.x as i32 + dx) as usize;
            let y = (self.y as i32 + dy) as usize;
            match cell {
                Cell::Mouth => {
                    if grid.mouth_eat(x, y) {
                        self.energy += 1;
                    }
                }
                Cell::Producer => {
                    grid.produce_food(x, y);
                }
                Cell::Mover => {
                    will_move = true;
                }
                Cell::Killer => {
                    // Do something
                }
                Cell::Armor => {
                    // Do something
                }
                Cell::Eye => {
                    // Do something
                }
                Cell::Brain => {
                    // Do something
                }
                _ => {}
            }
        }

        if will_move {
            self.random_movement();
        }
    }
}
