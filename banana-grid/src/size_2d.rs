use crate::prelude::*;

/// A trait for types representing a 2d size.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub trait Size2d: Clone + Copy {
    /// Returns width coordinate.
    fn width(&self) -> u32;

    /// Returns height coordinate.
    fn height(&self) -> u32;

    #[inline]
    fn count(&self) -> usize {
        (self.width() * self.height()) as usize
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

    #[inline]
    fn intersects<S>(&self, other: S) -> bool
    where
        S: Size2d,
    {
        self.width() > 0 && self.height() > 0 && other.width() > 0 && other.height() > 0
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

    fn coord_iter_row_major(self) -> PointIterRowMajor {
        PointIterRowMajor::new(self)
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
