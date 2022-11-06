use crate::{prelude::*, size_2d::Size2d};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

    #[inline(always)]
    pub fn new_coord<P>(self, this_axis: i32, other_axis: i32) -> IVec2
    where
        P: GridPoint,
    {
        match self {
            Axis::X => P::new(this_axis, other_axis),
            Axis::Y => P::new(other_axis, this_axis),
        }
    }

    pub fn try_new_size(
        self,
        this_axis: u32,
        other_axis: u32,
    ) -> Result<UVec2, DimensionTooLargeForSize> {
        match self {
            Axis::X => UVec2::try_new(this_axis, other_axis),
            Axis::Y => UVec2::try_new(other_axis, this_axis),
        }
    }

    #[inline]
    pub fn new_size(self, this_axis: u32, other_axis: u32) -> UVec2 {
        match self {
            Axis::X => UVec2::new(this_axis, other_axis),
            Axis::Y => UVec2::new(other_axis, this_axis),
        }
    }

    #[inline]
    pub fn size<S>(self, size: S) -> u32
    where
        S: Size2d,
    {
        match self {
            Axis::X => size.width(),
            Axis::Y => size.height(),
        }
    }
}
