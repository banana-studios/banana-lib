use crate::prelude::*;

/// A size cannot be created which would contain un-addressable cells.
/// That is, the maximum size has a width and height of one greater than the maximum `i32`.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    x: u32,
    y: u32,
}

impl Size2d for Size {
    fn width(&self) -> u32 {
        self.x
    }

    fn height(&self) -> u32 {
        self.y
    }
}
