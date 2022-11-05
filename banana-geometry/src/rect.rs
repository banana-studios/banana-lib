use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub start: IVec2,
    pub size: IVec2,
}

impl Rect {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// Create a new rectangle from the given start and size.
    #[inline(always)]
    pub fn new(start: impl Size2d, size: impl Size2d) -> Self {
        Self { start: start.as_ivec2(), size: size.as_ivec2() }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: i32) -> Self {
        Self { start: IVec2::splat(v), size: IVec2::splat(v) }
    }

    /// Returns the rectangle's width
    #[must_use]
    pub fn width(&self) -> i32 {
        i32::abs(self.size.x - self.start.x)
    }

    /// Returns the rectangle's height
    #[must_use]
    pub fn height(&self) -> i32 {
        i32::abs(self.size.y - self.start.y)
    }

    /// Returns true if this overlaps with other
    #[must_use]
    pub fn intersect(&self, other: &Rect) -> bool {
        self.size.intersects(other.size)
    }

    /// Returns the center of the rectangle
    #[must_use]
    pub fn center(&self) -> IVec2 {
        IVec2::new(self.start.x + self.size.x / 2, self.start.y + self.size.y / 2)
    }

    /// Returns true if a point is inside the rectangle
    #[must_use]
    pub fn point_in_rect(&self, point: impl GridPoint) -> bool {
        self.size.point_in_bounds(point)
    }
}
