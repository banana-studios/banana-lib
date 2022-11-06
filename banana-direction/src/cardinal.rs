use crate::prelude::*;
use banana_grid::prelude::{Axis, GridPoint, IVec2};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

pub const NUM_CARDINAL_DIRECTIONS: usize = 4;
pub const ALL_CARDINAL_DIRECTION_BITMAP_RAW: u8 = (1 << Direction::North as usize)
    | (1 << Direction::East as usize)
    | (1 << Direction::South as usize)
    | (1 << Direction::West as usize);

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CardinalDirection {
    North = 0,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn from_unit_coord<P>(coord: P) -> Self
    where
        P: GridPoint + std::fmt::Debug,
    {
        match [coord.x(), coord.y()] {
            [1, 0] => CardinalDirection::East,
            [-1, 0] => CardinalDirection::West,
            [0, 1] => CardinalDirection::South,
            [0, -1] => CardinalDirection::North,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            CardinalDirection::North => Direction::North,
            CardinalDirection::East => Direction::East,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
        }
    }

    pub fn opposite(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }

    pub fn coord(self) -> IVec2 {
        match self {
            CardinalDirection::North => IVec2::new(0, -1),
            CardinalDirection::East => IVec2::new(1, 0),
            CardinalDirection::South => IVec2::new(0, 1),
            CardinalDirection::West => IVec2::new(-1, 0),
        }
    }

    pub fn left90(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }

    pub fn right90(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    pub fn left45(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::NorthWest,
            CardinalDirection::East => OrdinalDirection::NorthEast,
            CardinalDirection::South => OrdinalDirection::SouthEast,
            CardinalDirection::West => OrdinalDirection::SouthWest,
        }
    }

    pub fn right45(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::NorthEast,
            CardinalDirection::East => OrdinalDirection::SouthEast,
            CardinalDirection::South => OrdinalDirection::SouthWest,
            CardinalDirection::West => OrdinalDirection::NorthWest,
        }
    }

    pub fn left135(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::SouthWest,
            CardinalDirection::East => OrdinalDirection::NorthWest,
            CardinalDirection::South => OrdinalDirection::NorthEast,
            CardinalDirection::West => OrdinalDirection::SouthEast,
        }
    }

    pub fn right135(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::SouthEast,
            CardinalDirection::East => OrdinalDirection::SouthWest,
            CardinalDirection::South => OrdinalDirection::NorthWest,
            CardinalDirection::West => OrdinalDirection::NorthEast,
        }
    }

    pub fn axis(self) -> Axis {
        match self {
            CardinalDirection::East | CardinalDirection::West => Axis::X,
            CardinalDirection::North | CardinalDirection::South => Axis::Y,
        }
    }

    pub fn sign(self) -> i32 {
        match self {
            CardinalDirection::South | CardinalDirection::East => 1,
            CardinalDirection::North | CardinalDirection::West => -1,
        }
    }

    pub fn axis_and_sign(self) -> (Axis, i32) {
        match self {
            CardinalDirection::North => (Axis::Y, -1),
            CardinalDirection::East => (Axis::X, 1),
            CardinalDirection::South => (Axis::Y, 1),
            CardinalDirection::West => (Axis::X, -1),
        }
    }

    pub const fn all() -> CardinalDirectionIter {
        CardinalDirectionIter::new()
    }

    pub const fn all_directions() -> DirectionCardinalIter {
        DirectionCardinalIter::new()
    }

    pub fn combine(self, other: Self) -> Option<OrdinalDirection> {
        OrdinalDirection::from_cardinals(self, other)
    }
}

impl From<CardinalDirection> for [i32; 2] {
    fn from(c: CardinalDirection) -> [i32; 2] {
        use self::CardinalDirection::*;
        match c {
            East => [1, 0],
            South => [0, 1],
            West => [-1, 0],
            North => [0, -1],
        }
    }
}
impl From<CardinalDirection> for (i32, i32) {
    fn from(c: CardinalDirection) -> (i32, i32) {
        use self::CardinalDirection::*;
        match c {
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
            North => (0, -1),
        }
    }
}

#[cfg(feature = "rng")]
impl Distribution<CardinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
        let index = rng.gen_range(0..NUM_CARDINAL_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
