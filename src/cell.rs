use rand::Rng;
use crate::Direction;

#[derive(Clone, Copy, PartialEq)]
pub enum EyeType {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Body,

    Mouth,
    Producer,
    Mover,
    Killer,
    Armor,
    Eye(EyeType),
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
            5 => Cell::Eye(Cell::random_eye_type()),
            6 => Cell::Brain,
            _ => Cell::Empty,
        }
    }

    pub fn random_eye_type() -> EyeType {
        match rand::thread_rng().gen_range(0..=3) {
            0 => EyeType::Up,
            1 => EyeType::Down,
            2 => EyeType::Left,
            3 => EyeType::Right,
            _ => EyeType::Up,
        }
    }
}

impl EyeType {
    pub fn to_direction(&self) -> Direction {
        match self {
            EyeType::Up => Direction::Up,
            EyeType::Down => Direction::Down,
            EyeType::Left => Direction::Left,
            EyeType::Right => Direction::Right,
        }
    }
}