use crate::prelude::*;
use ndarray::{
    iter::{AxisIter, AxisIterMut},
    *,
};
use std::ops::{Index, IndexMut};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid2D<T> {
    size: UVec2,
    data: ndarray::Array2<T>,
}

impl<T: Copy> GridLike<T> for Grid2D<T> {
    ///////////////////////////////////////////////////////////////////////////
    // New Functionality
    ///////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    fn new<S>(size: S, default_value: T) -> Self
    where
        S: Size2d,
        Self: Sized,
    {
        Self {
            size: size.as_uvec2(),
            data: ndarray::Array2::from_elem(size.as_uarray(), default_value),
        }
    }

    #[inline(always)]
    fn new_grid_map<G, U, F>(grid: G, f: F) -> Self
    where
        U: Clone,
        Self: Sized,
        G: GridLike<U>,
        F: FnMut(&U) -> T,
    {
        Self {
            size: grid.size(),
            data: Array2::from_shape_vec(
                Size2d::as_uarray(&grid.size()),
                grid.data().iter().map(f).collect(),
            )
            .unwrap(),
        }
    }

    #[inline(always)]
    fn new_fn<S, F>(size: S, mut f: F) -> Self
    where
        Self: Sized,
        S: Size2d,
        F: FnMut(IVec2) -> T,
    {
        Self {
            data: ndarray::Array2::from_shape_fn(size.as_uarray(), |a| {
                f(Size2d::as_ivec2(&a))
            }),
            size: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn new_default<S>(size: S) -> Self
    where
        T: Default,
        S: Size2d,
        Self: Sized,
    {
        Self {
            size: size.as_uvec2(),
            data: ndarray::Array2::from_elem(size.as_uarray(), T::default()),
        }
    }

    #[inline(always)]
    fn new_clone<S>(size: S, value: T) -> Self
    where
        T: Clone,
        S: Size2d,
    {
        Self {
            size: size.as_uvec2(),
            data: ndarray::Array2::from_elem(size.as_uarray(), value),
        }
    }

    #[inline(always)]
    fn new_copy<S>(size: S, value: T) -> Self
    where
        T: Copy,
        S: Size2d,
    {
        Self {
            size: size.as_uvec2(),
            data: ndarray::Array2::from_elem(size.as_uarray(), value),
        }
    }

    ///////////////////////////////////////////////////////////////////////////

    #[inline]
    fn data(&self) -> &[T] {
        self.data.as_slice().unwrap()
    }

    #[inline]
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value)
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
        let shape = self.data.shape();
        crate::grid_point::GridPoint::as_uvec2(&(shape[0], shape[1]))
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    ///////////////////////////////////////////////////////////////////////////
    ///  Getters
    ///////////////////////////////////////////////////////////////////////////

    fn get<I>(&self, index: I) -> Option<&T>
    where
        I: GridPoint,
    {
        self.data.get(index.as_uarray())
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where
        I: GridPoint,
    {
        self.data.get_mut(index.as_uarray())
    }

    fn get_checked<I>(&self, index: I) -> &T
    where
        I: GridPoint,
    {
        self.data.index(index.as_uarray())
    }

    fn get_mut_checked<I>(&mut self, index: I) -> &mut T
    where
        I: GridPoint,
    {
        self.data.index_mut(index.as_uarray())
    }

    ///////////////////////////////////////////////////////////////////////////
    /// Iters
    ///////////////////////////////////////////////////////////////////////////

    fn count_neighbors<I>(&self, index: I, val: T) -> usize
    where
        I: GridPoint,
        T: std::cmp::PartialEq,
    {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                let x = (index.x() + ix) as usize;
                let y = (index.y() + iy) as usize;
                if !(ix == 0 && iy == 0) && self.data[[x, y]] == val {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }
}

///////////////////////////////////////////////////////////////////////////
// New Functions
///////////////////////////////////////////////////////////////////////////

impl<T> Grid2D<T> {
    pub fn new_from_vec<Sh>(shape: Sh, data: Vec<T>) -> anyhow::Result<Grid2D<T>, ShapeError>
    where
        T: Clone,
        Sh: Size2d + ShapeBuilder<Dim = Ix2>,
    {
        ndarray::Array2::from_shape_vec(shape, data)
            .map(|data| Self { data, size: shape.as_uvec2() })
    }

    pub fn new_grid_map_ref<U, F>(grid: &Grid2D<U>, f: F) -> Self
    where
        F: for<'a> FnMut(&U) -> T,
    {
        Self { data: grid.data.map(f), size: grid.size }
    }
}

///////////////////////////////////////////////////////////////////////////
// Index Functionality
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Grid2D<T> {
    pub fn rows(&self) -> AxisIter<T, Ix1> {
        self.data.axis_iter(Axis(0))
    }

    pub fn rows_mut(&mut self) -> AxisIterMut<T, Ix1> {
        self.data.axis_iter_mut(Axis(0))
    }

    pub fn cols(&self) -> AxisIter<T, Ix1> {
        self.data.axis_iter(Axis(1))
    }

    pub fn cols_mut(&mut self) -> AxisIterMut<T, Ix1> {
        self.data.axis_iter_mut(Axis(1))
    }

    pub fn raw(&self) -> &ndarray::Array2<T> {
        &self.data
    }

    pub fn raw_mut(&mut self) -> &mut ndarray::Array2<T> {
        &mut self.data
    }
}

///////////////////////////////////////////////////////////////////////////
// Iterator Functionality
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Grid2D<T> {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn map<U, F>(self, f: F) -> Self
    where
        U: Clone,
        T: std::marker::Copy,
        F: for<'a> FnMut(&'a T) -> T,
    {
        Self::new_grid_map(self, f)
        // Self
    }

    pub fn map_ref<U, Sh, F>(&self, f: F) -> Grid2D<U>
    where
        U: Clone,
        F: FnMut(&T) -> U,
        Sh: Size2d + ShapeBuilder<Dim = Ix2>,
    {
        Grid2D::new_grid_map_ref(self, f)
    }

    pub fn map_mut<B, F>(&mut self, f: F) -> Self
    where
        T: Clone,
        F: Fn(T) -> T,
    {
        Self { data: self.data.mapv(f), size: self.size }
    }

    pub fn map_inplace<F>(&mut self, f: F)
    where
        T: Clone,
        F: Fn(T) -> T,
    {
        self.data.mapv_inplace(f)
    }

    pub fn map_inplace_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        self.data.map_inplace(f)
    }

    pub fn slice<I>(&self, start: I, end: I) -> ArrayView<T, Ix2>
    where
        I: GridPoint,
    {
        let start1 = start.x() as i32;
        let end1 = start.y() as i32;
        let start2 = end.x() as i32;
        let end2 = end.y() as i32;
        self.data.slice(s![start1..end1, start2..end2])
    }

    pub fn row<X: TryInto<i32>>(&self, x: X) -> ArrayView<T, Ix1> {
        self.data.row(x.try_into().ok().expect("Failed to convert x to row_i32") as usize)
    }

    pub fn column<X: TryInto<i32>>(&self, y: X) -> ArrayView<T, Ix1> {
        self.data
            .column(y.try_into().ok().expect("Failed to convert y to column_i32") as usize)
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Index<usize> for Grid2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        let x = index % self.size.x as usize;
        let y = index / self.size.x as usize;
        &self.data[(x, y)]
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for Grid2D<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let x = index % self.size.x as usize;
        let y = index / self.size.x as usize;
        &mut self.data[(x, y)]
    }
}

impl<T: Copy, P: GridPoint> Index<P> for Grid2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &Self::Output {
        &self.data[index.as_uarray()]
    }
}

impl<T: Copy, P: GridPoint> IndexMut<P> for Grid2D<T> {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        &mut self.data[index.as_uarray()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice() {
        let grid: Grid2D<i32> = Grid2D::new_fn((10, 10), |(x, y)| (x + y) as i32);
        let slice = grid.slice::<(i32, i32)>((1, -1), (1, -1));

        assert_eq!(slice.shape(), [8, 8]);
        assert_eq!(slice.first(), Some(&2));
        assert_eq!(slice.last(), Some(&16));
    }
}
