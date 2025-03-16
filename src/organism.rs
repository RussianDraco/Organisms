use crate::{cell::Cell, grid};
use std::collections::VecDeque;
use rand::prelude::*;
use crate::utils::{WIDTH, HEIGHT, DEFAULT_LIFETIME, MUTATION_RATE};

#[derive(PartialEq)]
pub struct Organism {
    pub x: usize,
    pub y: usize,
    pub cells: Vec<(i32, i32, Cell)>,
    pub id: usize,
    pub energy: i32,
    pub lifetime: i32,
    pub killed: bool,

    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Organism {
    pub fn new(x: usize, y: usize, cells: Vec<(i32, i32, Cell)>, id: usize) -> Self {
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

        Organism { x, y, cells, id, energy: 0, lifetime: DEFAULT_LIFETIME, killed: false,
            min_x: min_x.abs() as usize, max_x: max_x as usize, min_y: min_y.abs() as usize, max_y: max_y as usize}
    }
    fn is_connected(&self) -> bool {
        if self.cells.is_empty() {
            return false;
        }

        let mut visited = vec![false; self.cells.len()];
        let mut queue = VecDeque::new();

        queue.push_back(0);
        visited[0] = true;
        let mut visited_count = 1;

        while let Some(index) = queue.pop_front() {
            let (x1, y1, _) = self.cells[index];

            for (i, (x2, y2, _)) in self.cells.iter().enumerate() {
                if !visited[i] && ((x1 - x2).abs() + (y1 - y2).abs() == 1) {
                    visited[i] = true;
                    queue.push_back(i);
                    visited_count += 1;
                }
            }
        }

        visited_count == self.cells.len()
    }
    pub fn child(&self, id: usize) -> Organism {
        let mut child = Organism { x: self.x, y: self.y, id, cells: self.cells.clone(), energy: 0, lifetime: DEFAULT_LIFETIME, killed: false,
            min_x: self.min_x, max_x: self.max_x, min_y: self.min_y, max_y: self.max_y};
        
        child.mutate();
        child
    }
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < MUTATION_RATE {
            let val = rng.gen::<f32>();
            if val < 0.33 {
                self.add_cell(Cell::random_cell());
            } else if val < 0.66 {
                self.remove_cell();
            } else {
                self.change_cell();
            }
        }
    }
    pub fn add_cell(&mut self, new_cell: Cell) {
        let mut rng = rand::thread_rng();
        if self.cells.is_empty() {
            return;
        }

        let &(base_x, base_y, _) = self.cells.choose(&mut rng).unwrap();

        let possible_positions = vec![
            (base_x + 1, base_y),
            (base_x - 1, base_y),
            (base_x, base_y + 1),
            (base_x, base_y - 1),
        ];

        let valid_positions: Vec<_> = possible_positions
            .into_iter()
            .filter(|&(x, y)| !self.cells.iter().any(|(cx, cy, _)| *cx == x && *cy == y))
            .collect();

        if let Some(&(new_x, new_y)) = valid_positions.choose(&mut rng) {
            self.cells.push((new_x, new_y, new_cell));
        }
    }
    pub fn remove_cell(&mut self) {
        let mut rng = rand::thread_rng();
        if self.cells.len() <= 1 {
            return; 
        }

        let original_cells = self.cells.clone();
        let remove_index = rng.gen_range(0..self.cells.len());

        self.cells.remove(remove_index);

        if !self.is_connected() {
            self.cells = original_cells;
        }
    }
    pub fn change_cell(&mut self) {
        let mut rng = rand::thread_rng();
        if self.cells.is_empty() {
            return;
        }

        let index = rng.gen_range(0..self.cells.len());
        self.cells[index].2 = Cell::random_cell();
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
            //grid.make_remains(self);
            self.killed = true;
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
                    grid.killer_activates(x, y, self.id);
                }
                Cell::Armor => {}
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
