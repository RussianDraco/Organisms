use crate::{cell::Cell, grid};
use rand::prelude::*;
use crate::utils::{WIDTH, HEIGHT, DEFAULT_LIFETIME};

#[derive(PartialEq)]
pub struct Organism {
    pub x: usize,
    pub y: usize,
    pub cells: Vec<(i32, i32, Cell)>,
    pub energy: i32,

    lifetime: i32,
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

        Organism { x, y, cells, energy: 0, lifetime: DEFAULT_LIFETIME,
            min_x: min_x.abs() as usize, max_x: max_x as usize, min_y: min_y.abs() as usize, max_y: max_y as usize}
    }
    pub fn child(&self) -> Self {
        Organism { x: self.x, y: self.y, cells: self.cells.clone(), energy: 0, lifetime: DEFAULT_LIFETIME,
            min_x: self.min_x, max_x: self.max_x, min_y: self.min_y, max_y: self.max_y}
    }

    pub fn random_offset(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-5..=5);
        let dy = rng.gen_range(-5..=5);
        self.x = (self.x as i32 + dx) as usize;
        self.y = (self.y as i32 + dy) as usize;
    }

    pub fn move_org(&mut self, dx: i32, dy: i32, grid: &grid::Grid) {
        let new_x = (self.x as i32 + dx) as usize;
        let new_y = (self.y as i32 + dy) as usize;
    
        if new_x >= WIDTH - self.max_x || new_y >= HEIGHT - self.max_y {
            return;
        }
        if new_x <= self.min_x || new_y <= self.min_y {
            return;
        }
    
        for (dx, dy, _) in self.cells.iter() {
            let check_x = (new_x as i32 + dx) as usize;
            let check_y = (new_y as i32 + dy) as usize;
            
            if !grid.is_cell_empty(check_x, check_y) {
                return;
            }
        }
    
        self.x = new_x;
        self.y = new_y;
    }
    

    pub fn random_movement(&mut self, grid: &mut grid::Grid) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);
        self.move_org(dx, dy, grid);
    }

    pub fn can_reproduce(&self) -> bool {
        self.energy >= self.cells.len() as i32
    }
    pub fn consume_reproduction_energy(&mut self) {
        self.energy -= self.cells.len() as i32;
    }

    pub fn update(&mut self, grid: &mut grid::Grid) -> bool {
        self.lifetime -= 1;
        if self.lifetime <= 0 {
            grid.make_remains(self);
            return false;
        }

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
            self.random_movement(grid);
        }
        true
    }
}
