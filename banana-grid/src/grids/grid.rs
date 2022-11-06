use crate::prelude::*;
use std::{
    ops::{Index, IndexMut, RangeBounds},
    slice,
};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

pub type PointIter = PointIterRowMajor;
pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridRows<'a, T> = slice::Chunks<'a, T>;
pub type GridRowsMut<'a, T> = slice::ChunksMut<'a, T>;

/// Compact bitwise implementation of a [Grid] of [bool]'s.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub size: UVec2,
    pub cells: Vec<T>,
}

impl<T: Copy + Clone> GridLike<T> for Grid<T> {
    ///////////////////////////////////////////////////////////////////////////
    // New Functionality
    ///////////////////////////////////////////////////////////////////////////
    #[inline(always)]
    fn new<S>(size: S, default_value: T) -> Self
    where
        S: Size2d,
        Self: Sized,
    {
        Self { size: size.as_uvec2(), cells: vec![default_value; size.count()] }
    }

    fn new_grid_map<G, U, F>(grid: G, f: F) -> Self
    where
        G: GridLike<U>,
        F: FnMut(&U) -> T,
        Self: Sized,
    {
        Self { cells: grid.data().iter().map(f).collect(), size: grid.size() }
    }

    #[inline(always)]
    fn new_fn<S, F>(size: S, mut f: F) -> Self
    where
        S: Size2d,
        F: FnMut(IVec2) -> T,
        Self: Sized,
    {
        let mut cells = Vec::with_capacity(size.count());
        for coord in size.iter() {
            cells.push(f(coord));
        }

        Self { size: size.as_uvec2(), cells }
    }

    #[inline(always)]
    fn new_clone<S>(size: S, value: T) -> Self
    where
        T: Clone,
        S: Size2d,
    {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, size: size.as_uvec2() }
    }

    #[inline(always)]
    fn new_copy<S>(size: S, value: T) -> Self
    where
        T: Copy,
        S: Size2d,
    {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells, size: size.as_uvec2() }
    }

    #[inline(always)]
    fn new_default<S>(size: S) -> Self
    where
        T: Default,
        S: Size2d,
    {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, T::default);
        Self { cells, size: size.as_uvec2() }
    }

    ///////////////////////////////////////////////////////////////////////////

    #[inline]
    fn data(&self) -> &[T] {
        &self.cells
    }

    #[inline]
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.cells.fill(value);
    }

    #[inline]
    fn width(&self) -> u32 {
        self.size.width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.size.height()
    }

    #[inline]
    fn size(&self) -> UVec2 {
        self.size
    }

    #[inline]
    fn len(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    fn get<I>(&self, index: I) -> Option<&T>
    where
        I: GridPoint,
    {
        self.try_idx(index).map(|idx| &self.cells[idx])
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where
        I: GridPoint,
    {
        self.try_idx(index).map(move |idx| &mut self.cells[idx])
    }

    fn get_checked<I>(&self, index: I) -> &T
    where
        I: GridPoint,
    {
        self.cells.index(self.get_idx(index))
    }

    fn get_mut_checked<I>(&mut self, index: I) -> &mut T
    where
        I: GridPoint,
    {
        self.cells.index_mut(self.get_idx(index))
    }

    fn count_neighbors<I>(&self, index: I, val: T) -> usize
    where
        I: GridPoint,
        T: std::cmp::PartialEq,
    {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && self[((index.x() + ix) as usize, (index.y() + iy) as usize)] == val
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }
}

///////////////////////////////////////////////////////////////////////////
// Iterator Functionality
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Grid<T> {
    pub fn map<U: Copy, F: FnMut(&T) -> U>(self, f: F) -> Grid<U> {
        Grid::new_grid_map(self, f)
    }

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> GridIter<T> {
        self.cells.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        self.cells.iter_mut()
    }

    pub fn point_iter(&self) -> PointIterRowMajor {
        self.size.iter()
    }

    #[inline]
    pub fn rows(&self) -> GridRows<T> {
        self.cells.chunks(self.size.width() as usize)
    }

    #[inline]
    pub fn rows_mut(&mut self) -> GridRowsMut<T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }

    #[inline]
    pub fn cols(&self) -> GridRows<T> {
        self.cells.chunks(self.size.width() as usize)
    }

    #[inline]
    pub fn cols_mut(&mut self) -> GridRowsMut<T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }

    /// Iterate over a range of rows.
    ///
    /// Yields &\[T\] (Slice of T)
    pub fn iter_rows(
        &self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &[T]> {
        let [start, end] = self.range_to_start_end(range, Axis::Y);
        let width = self.width() as usize;
        let count = end.saturating_sub(start) + 1;
        let chunks = self.cells[start * width..].chunks(width);
        chunks.take(count)
    }

    /// Iterate mutably over a range of rows.
    ///
    /// Yields &mut \[`T`\] (Slice of mutable `T`)
    pub fn iter_rows_mut(
        &mut self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &mut [T]> {
        let [start, end] = self.range_to_start_end(range, Axis::Y);
        let width = self.width() as usize;
        let count = end - start + 1;
        let chunks = self.cells[start * width..].chunks_mut(width);
        chunks.take(count)
    }

    /// An iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> {
        let w = self.width() as usize;
        return self.cells[x..].iter().step_by(w);
    }

    /// A mutable iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        let w = self.width() as usize;
        return self.cells[x..].iter_mut().step_by(w);
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.cells[index]
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl<T: Copy, P: GridPoint> Index<P> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &T {
        self.get_checked(index)
    }
}

impl<T: Copy, P: GridPoint> IndexMut<P> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        self.get_mut_checked(index)
    }
}
