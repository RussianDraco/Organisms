use rand::Rng;
use rand::rngs::StdRng;
use crate::Direction;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EyeType {
    Up,
    Down,
    Left,
    Right,
}

impl EyeType {
    /// Converts a string to an `EyeType`, returns `None` if invalid.
    pub fn from_string(s: &str) -> Option<EyeType> {
        match s {
            "Up" => Some(EyeType::Up),
            "Down" => Some(EyeType::Down),
            "Left" => Some(EyeType::Left),
            "Right" => Some(EyeType::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub fn from_string(s: &str) -> Option<Cell> {
        match s {
            "Mouth" => Some(Cell::Mouth),
            "Producer" => Some(Cell::Producer),
            "Mover" => Some(Cell::Mover),
            "Killer" => Some(Cell::Killer),
            "Armor" => Some(Cell::Armor),
            "Brain" => Some(Cell::Brain),
            _ if s.starts_with("Eye(") && s.ends_with(")") => {
                let inner = &s[4..s.len() - 1]; // Extract inner part of "Eye(Type)"
                EyeType::from_string(inner).map(Cell::Eye)
            }
            _ => None,
        }
    }

    pub fn random_cell(rng: &mut StdRng) -> Cell {
        match rng.gen_range(0..=6) {
            0 => Cell::Mouth,
            1 => Cell::Producer,
            2 => Cell::Mover,
            3 => Cell::Killer,
            4 => Cell::Armor,
            5 => Cell::Eye(Cell::random_eye_type(rng)),
            6 => Cell::Brain,
            _ => Cell::Empty,
        }
    }

    pub fn random_eye_type(rng: &mut StdRng) -> EyeType {
        match rng.gen_range(0..=3) {
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