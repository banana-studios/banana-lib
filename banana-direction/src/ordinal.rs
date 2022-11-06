use crate::prelude::*;
use banana_grid::prelude::{GridPoint, IVec2};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

pub const NUM_ORDINAL_DIRECTIONS: usize = 4;
pub const ALL_ORDINAL_DIRECTION_BITMAP_RAW: u8 = (1 << Direction::NorthEast as usize)
    | (1 << Direction::SouthEast as usize)
    | (1 << Direction::SouthWest as usize)
    | (1 << Direction::NorthWest as usize);

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum OrdinalDirection {
    NorthEast = 0,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl OrdinalDirection {
    pub fn from_unit_coord<P>(coord: P) -> Self
    where
        P: GridPoint + std::fmt::Debug,
    {
        match [coord.x(), coord.y()] {
            [1, 1] => OrdinalDirection::SouthEast,
            [1, -1] => OrdinalDirection::NorthEast,
            [-1, 1] => OrdinalDirection::SouthWest,
            [-1, -1] => OrdinalDirection::NorthWest,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            OrdinalDirection::NorthEast => Direction::NorthEast,
            OrdinalDirection::SouthEast => Direction::SouthEast,
            OrdinalDirection::SouthWest => Direction::SouthWest,
            OrdinalDirection::NorthWest => Direction::NorthWest,
        }
    }

    pub fn opposite(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthEast,
        }
    }

    pub fn coord(self) -> IVec2 {
        match self {
            OrdinalDirection::NorthEast => IVec2::new(1, -1),
            OrdinalDirection::SouthEast => IVec2::new(1, 1),
            OrdinalDirection::SouthWest => IVec2::new(-1, 1),
            OrdinalDirection::NorthWest => IVec2::new(-1, -1),
        }
    }

    pub fn left90(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthEast,
            OrdinalDirection::SouthWest => OrdinalDirection::SouthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthWest,
        }
    }

    pub fn right90(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthEast,
            OrdinalDirection::SouthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthWest,
            OrdinalDirection::NorthWest => OrdinalDirection::NorthEast,
        }
    }

    pub fn left45(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::North,
            OrdinalDirection::SouthEast => CardinalDirection::East,
            OrdinalDirection::SouthWest => CardinalDirection::South,
            OrdinalDirection::NorthWest => CardinalDirection::West,
        }
    }

    pub fn right45(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::East,
            OrdinalDirection::SouthEast => CardinalDirection::South,
            OrdinalDirection::SouthWest => CardinalDirection::West,
            OrdinalDirection::NorthWest => CardinalDirection::North,
        }
    }

    pub fn left135(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::West,
            OrdinalDirection::SouthEast => CardinalDirection::North,
            OrdinalDirection::SouthWest => CardinalDirection::East,
            OrdinalDirection::NorthWest => CardinalDirection::South,
        }
    }

    pub fn right135(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::South,
            OrdinalDirection::SouthEast => CardinalDirection::West,
            OrdinalDirection::SouthWest => CardinalDirection::North,
            OrdinalDirection::NorthWest => CardinalDirection::East,
        }
    }

    pub fn from_cardinals(a: CardinalDirection, b: CardinalDirection) -> Option<Self> {
        match a {
            CardinalDirection::North => match b {
                CardinalDirection::East => Some(OrdinalDirection::NorthEast),
                CardinalDirection::West => Some(OrdinalDirection::NorthWest),
                _ => None,
            },
            CardinalDirection::East => match b {
                CardinalDirection::North => Some(OrdinalDirection::NorthEast),
                CardinalDirection::South => Some(OrdinalDirection::SouthEast),
                _ => None,
            },
            CardinalDirection::South => match b {
                CardinalDirection::East => Some(OrdinalDirection::SouthEast),
                CardinalDirection::West => Some(OrdinalDirection::SouthWest),
                _ => None,
            },
            CardinalDirection::West => match b {
                CardinalDirection::North => Some(OrdinalDirection::NorthWest),
                CardinalDirection::South => Some(OrdinalDirection::SouthWest),
                _ => None,
            },
        }
    }

    pub fn to_cardinals(self) -> (CardinalDirection, CardinalDirection) {
        use self::CardinalDirection::*;
        use self::OrdinalDirection::*;
        match self {
            NorthEast => (North, East),
            SouthEast => (East, South),
            SouthWest => (South, West),
            NorthWest => (West, North),
        }
    }

    pub fn cardinal_bitmap(self) -> DirectionBitmap {
        let (a, b) = self.to_cardinals();
        a.direction().bitmap() | b.direction().bitmap()
    }

    pub const fn all() -> OrdinalDirectionIter {
        OrdinalDirectionIter::new()
    }

    pub const fn all_directions() -> DirectionOrdinalIter {
        DirectionOrdinalIter::new()
    }
}

impl From<OrdinalDirection> for [i32; 2] {
    fn from(o: OrdinalDirection) -> [i32; 2] {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => [-1, -1],
            NorthEast => [1, -1],
            SouthEast => [1, 1],
            SouthWest => [-1, 1],
        }
    }
}
impl From<OrdinalDirection> for (i32, i32) {
    fn from(o: OrdinalDirection) -> (i32, i32) {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
    }
}

#[cfg(feature = "rng")]
impl Distribution<OrdinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OrdinalDirection {
        let index = rng.gen_range(0..NUM_ORDINAL_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
