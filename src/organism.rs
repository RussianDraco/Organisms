use crate::{brain::Brain, cell::Cell, grid, utils::FOOD_BENEFIT, Direction};
use std::collections::VecDeque;
use rand::{rngs::StdRng, Rng, seq::SliceRandom};
use crate::utils::{WIDTH, HEIGHT, LIFETIME_MULTIPLIER, MUTATION_RATE, HUNGER_RATE, REPRODUCTION_ENEGRGY_MULTIPLER};

pub struct Organism {
    pub x: usize,
    pub y: usize,
    pub cells: Vec<(i32, i32, Cell)>,
    pub brain: Option<Brain>,
    pub id: usize,
    pub energy: i32,
    pub lifetime: i32,
    pub satiety: f32,
    pub killed: bool,

    cells_len: usize,
    eye_data: Vec<f32>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Organism {
    pub fn new(x: usize, y: usize, cells: Vec<(i32, i32, Cell)>, id: usize, rng: &mut StdRng) -> Self {
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

        let brain: Option<Brain> = if let Some((eye_num, brain_num)) = Organism::brain_quality(&cells) {
            Some(Brain::new(eye_num, brain_num, rng))
        } else {
            None
        };

        let lifetime = Organism::lifetime_len(&cells);
        let cells_len = cells.len();

        Organism { x, y, cells, brain, id, energy: 0, lifetime, satiety: 1.0, killed: false,
            cells_len, eye_data: Vec::new(), min_x: min_x.abs() as usize, max_x: max_x as usize, min_y: min_y.abs() as usize, max_y: max_y as usize}
    }
    fn lifetime_len(cells: &Vec<(i32, i32, Cell)>) -> i32 {return cells.len() as i32 * LIFETIME_MULTIPLIER;}
    fn brain_quality(cells: &Vec<(i32, i32, Cell)>) -> Option<(usize, usize)> {
        let mut has_mover = false;
        let mut eye_num = 0;
        let mut brain_num = 0;
        for (_dx, _dy, cell) in cells.iter() {
            if let Cell::Eye(_) = cell {
                eye_num += 1;
            } else if *cell == Cell::Mover {
                has_mover = true;
            } else if *cell == Cell::Brain {
                brain_num += 1;
            }
        }
        if eye_num > 0 && has_mover {
            Some((eye_num, brain_num))
        } else {
            None
        }
    }
    pub fn body_range(&self) -> (usize, usize) {(self.max_x + self.min_x + 1, self.max_y - self.min_y + 1)}
    pub fn is_connected(&self) -> bool {
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
                if !visited[i] {
                    let dx = (x1 - x2).abs();
                    let dy = (y1 - y2).abs();

                    if dx <= 1 && dy <= 1 {
                        visited[i] = true;
                        queue.push_back(i);
                        visited_count += 1;
                    }
                }
            }
        }

        visited_count == self.cells.len()
    }
    pub fn child(&self, id: usize, rng: &mut StdRng) -> Organism {
        let lifetime = Organism::lifetime_len(&self.cells);
        let mut child = Organism { x: self.x, y: self.y, id, cells: self.cells.clone(), brain: None, energy: 0, lifetime, satiety: 1.0, killed: false,
            cells_len: self.cells_len, eye_data: Vec::new(), min_x: self.min_x, max_x: self.max_x, min_y: self.min_y, max_y: self.max_y};
        
        child.mutate(rng);

        let child_brain = if let Some((eye_num, brain_num)) = Organism::brain_quality(&child.cells) {
            if let Some(brain) = &self.brain {
                Some(brain.child_brain(brain_num, rng))
            } else {
                Some(Brain::new(eye_num, brain_num, rng))
            }
        } else {
            None
        };

        child.brain =  child_brain;
        
        child
    }
    pub fn mutate(&mut self, rng: &mut StdRng) {
        if rng.gen::<f32>() < MUTATION_RATE {
            let val = rng.gen::<f32>();
            if val < 0.33 {
                self.change_cell(rng);
            } else if val < 0.66 && self.cells_len > 1 {
                self.remove_cell(rng);
                self.lifetime -= LIFETIME_MULTIPLIER;
                self.cells_len -= 1;
            } else {
                self.add_cell(Cell::random_cell(rng), rng);
                self.lifetime += LIFETIME_MULTIPLIER;
                self.cells_len += 1;
            }
        }
    }
    pub fn add_cell(&mut self, new_cell: Cell, rng: &mut StdRng) {
        if self.cells.is_empty() {
            return;
        }

        let &(base_x, base_y, _) = self.cells.choose(rng).unwrap();

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

        if let Some(&(new_x, new_y)) = valid_positions.choose(rng) {
            self.cells.push((new_x, new_y, new_cell));
        }
    }
    pub fn remove_cell(&mut self, rng: &mut StdRng) {
        if self.cells_len <= 1 {
            return; 
        }

        let original_cells = self.cells.clone();
        let remove_index = rng.gen_range(0..self.cells_len);

        self.cells.remove(remove_index);

        if !self.is_connected() {
            self.cells = original_cells;
        }
    }
    pub fn change_cell(&mut self, rng: &mut StdRng) {
        if self.cells.is_empty() {
            return;
        }

        let index = rng.gen_range(0..self.cells_len);
        self.cells[index].2 = Cell::random_cell(rng);
    }
    
    pub fn decode_anatomy(encoded: &str) -> Vec<(i32, i32, Cell)> {
        let mut cells = Vec::new();
        let parts: Vec<&str> = encoded.split(',').collect();

        let mut i = 0;
        while i + 2 < parts.len() {
            if let (Ok(dx), Ok(dy)) = (parts[i].parse::<i32>(), parts[i + 1].parse::<i32>()) {
                if let Some(cell) = Cell::from_string(parts[i + 2]) { // Ensure `Cell::from_string()` exists
                    cells.push((dx, dy, cell));
                }
            }
            i += 3;
        }

        cells
    }
    pub fn encode_anatomy(&self) -> String {
        let mut anatomy = String::new();
        for (dx, dy, cell) in &self.cells {
            anatomy.push_str(&format!("{},{},", dx, dy));
            anatomy.push_str(&format!("{:?},", cell));
        }
        anatomy
    }

    pub fn random_offset(&mut self, rng: &mut StdRng) {
        let rang_tup = self.body_range();
        let x_range = (4 + rang_tup.0) as i32;
        let y_range = (4 + rang_tup.1) as i32;
        let dx = rng.gen_range(-x_range..=x_range);
        let dy = rng.gen_range(-y_range..=y_range);
        self.x = (self.x as i32 + dx) as usize;
        self.y = (self.y as i32 + dy) as usize;
    }

    pub fn rotate(&mut self, clockwise: bool, grid: &grid::Grid) {
        if self.cells.is_empty() {
            return;
        }

        let sum_x: i32 = self.cells.iter().map(|(dx, _, _)| dx).sum();
        let sum_y: i32 = self.cells.iter().map(|(_, dy, _)| dy).sum();
        let center_x = sum_x / self.cells.len() as i32;
        let center_y = sum_y / self.cells.len() as i32;

        let mut new_cells = Vec::new();
        for &(dx, dy, cell) in &self.cells {
            let new_dx;
            let new_dy;

            if clockwise {
                new_dx = center_x + (dy - center_y);
                new_dy = center_y - (dx - center_x);
            } else {
                new_dx = center_x - (dy - center_y);
                new_dy = center_y + (dx - center_x);
            }

            new_cells.push((new_dx, new_dy, cell));
        }

        for &(dx, dy, _) in &new_cells {
            let check_x = (self.x as i32 + dx) as usize;
            let check_y = (self.y as i32 + dy) as usize;
            if check_x >= WIDTH || check_y >= HEIGHT || !grid.is_cell_empty(check_x, check_y) {
                return;
            }
        }

        self.cells = new_cells;
    }
    pub fn move_dir(&mut self, dir: Direction, grid: &grid::Grid) {
        if dir == Direction::None {
            return;
        }
        
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => (0, 0),
        };
        self.move_org(dx, dy, grid);
    }
    pub fn move_org(&mut self, dx: i32, dy: i32, grid: &grid::Grid) {
        if dx == 0 && dy == 0 {
            return;
        }
        
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
    pub fn random_movement(&mut self, grid: &grid::Grid, rng: &mut StdRng) {
        if rng.gen::<f32>() < 0.5 {
            self.rotate(rng.gen::<bool>(), grid);
            return;
        } else {
            let dir = Direction::random_direction(rng);
            self.move_dir(dir, grid);
        }
    }
    pub fn can_reproduce(&self) -> bool {
        self.energy as f32 >= self.cells_len as f32 * REPRODUCTION_ENEGRGY_MULTIPLER
    }
    pub fn consume_reproduction_energy(&mut self) {
        self.energy -= self.cells_len as i32;
    }

    pub fn update(&mut self, grid: &mut grid::Grid, rng: &mut StdRng) -> bool {
        self.lifetime -= 1;
        self.satiety -= HUNGER_RATE * self.cells_len as f32;
        if self.lifetime <= 0 {
            self.killed = true;
            return false;
        }
        if self.satiety <= 0.0 {
            self.killed = true;
            return false;
        }

        self.eye_data.clear();

        let mut will_move: bool = false;
        for (dx, dy, cell) in self.cells.iter() {
            let x = (self.x as i32 + dx) as usize;
            let y = (self.y as i32 + dy) as usize;
            match cell {
                Cell::Mouth => {
                    if grid.mouth_eat(x, y) {
                        self.energy += 1;
                        self.satiety += FOOD_BENEFIT;
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
                Cell::Eye(eye_type) => {
                    self.eye_data.push(grid.get_eye_data(x, y, (*eye_type).to_direction()));
                }
                Cell::Brain => {}
                _ => {}
            }
        }

        if will_move {
            if let Some(ref mut brain) = self.brain {
                let dir = brain.process_input(self.eye_data.clone());
                self.move_dir(dir, grid);
            } else {
                self.random_movement(grid, rng);
            }
        }
        
        true
    }
}
