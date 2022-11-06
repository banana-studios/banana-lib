use crate::prelude::*;
use std::ops::{Bound, RangeBounds};

/// The [Grid] trait abstracts over containers of [Clone] and [Copy] items laid out in a rectangle
/// with a certain [width](Self::width) and [height](Self::height).
pub trait GridLike<T> {
    fn new<S>(size: S, new_value: T) -> Self
    where
        S: Size2d,
        Self: Sized;

    fn new_default<S>(size: S) -> Self
    where
        T: Default,
        S: Size2d,
        Self: Sized;

    fn new_fn<S, F>(size: S, f: F) -> Self
    where
        S: Size2d,
        F: FnMut(IVec2) -> T,
        Self: Sized;

    fn fill(&mut self, value: T)
    where
        T: Clone;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn size(&self) -> UVec2;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    /// Tests whether a point is in bounds.
    fn in_bounds<I>(&self, index: I) -> bool
    where
        I: GridPoint,
    {
        let pos = index.as_ivec2();
        pos.cmpge(IVec2::ZERO).all() && pos.cmplt(self.size().as_ivec2()).all()
    }

    /// Gets the index corresponding to a coordinate, which is row-wise.
    fn get_idx<I>(&self, index: I) -> usize
    where
        I: GridPoint,
    {
        index.as_index(self.width() as usize)
    }

    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn try_idx<I>(&self, index: I) -> Option<usize>
    where
        I: GridPoint,
    {
        if self.in_bounds(index) {
            Some(self.get_idx(index))
        } else {
            None
        }
    }

    /// Gets the `GridPoint` corresponding to an index
    fn index_to_pt(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn try_index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let w: usize = self.width().try_into().expect("width is too large");
        let x = idx % w;
        let y = idx / w;
        if self.in_bounds((x, y)) {
            Some(GridPoint::as_ivec2(&(x, y)))
        } else {
            None
        }
    }

    /// Convert a range into a [start,end] pair.
    ///
    /// An unbounded "end" returned by this function should be treated as EXCLUSIVE.
    fn range_to_start_end(&self, range: impl RangeBounds<usize>, axis: Axis) -> [usize; 2] {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
            Bound::Unbounded => axis.size(self.size()) as usize,
        };

        [start, end]
    }

    ///////////////////////////////////////////////////////////////////////////
    /// GridPoint Getters
    ///////////////////////////////////////////////////////////////////////////

    // No bounds Checking
    fn get<I>(&self, index: I) -> Option<&T>
    where
        I: GridPoint;
    fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where
        I: GridPoint;

    // Bounds Checking
    fn get_unchecked<I>(&self, index: I) -> &T
    where
        I: GridPoint;
    fn get_mut_unchecked<I>(&mut self, index: I) -> &mut T
    where
        I: GridPoint;

    ///////////////////////////////////////////////////////////////////////////
    /// GridPoint Iters
    ///////////////////////////////////////////////////////////////////////////
    fn count_neighbors<I>(&self, index: I, val: T) -> usize
    where
        I: GridPoint,
        T: std::cmp::PartialEq;
}
