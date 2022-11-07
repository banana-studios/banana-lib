use crate::prelude::*;

////////////////////////////////////////////////////////////
// Point Iter
////////////////////////////////////////////////////////////

pub struct PointIterRowMajor {
    coord: IVec2,
    size: UVec2,
}

impl PointIterRowMajor {
    pub fn new(size: impl Size2d) -> Self {
        Self { size: size.as_uvec2(), coord: IVec2::new(0, 0) }
    }
}

impl Iterator for PointIterRowMajor {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.height() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;

        if self.coord.x == self.size.width() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }

        Some(coord)
    }
}

////////////////////////////////////////////////////////////

pub const MAX_SIZE_FIELD: u32 = ::core::i32::MAX as u32;
pub const MAX_SIZE: UVec2 = UVec2 { x: MAX_SIZE_FIELD, y: MAX_SIZE_FIELD };

#[derive(Debug)]
pub struct DimensionTooLargeForSize;

pub(crate) const fn check_size_limit(value: u32) -> Result<(), DimensionTooLargeForSize> {
    if value >= MAX_SIZE_FIELD {
        Err(DimensionTooLargeForSize)
    } else {
        Ok(())
    }
}

/// A trait for types representing a 2d size.
pub trait Size2d: Clone + Copy {
    fn try_new(width: u32, height: u32) -> Result<UVec2, DimensionTooLargeForSize> {
        check_size_limit(width)?;
        check_size_limit(height)?;
        Ok(UVec2 { x: width, y: height })
    }

    /// Creates a new `UVec2`.
    /// Panics if `width` or `width` is greater than `::core::i32::MAX as u32`
    #[allow(clippy::new_ret_no_self)]
    fn new(width: u32, height: u32) -> UVec2 {
        match Self::try_new(width, height) {
            Err(DimensionTooLargeForSize) => {
                panic!("Size is too big: ({}, {}). Max is {}.", width, width, MAX_SIZE_FIELD);
            }
            Ok(size) => size,
        }
    }

    /// Returns width coordinate.
    fn width(&self) -> u32;

    /// Returns height coordinate.
    fn height(&self) -> u32;

    #[inline]
    fn count(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    #[inline]
    fn point_in_bounds<P>(&self, point: P) -> bool
    where
        P: GridPoint,
    {
        point.x() >= 0
            && point.y() >= 0
            && point.x() < self.width() as i32
            && point.y() < self.height() as i32
    }

    /// Returns an iterator over all points in the grid.
    fn iter(self) -> PointIterRowMajor {
        PointIterRowMajor::new(self)
    }

    #[inline]
    fn with_axis<F: FnMut(u32) -> u32>(self, axis: Axis, mut f: F) -> UVec2 {
        match axis {
            Axis::X => UVec2 { x: f(self.width()), ..self.as_uvec2() },
            Axis::Y => UVec2 { y: f(self.height()), ..self.as_uvec2() },
        }
    }

    #[inline]
    fn try_new_axis(
        this_axis: u32,
        other_axis: u32,
        axis: Axis,
    ) -> Result<UVec2, DimensionTooLargeForSize> {
        axis.try_new_size(this_axis, other_axis)
    }

    #[inline]
    fn new_axis(this_axis: u32, other_axis: u32, axis: Axis) -> UVec2 {
        axis.new_size(this_axis, other_axis)
    }

    /// Convert dimensions to IVec2 (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.width() as i32, self.height() as i32)
    }

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 {
        self.as_ivec2().as_uvec2()
    }

    /// Convert dimensions to Vec2 (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    /// Convert dimensions to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }

    /// Convert dimensions to tuple format
    #[inline]
    fn as_tuple(&self) -> (i32, i32) {
        (self.width() as i32, self.height() as i32)
    }

    /// Convert dimensions to `[usize; 2]`.
    #[inline]
    fn as_uarray(&self) -> [usize; 2] {
        [self.width() as usize, self.height() as usize]
    }
}

#[macro_export]
macro_rules! impl_size2d_array {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 {
                self[0] as u32
            }

            fn height(&self) -> u32 {
                self[1] as u32
            }
        }
    };
}

#[macro_export]
macro_rules! impl_size2d_tuple {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 {
                self.0 as u32
            }

            fn height(&self) -> u32 {
                self.1 as u32
            }
        }
    };
}

impl_size2d_array!(IVec2);
impl_size2d_array!(UVec2);
impl_size2d_array!([u32; 2]);
impl_size2d_array!([i32; 2]);
impl_size2d_array!([usize; 2]);

impl_size2d_tuple!((u32, u32));
impl_size2d_tuple!((i32, i32));
impl_size2d_tuple!((usize, usize));
