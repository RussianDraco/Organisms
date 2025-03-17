use crate::organism::Organism;

use crate::{Direction, cell::Cell, organism_manager::SimData};
use macroquad::prelude::*;
use ::rand::{SeedableRng, Rng};
use ::rand::rngs::StdRng;

use crate::utils::*;//{WIDTH, HEIGHT, CELL_SIZE, PRODUCER_RATE}; 

enum CellContent {
    Empty,
    Food,
    Organism,
}

pub struct Grid {
    pub rng: StdRng,
    pub foods: [[bool; WIDTH]; HEIGHT],
    pub organs: [[Cell; WIDTH]; HEIGHT],
    pending_kill_coordinates: Vec<(usize, usize)>, // x, y
    pending_kill_killers: Vec<usize>, // id
    graphics_on: bool,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rng: StdRng::seed_from_u64(SEED+1),
            foods: [[false; WIDTH]; HEIGHT],
            organs: [[Cell::Empty; WIDTH]; HEIGHT],
            pending_kill_coordinates: Vec::new(),
            pending_kill_killers: Vec::new(),
            graphics_on: true,
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
        ((WIDTH as f32 * CELL_SIZE + MENU_WIDTH) as i32, (HEIGHT as f32 * CELL_SIZE) as i32)
    }

    pub fn update(&mut self, organisms: &mut Vec<Organism>) {
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
        if crate::utils::GRAPHICS {self.draw();}
    }

    fn get_cell_color(cell: &Cell) -> Color {
        match cell {
            Cell::Empty => DARKGRAY,
            Cell::Body => WHITE,
            Cell::Mouth => ORANGE,
            Cell::Producer => GREEN,
            Cell::Mover => LIGHTGRAY,
            Cell::Killer => RED,
            Cell::Armor => YELLOW,
            Cell::Eye(_) => PURPLE,
            Cell::Brain => PINK,
            //_ => DARKGRAY,
        }
    }

    pub fn update_sim_menu(&mut self, sim_data: &SimData) {
        draw_rectangle(WIDTH as f32 * CELL_SIZE, 0.0, MENU_WIDTH, MENU_HEIGHT, LIGHTGRAY);
        draw_rectangle(WIDTH as f32 * CELL_SIZE + CELL_SIZE, CELL_SIZE, MENU_WIDTH - 2.0 * CELL_SIZE, MENU_WIDTH - 2.0 * CELL_SIZE, WHITE);
        self.draw_success(sim_data.best_species.as_str());

        let mut text = format!("Organism #: {}", sim_data.organism_num);
        draw_text(&text, WIDTH as f32 * CELL_SIZE + CELL_SIZE * 2.0, MENU_HEIGHT / 1.75, 20.0, BLACK);
        text = format!("Hunger Deaths: {}", sim_data.hunger_death);
        draw_text(&text, WIDTH as f32 * CELL_SIZE + CELL_SIZE * 2.0, MENU_HEIGHT / 1.75 + 25.0, 20.0, BLACK);
        text = format!("Age Deaths: {}", sim_data.age_death);
        draw_text(&text, WIDTH as f32 * CELL_SIZE + CELL_SIZE * 2.0, MENU_HEIGHT / 1.75 + 50.0, 20.0, BLACK);

        let button_x = WIDTH as f32 * CELL_SIZE + CELL_SIZE * 2.0;
        let button_y = MENU_HEIGHT / 1.5 + 75.0;
        let button_width = if self.graphics_on {190.0} else {200.0};
        let button_height = 30.0;
        draw_rectangle(button_x, button_y, button_width, button_height, GRAY);
        draw_text(&format!("Toggle Graphics: {}", self.graphics_on), button_x + 5.0, button_y + 20.0, 20.0, BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x >= button_x && mouse_x <= button_x + button_width && mouse_y >= button_y && mouse_y <= button_y + button_height {
                self.graphics_on = !self.graphics_on;
            }
        }
    }

    fn draw_success(&self, success_org: &str) {
        let decoded_cells = Organism::decode_anatomy(success_org);
        if decoded_cells.is_empty() {
            return;
        }

        let min_x = decoded_cells.iter().map(|(x, _, _)| *x).min().unwrap_or(0);
        let max_x = decoded_cells.iter().map(|(x, _, _)| *x).max().unwrap_or(0);
        let min_y = decoded_cells.iter().map(|(_, y, _)| *y).min().unwrap_or(0);
        let max_y = decoded_cells.iter().map(|(_, y, _)| *y).max().unwrap_or(0);

        let organism_width = (max_x - min_x + 1) as f32 * CELL_SIZE;
        let organism_height = (max_y - min_y + 1) as f32 * CELL_SIZE;

        let center_x = WIDTH as f32 * CELL_SIZE + MENU_WIDTH / 2.0;
        let center_y = MENU_WIDTH / 2.0;

        let start_x = center_x - organism_width / 2.0;
        let start_y = center_y - organism_height / 2.0;

        for (dx, dy, cell) in decoded_cells.iter() {
            let x = start_x + (*dx - min_x) as f32 * CELL_SIZE;
            let y = start_y + (*dy - min_y) as f32 * CELL_SIZE;

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Grid::get_cell_color(cell));
        }
    }

    pub fn draw(&self) {
        if !self.graphics_on {return;}
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
