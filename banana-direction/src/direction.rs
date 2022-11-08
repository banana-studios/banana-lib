use crate::prelude::*;
use banana_grid::prelude::{GridPoint, IVec2};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

pub const NUM_DIRECTIONS: usize = 8;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DirectionType {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

impl Direction {
    pub fn from_unit_coord<P>(coord: P) -> Self
    where
        P: GridPoint + std::fmt::Debug,
    {
        match [coord.x(), coord.y()] {
            [1, 0] => Direction::East,
            [-1, 0] => Direction::West,
            [0, 1] => Direction::South,
            [0, -1] => Direction::North,
            [1, 1] => Direction::SouthEast,
            [1, -1] => Direction::NorthEast,
            [-1, 1] => Direction::SouthWest,
            [-1, -1] => Direction::NorthWest,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub fn coord(self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, -1),
            Direction::NorthEast => IVec2::new(1, -1),
            Direction::East => IVec2::new(1, 0),
            Direction::SouthEast => IVec2::new(1, 1),
            Direction::South => IVec2::new(0, 1),
            Direction::SouthWest => IVec2::new(-1, 1),
            Direction::West => IVec2::new(-1, 0),
            Direction::NorthWest => IVec2::new(-1, -1),
        }
    }

    pub fn left90(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        }
    }

    pub fn right90(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    pub fn left45(self) -> Direction {
        match self {
            Direction::North => Direction::NorthWest,
            Direction::NorthEast => Direction::North,
            Direction::East => Direction::NorthEast,
            Direction::SouthEast => Direction::East,
            Direction::South => Direction::SouthEast,
            Direction::SouthWest => Direction::South,
            Direction::West => Direction::SouthWest,
            Direction::NorthWest => Direction::West,
        }
    }

    pub fn right45(self) -> Direction {
        match self {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }

    pub fn left135(self) -> Direction {
        match self {
            Direction::North => Direction::SouthWest,
            Direction::NorthEast => Direction::West,
            Direction::East => Direction::NorthWest,
            Direction::SouthEast => Direction::North,
            Direction::South => Direction::NorthEast,
            Direction::SouthWest => Direction::East,
            Direction::West => Direction::SouthEast,
            Direction::NorthWest => Direction::South,
        }
    }

    pub fn right135(self) -> Direction {
        match self {
            Direction::North => Direction::SouthEast,
            Direction::NorthEast => Direction::South,
            Direction::East => Direction::SouthWest,
            Direction::SouthEast => Direction::West,
            Direction::South => Direction::NorthWest,
            Direction::SouthWest => Direction::North,
            Direction::West => Direction::NorthEast,
            Direction::NorthWest => Direction::East,
        }
    }

    pub const fn bitmap_raw(self) -> u8 {
        1 << self as usize
    }

    pub const fn bitmap(self) -> DirectionBitmap {
        DirectionBitmap::new(self.bitmap_raw())
    }

    pub fn is_cardinal(self) -> bool {
        matches!(self, Direction::North | Direction::East | Direction::South | Direction::West)
    }

    pub fn is_ordinal(self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }

    pub fn typ(self) -> DirectionType {
        match self {
            Direction::North => DirectionType::Cardinal(CardinalDirection::North),
            Direction::NorthEast => DirectionType::Ordinal(OrdinalDirection::NorthEast),
            Direction::East => DirectionType::Cardinal(CardinalDirection::East),
            Direction::SouthEast => DirectionType::Ordinal(OrdinalDirection::SouthEast),
            Direction::South => DirectionType::Cardinal(CardinalDirection::South),
            Direction::SouthWest => DirectionType::Ordinal(OrdinalDirection::SouthWest),
            Direction::West => DirectionType::Cardinal(CardinalDirection::West),
            Direction::NorthWest => DirectionType::Ordinal(OrdinalDirection::NorthWest),
        }
    }

    pub fn cardinal(self) -> Option<CardinalDirection> {
        match self {
            Direction::North => Some(CardinalDirection::North),
            Direction::East => Some(CardinalDirection::East),
            Direction::South => Some(CardinalDirection::South),
            Direction::West => Some(CardinalDirection::West),
            _ => None,
        }
    }

    pub fn ordinal(self) -> Option<OrdinalDirection> {
        match self {
            Direction::NorthEast => Some(OrdinalDirection::NorthEast),
            Direction::SouthEast => Some(OrdinalDirection::SouthEast),
            Direction::SouthWest => Some(OrdinalDirection::SouthWest),
            Direction::NorthWest => Some(OrdinalDirection::NorthWest),
            _ => None,
        }
    }

    pub const fn all() -> DirectionIter {
        DirectionIter::new()
    }
}

impl From<Direction> for [i32; 2] {
    fn from(d: Direction) -> [i32; 2] {
        use self::Direction::*;
        match d {
            North => [0, -1],
            East => [1, 0],
            South => [0, 1],
            West => [-1, 0],
            NorthWest => [-1, -1],
            NorthEast => [1, -1],
            SouthEast => [1, 1],
            SouthWest => [-1, 1],
        }
    }
}
impl From<Direction> for (i32, i32) {
    fn from(d: Direction) -> (i32, i32) {
        use self::Direction::*;
        match d {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
    }
}

#[cfg(feature = "rng")]
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let index = rng.gen_range(0..NUM_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
