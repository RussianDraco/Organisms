use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    //Body,

    Mouth,
    Producer,
    Mover,
    Killer,
    Armor,
    Eye,
    Brain,
}

impl Cell {
    pub fn random_cell() -> Cell {
        match rand::thread_rng().gen_range(0..=6) {
            0 => Cell::Mouth,
            1 => Cell::Producer,
            2 => Cell::Mover,
            3 => Cell::Killer,
            4 => Cell::Armor,
            5 => Cell::Eye,
            6 => Cell::Brain,
            _ => Cell::Empty,
        }
    }
}