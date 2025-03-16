pub struct Organism {
    pub x: usize,
    pub y: usize,
}

impl Organism {
    pub fn new(x: usize, y: usize) -> Self {
        Organism { x, y }
    }

    pub fn move_randomly(&mut self) {
        self.x = (self.x + 1) % 20; // Simple example movement
    }
}
