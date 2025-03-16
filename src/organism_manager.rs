use crate::grid::Grid;
use crate::cell::{Cell};
use crate::organism::Organism;
use crate::utils::{HEIGHT, WIDTH};
use rand::Rng;

pub struct OrganismManager {
    pub organisms: Vec<Organism>,
    pub grid: Grid,
    pub frame: u128,
}

impl OrganismManager {
    pub fn new() -> Self {
        OrganismManager {
            organisms: Vec::new(),
            grid: Grid::new(),
            frame: 0,
        }
    }

    fn default_org() -> Vec<(i32, i32, Cell)> {
        vec![(-1, -1, Cell::Producer), (0, 0, Cell::Mouth), (1, 1, Cell::Producer)]
        //Organism::new(50, 50, vec![(0, -1, Cell::Mouth),(-1, 0, Cell::Producer), (0, 0, Cell::Mover), (1, 0, Cell::Producer),(-1, 1, Cell::Brain), (0, 1, Cell::Eye(EyeType::Down)), (1, 1, Cell::Brain),], 0)
    }

    pub fn init(&mut self) {
        self.grid.scatter_food();
        let mut rng = rand::thread_rng();
        for i in 0..10 {
            self.organisms.push(Organism::new(rng.gen_range(10..WIDTH - 10), rng.gen_range(10..HEIGHT - 10), OrganismManager::default_org(), i));
        }
    }

    pub fn assign_id(&self) -> usize {
        self.organisms.len()
    }

    pub fn draw_organisms(&mut self) {
        self.grid.draw_organisms(&mut self.organisms);
    }

    pub fn update(&mut self) {
        self.organisms.retain(|organism| {
            if organism.killed {
            self.grid.make_remains(organism);
            false
            } else {
            true
            }
        });
        
        let mut next_id = self.assign_id();

        let mut new_organisms = Vec::new();
        for organism in self.organisms.iter_mut() {
            organism.update(&mut self.grid);
            if organism.can_reproduce() {
                let mut new_org = organism.child(next_id); next_id += 1;
                new_org.random_offset();
                if self.grid.check_spawn(&new_org) {
                    new_organisms.push(new_org);
                    organism.consume_reproduction_energy();
                }
            }
        }
        self.organisms.extend(new_organisms);

        if self.organisms.is_empty() {
            self.grid.foods = [[false; WIDTH]; HEIGHT];
            self.init();
        }

        self.frame += 1;
        if self.frame % 100 == 0 {
            println!("Frame: {}", self.frame);
            println!("Organisms: {}", self.organisms.len());
        }
    }
}