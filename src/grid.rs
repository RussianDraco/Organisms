use crate::organism::Organism;

use crate::{Direction, cell::Cell};
use macroquad::prelude::*;
use ::rand::prelude::*;

use crate::utils::*;//{WIDTH, HEIGHT, CELL_SIZE, PRODUCER_RATE}; 

enum CellContent {
    Empty,
    Food,
    Organism,
}

pub struct Grid {
    pub rng: ThreadRng,
    pub foods: [[bool; WIDTH]; HEIGHT],
    pub organs: [[Cell; WIDTH]; HEIGHT],
    pending_kill_coordinates: Vec<(usize, usize)>, // x, y
    pending_kill_killers: Vec<usize>, // id
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rng: thread_rng(),
            foods: [[false; WIDTH]; HEIGHT],
            organs: [[Cell::Empty; WIDTH]; HEIGHT],
            pending_kill_coordinates: Vec::new(),
            pending_kill_killers: Vec::new(),
        }
    }

    pub fn scatter_food(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.rng.gen::<f32>() < 0.1 {
                    self.foods[y][x] = true;
                }
            }
        }
    }

    pub fn produce_food(&mut self, x: usize, y: usize) {
        if self.rng.gen::<f32>() > PRODUCER_RATE {
            return;
        }

        self.foods[
            ((if self.rng.gen_bool(0.5) { 1 } else { -1 }) as i32 + y as i32).clamp(0, (HEIGHT - 1) as i32) as usize
        ][
            ((if self.rng.gen_bool(0.5) { 1 } else { -1 }) as i32 + x as i32).clamp(0, (WIDTH - 1) as i32) as usize
        ] = true;
    }

    pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }
    
        if self.organs[y][x] != Cell::Empty {
            return false;
        }
    
        true
    }
    fn cell_contents(&self, x: usize, y: usize) -> CellContent {
        if x >= WIDTH || y >= HEIGHT {
            return CellContent::Empty;
        }
    
        if self.organs[y][x] != Cell::Empty {
            return CellContent::Organism;
        }
    
        if self.foods[y][x] {
            return CellContent::Food;
        }
    
        CellContent::Empty
    }

    pub fn make_remains(&mut self, organism: &Organism) {
        for (dx, dy, _) in organism.cells.iter() {
            if self.rng.gen::<f32>() > DROP_FOOD_RATE {
                continue;
            }

            let x = organism.x as i32 + dx;
            let y = organism.y as i32 + dy;
            if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            self.foods[y][x] = true;
        }
    }

    pub fn killer_activates(&mut self, x: usize, y: usize, id: usize) {
        self.pending_kill_coordinates.push((x, y));
        self.pending_kill_killers.push(id);
    }

    pub fn get_eye_data(&self, x: usize, y: usize, dir: Direction) -> f32 {
        fn increment_pos(x: usize, y:usize, dir: Direction) -> (usize, usize) {
            match dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
                _ => (x, y),
            }
        }
        
        let mut data = 0.1;

        let mut depth = 0;

        let mut cx = x;
        let mut cy = y;

        loop {
            if cx == 0 || cy == 0 || cx >= WIDTH || cy >= HEIGHT {
                break;
            }

            (cx, cy) = increment_pos(cx, cy, dir);

            match self.cell_contents(cx, cy) {
                CellContent::Food => {
                    data = 0.5;
                    break;
                }
                CellContent::Organism => {
                    data = -1.0;
                    break;
                }
                _ => {}
            }

            depth+=1;
            if depth >= MAX_EYE_DIST {
                break;
            }
        }

        data
    }

    pub fn check_spawn(&self, organism: &Organism) -> bool {
        for (dx, dy, _) in organism.cells.iter() {
            let x = organism.x as i32 + dx;
            let y = organism.y as i32 + dy;
            if x < 0 || y < 0 {
                return false;
            }
            let x = x as usize;
            let y = y as usize;
            if !self.is_cell_empty(x, y) || x >= WIDTH || y >= HEIGHT {
                return false;
            }
        }
        true
    }

    pub fn mouth_eat(&mut self, x: usize, y: usize) -> bool {
        if self.foods[y][x] {
            self.foods[y][x] = false;
            return true;
        } else if x + 1 < WIDTH && self.foods[y][x + 1] {
            self.foods[y][x + 1] = false;
            return true;
        } else if x > 0 && self.foods[y][x - 1] {
            self.foods[y][x - 1] = false;
            return true;
        } else if y + 1 < HEIGHT && self.foods[y + 1][x] {
            self.foods[y + 1][x] = false;
            return true;
        } else if y > 0 && self.foods[y - 1][x] {
            self.foods[y - 1][x] = false;
            return true;
        }
        false
    }

    pub fn screen_size() -> (i32, i32) {
        ((WIDTH as f32 * CELL_SIZE) as i32, (HEIGHT as f32 * CELL_SIZE) as i32)
    }

    pub fn draw_organisms(&mut self, organisms: &mut Vec<Organism>) {
        self.organs = [[Cell::Empty; WIDTH]; HEIGHT];
        for organism in organisms.iter_mut() {
            for cell in organism.cells.iter() {
                let x = (organism.x as i32 + cell.0) as usize;
                let y = (organism.y as i32 + cell.1) as usize;

                if cell.2 != Cell::Armor {
                    for (i, (kill_x, kill_y)) in self.pending_kill_coordinates.iter().enumerate() {
                        if *kill_x <= 1 || *kill_y <= 1 {
                            if (x == *kill_x + 1 && y == *kill_y ||
                                x == *kill_x && y == *kill_y + 1) &&
                                self.pending_kill_killers[i] != organism.id {
                                organism.killed = true;
                                continue;
                            }
                        } else {
                            if (x == *kill_x + 1 && y == *kill_y ||
                                x == *kill_x - 1 && y == *kill_y ||
                                x == *kill_x && y == *kill_y + 1 ||
                                x == *kill_x && y == *kill_y - 1) &&
                                self.pending_kill_killers[i] != organism.id {
                                organism.killed = true;
                                continue;
                            }
                        } 
                    }
                }

                self.organs[y][x] = cell.2;
            }
        }
        self.pending_kill_coordinates.clear();
        self.pending_kill_killers.clear();
        self.draw();
    }

    pub fn draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut extra_rect: Direction = Direction::None;
                
                let color = match self.organs[y][x] {
                    Cell::Empty => {if self.foods[y][x] {BLUE} else {DARKGRAY}},
                    Cell::Body => WHITE,

                    Cell::Mouth => ORANGE,
                    Cell::Producer => GREEN,
                    Cell::Mover => LIGHTGRAY,
                    Cell::Killer => RED,
                    Cell::Armor => YELLOW,
                    Cell::Eye(eye_dir) => {
                        extra_rect = eye_dir.to_direction();
                        PURPLE},
                    Cell::Brain => PINK,
                };

                draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
                if extra_rect != Direction::None {
                    draw_line(CELL_SIZE * (x as f32 + 0.5), CELL_SIZE * (y as f32 + 0.5), 
                    CELL_SIZE * (x as f32 + 0.5 * (1.0 + extra_rect.x_offset())), 
                    CELL_SIZE * (y as f32 + 0.5 * (1.0 + extra_rect.y_offset())), 4.0, BLACK);
                }
            }
        }
    }
}
