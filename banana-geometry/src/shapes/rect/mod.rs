use banana_grid::prelude::{GridPoint, IVec2, Size2d};
use std::collections::HashSet;

mod arithmitic;
mod iter;

pub use arithmitic::*;
pub use iter::*;

pub enum GridCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A rectangle defined by two opposite corners.
///
/// The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
/// stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant
/// must be upheld by the user when directly assigning the fields, otherwise some methods
/// produce invalid results. It is generally recommended to use one of the constructor
/// methods instead, which will ensure this invariant is met, unless you already have
/// the minimum and maximum corners.
#[repr(C)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    /// The minimum corner point of the rect.
    pub min: IVec2,
    /// The maximum corner point of the rect.
    pub max: IVec2,
    // /// The size of the rect.
    // pub size: IVec2,
}

impl Rect {
    /// Create a new rectangle from top corner and size.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    #[inline]
    pub fn new<S>(top_corner: S, size: S) -> Self
    where
        S: Size2d,
    {
        let top_corner = top_corner.as_ivec2();
        Self::from_corners(top_corner, top_corner + size.as_ivec2())
    }

    /// Create a new rectangle from two corner points.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    #[inline]
    pub fn from_corners<S>(top_corner: S, bottom_corner: S) -> Self
    where
        S: Size2d,
    {
        let top_corner = top_corner.as_ivec2();
        let bottom_corner = bottom_corner.as_ivec2();
        Rect { min: top_corner, max: bottom_corner }
    }

    /// Create a new rectangle from its center and size.
    ///
    /// # Panics
    ///
    /// This method panics if any of the components of the size is negative.
    #[inline]
    pub fn from_center_size<S>(origin: S, size: S) -> Self
    where
        S: Size2d,
    {
        let size = size.as_ivec2();
        let origin = origin.as_ivec2();

        assert!(size.cmpge(IVec2::ZERO).all());

        let half_size = size / 2;
        Self::from_center_half_size(origin, half_size)
    }

    /// Create a new rectangle from its center and half-size.
    ///
    /// # Panics
    ///
    /// This method panics if any of the components of the half-size is negative.
    #[inline]
    pub fn from_center_half_size<S>(origin: S, half_size: S) -> Self
    where
        S: Size2d,
    {
        let origin = origin.as_ivec2();
        let half_size = half_size.as_ivec2();

        assert!(half_size.cmpge(IVec2::ZERO).all());
        Self { min: origin - half_size, max: origin + half_size }
    }

    /// Check if the rectangle is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.min.cmpge(self.max).any()
    }

    /// Rectangle width (max.x - min.x).
    #[inline]
    #[must_use]
    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    /// Rectangle height (max.y - min.y).
    #[inline]
    #[must_use]
    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    /// Rectangle size.
    #[inline]
    #[must_use]
    pub fn size(&self) -> IVec2 {
        self.max - self.min
    }

    /// Rectangle half-size.
    #[inline]
    #[must_use]
    pub fn half_size(&self) -> IVec2 {
        self.size() / 2
    }

    /// The center point of the rectangle.
    #[inline]
    #[must_use]
    pub fn center(&self) -> IVec2 {
        (self.min + self.max) / 2
    }

    /// Check if a point lies within this rectangle, inclusive of its edges.
    #[inline]
    pub fn contains<P>(&self, point: P) -> bool
    where
        P: GridPoint,
    {
        let point = point.as_ivec2();
        (point.cmpge(self.min) & point.cmple(self.max)).all()
    }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    #[must_use]
    pub fn intersects(&self, other: Rect) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()

        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Grab the corner of the rectangle
    #[inline]
    #[must_use]
    pub fn corner(&self, corner: &GridCorner) -> IVec2 {
        let [w, h] = (self.size() / 2).to_array();
        self.center()
            + IVec2::from(match corner {
                GridCorner::TopLeft => [-w, h],
                GridCorner::TopRight => [w, h],
                GridCorner::BottomLeft => [-w, -h],
                GridCorner::BottomRight => [w, -h],
            })
    }

    /// Gets a set of all tiles in the rectangle
    #[must_use]
    #[inline]
    pub fn point_set(&self) -> HashSet<IVec2> {
        let mut result = HashSet::new();
        for y in self.min.y..self.max.y {
            for x in self.min.x..self.max.x {
                result.insert(IVec2::new(x, y));
            }
        }
        result
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(IVec2),
    {
        RectPointIter::new(self.min, self.max).for_each(f);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use banana_grid::prelude::IVec2;
    use banana_utils::Canvas;

    #[test]
    fn iter() {
        let ret = Rect::from_corners(IVec2::new(0, 0), IVec2::new(3, 3));
        let mut canvas = Canvas::new([6, 6]);
        RectPointIter::from(ret).for_each(|p| {
            canvas.put(p, '*');
        });
        canvas.print();
    }

    #[test]
    fn big() {
        let rect = Rect::from_corners([2, 2], [8, 8]);
        let mut canvas = Canvas::new([10, 10]);

        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    fn test_dimensions() {
        let rect = Rect::new([0, 0], [10, 10]);
        assert!(rect.width() == 10);
        assert!(rect.height() == 10);
    }

    #[test]
    fn test_add() {
        let rect = Rect::from_corners([0, 0], [10, 10]) + Rect::from_corners((1, 1), (1, 1));
        assert!(rect.min == IVec2::new(1, 1));
        assert!(rect.max == IVec2::new(11, 11));
    }

    #[test]
    fn test_intersect() {
        let r1 = Rect::new([0, 0], [10, 10]);
        let r2 = Rect::new([5, 5], [10, 10]);
        let r3 = Rect::new([100, 100], [5, 5]);
        assert!(r1.intersects(r2));
        assert!(!r1.intersects(r3));
    }

    #[test]
    fn test_center() {
        let r1 = Rect::new([0, 0], [10, 10]);
        let center = r1.center();
        assert!(center.x == 5 && center.y == 5);
    }

    #[test]
    fn test_contains() {
        let r1 = Rect::new([0, 0], [10, 10]);
        assert!(r1.contains(IVec2::new(5, 5)));
        assert!(!r1.contains(IVec2::new(100, 100)));
    }

    #[test]
    fn test_rect_set() {
        let r1 = Rect::new([0, 0], [1, 1]);
        let points = r1.point_set();
        assert!(points.contains(&IVec2::new(0, 0)));
        assert!(!points.contains(&IVec2::new(1, 0)));
        assert!(!points.contains(&IVec2::new(0, 1)));
        assert!(!points.contains(&IVec2::new(1, 1)));
    }

    #[test]
    fn test_rect_callback() {
        use std::collections::HashSet;

        let r1 = Rect::new([0, 0], [1, 1]);
        let mut points: HashSet<IVec2> = HashSet::new();
        r1.for_each(|p| {
            points.insert(p);
        });
        assert!(points.contains(&IVec2::new(0, 0)));
        assert!(!points.contains(&IVec2::new(1, 0)));
        assert!(!points.contains(&IVec2::new(0, 1)));
        assert!(!points.contains(&IVec2::new(1, 1)));
    }
}
