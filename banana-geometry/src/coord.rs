use crate::prelude::*;

/// General purpose coordinate
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl GridPoint for Coord {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}
