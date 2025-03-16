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