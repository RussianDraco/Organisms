use crate::grid::Grid;
use crate::cell::Cell;
use crate::organism::Organism;
use crate::utils::{HEIGHT, WIDTH};
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::collections::HashMap;

pub struct SimData {
    pub frame: u128,
    pub organism_num: usize,
    pub best_species: String,
    pub hunger_death: usize,
    pub age_death: usize,
}
impl SimData {
    pub fn new() -> Self {
        SimData {
            frame: 0,
            organism_num: 0,
            best_species: String::new(),
            hunger_death: 0,
            age_death: 0,
        }
    }
}

pub struct OrganismManager {
    pub organisms: Vec<Organism>,
    pub grid: Grid,
    pub frame: u128,
    pub rng: StdRng,
    
    species_success: HashMap<String, usize>,
    sim_data: SimData
}

impl OrganismManager {
    pub fn new() -> Self {
        OrganismManager {
            organisms: Vec::new(),
            grid: Grid::new(),
            frame: 0,
            species_success: HashMap::new(),
            rng: StdRng::seed_from_u64(crate::utils::SEED),

            sim_data: SimData::new(),
        }
    }

    fn default_org() -> Vec<(i32, i32, Cell)> {
        vec![(-1, -1, Cell::Mover), (0, 0, Cell::Mouth), (1, 1, Cell::Producer)]
        //Organism::new(50, 50, vec![(0, -1, Cell::Mouth),(-1, 0, Cell::Producer), (0, 0, Cell::Mover), (1, 0, Cell::Producer),(-1, 1, Cell::Brain), (0, 1, Cell::Eye(EyeType::Down)), (1, 1, Cell::Brain),], 0)
    }

    pub fn init(&mut self) {
        //self.grid.scatter_food();
        for i in 0..10 {
            self.organisms.push(Organism::new(self.rng.gen_range(10..WIDTH - 10), self.rng.gen_range(10..HEIGHT - 10), OrganismManager::default_org(), i, &mut self.rng));
        }
    }

    pub fn assign_id(&self) -> usize {
        self.organisms.len()
    }

    pub fn update(&mut self) {
        self.grid.update(&mut self.organisms);
        self.organisms.retain(|organism| {
            if organism.killed {
            if organism.lifetime <= 0 {
                self.sim_data.age_death += 1;
            } else {
                self.sim_data.hunger_death += 1;
            }
            self.grid.make_remains(organism);
            false
            } else {
            true
            }
        });
        
        let mut next_id = self.assign_id();

        let mut new_organisms = Vec::new();
        for organism in self.organisms.iter_mut() {
            organism.update(&mut self.grid, &mut self.rng);
            if organism.can_reproduce() {
                let mut new_org = organism.child(next_id, &mut self.rng); next_id += 1;
                new_org.random_offset(&mut self.rng);
                if self.grid.check_spawn(&new_org) {
                    new_organisms.push(new_org);
                    self.species_success.insert(organism.encode_anatomy(), self.species_success.get(&organism.encode_anatomy()).unwrap_or(&0) + 1);
                }
                organism.consume_reproduction_energy();
            }
        }
        self.organisms.extend(new_organisms);

        if self.organisms.is_empty() {
            //self.grid.foods = [[false; WIDTH]; HEIGHT];
            self.init();
        }

        if let Some((species, _success)) = self.species_success.iter().max_by_key(|entry| entry.1) {
            self.sim_data.best_species = species.clone();
        }
        
        if crate::utils::GRAPHICS {self.grid.update_sim_menu(&self.sim_data);}

        self.frame += 1;
        self.sim_data.organism_num = self.organisms.len();
    }
}