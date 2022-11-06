use banana_grid::prelude::GridPoint;

use super::Rect;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Neg
impl Neg for Rect {
    type Output = Rect;
    fn neg(self) -> Self::Output {
        Rect { min: -self.min, max: -self.max }
    }
}

// Add + Assign
impl Add<Rect> for Rect {
    type Output = Self;
    fn add(self, Rect { min, max }: Rect) -> Rect {
        Self { min: self.min + min, max: self.max + max }
    }
}

impl<P> Add<P> for Rect
where
    P: GridPoint,
{
    type Output = Self;
    fn add(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rect { min: self.min + rhs.x, max: self.max + rhs.x }
    }
}

impl<T> AddAssign<T> for Rect
where
    Rect: Add<T, Output = Rect>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

// Sub + Assign
impl Sub<Rect> for Rect {
    type Output = Self;
    fn sub(self, rhs: Rect) -> Self::Output {
        self + -rhs
    }
}

impl<P> Sub<P> for Rect
where
    P: GridPoint,
{
    type Output = Self;
    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rect { min: self.min - rhs.x, max: self.max - rhs.x }
    }
}

impl<T> SubAssign<T> for Rect
where
    Rect: Sub<T, Output = Rect>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

// Mul + MulAssign
impl Mul<Rect> for Rect {
    type Output = Rect;
    fn mul(self, rhs: Rect) -> Self::Output {
        Rect { min: self.min * rhs.min, max: self.max * rhs.max }
    }
}

impl<P> Mul<P> for Rect
where
    P: GridPoint,
{
    type Output = Rect;
    fn mul(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rect { min: self.min * rhs, max: self.max * rhs }
    }
}

impl<T> MulAssign<T> for Rect
where
    Rect: Mul<T, Output = Rect>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

// Div + DivAssign

impl Div<Rect> for Rect {
    type Output = Rect;
    fn div(self, rhs: Rect) -> Self::Output {
        Rect { min: self.min / rhs.min, max: self.max / rhs.max }
    }
}

impl<P> Div<P> for Rect
where
    P: GridPoint,
{
    type Output = Rect;
    fn div(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rect { min: self.min / rhs, max: self.max / rhs }
    }
}

impl<T> DivAssign<T> for Rect
where
    Rect: Div<T, Output = Rect>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
