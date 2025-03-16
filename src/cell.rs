#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Food,
    Body,

    Mouth,
    Producer,
    Mover,
    Killer,
    Armor,
    Eye,
    Brain,
}