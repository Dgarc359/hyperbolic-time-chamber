use std::ops::{Add, Mul};

use crate::{Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];
    pub const CARDINAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    pub const ORDINAL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Vector { x: 0, y: -1 },
            Direction::NorthEast => Vector { x: 1, y: -1 },
            Direction::East => Vector { x: 1, y: 0 },
            Direction::SouthEast => Vector { x: 1, y: 1 },
            Direction::South => Vector { x: 0, y: 1 },
            Direction::SouthWest => Vector { x: -1, y: 1 },
            Direction::West => Vector { x: -1, y: 0 },
            Direction::NorthWest => Vector { x: -1, y: -1 },
        }
    }
}

impl Mul<i32> for Direction {
    type Output = Vector;
    fn mul(self, rhs: i32) -> Self::Output {
        let us_as_a_vector_woot_woot: Vector = self.into();
        us_as_a_vector_woot_woot * rhs
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Self::Output {
        let rhs_as_a_vector_woot_woot: Vector = rhs.into();
        self + rhs_as_a_vector_woot_woot
    }
}

impl Add<Direction> for Vector {
    type Output = Vector;
    fn add(self, rhs: Direction) -> Self::Output {
        let rhs_as_vector: Vector = rhs.into();
        self + rhs_as_vector
    }
}
