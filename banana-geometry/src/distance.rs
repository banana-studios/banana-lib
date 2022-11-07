use banana_grid::prelude::GridPoint;
use std::ops::Sub;

/// Enumeration of available 2D Distance algorithms
#[allow(clippy::module_name_repetitions)]
pub enum DistanceAlg {
    /// Use the Pythagoras algorithm for determining distance - sqrt(A^2 + B^2)
    Pythagoras,
    /// Us the Pythagoras algorithm for distance, but omitting the square-root for a faster but squared result.
    PythagorasSquared,
    /// Use Manhattan distance (distance up plus distance along)
    Manhattan,
    /// Use Chebyshev distance (like Manhattan, but adds one to each entry)
    Chebyshev,
    /// Use a diagonal distance, the max of the x and y distances
    Diagonal,
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.

    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d<P>(self, start: P, end: P) -> f32
    where
        P: GridPoint,
    {
        match self {
            DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance2d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance2d_chebyshev(start, end),
            DistanceAlg::Diagonal => distance2d_diagonal(start, end),
        }
    }
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras<P>(start: P, end: P) -> f32
where
    P: GridPoint,
{
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared<P>(start: P, end: P) -> f32
where
    P: GridPoint,
{
    let start = start.as_vec2();
    let end = end.as_vec2();

    let dist = (start.max(end) - start.min(end)).powf(2.0);
    dist.x + dist.y
}

/// Calculates a Manhattan distance between two points
fn distance2d_manhattan<P>(start: P, end: P) -> f32
where
    P: GridPoint,
{
    let start = start.as_vec2();
    let end = end.as_vec2();
    let dist = start.max(end) - start.min(end);
    dist.x + dist.y
}

/// Calculates a Chebyshev distance between two points
/// See: [GameProgramming/Heuristics](http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html)
fn distance2d_chebyshev<P>(start: P, end: P) -> f32
where
    P: GridPoint,
{
    let start = start.as_vec2();
    let end = end.as_vec2();
    let dist = start.max(end) - start.min(end);
    if dist.x > dist.y {
        (dist.x - dist.y) + 1.0 * dist.y
    } else {
        (dist.y - dist.x) + 1.0 * dist.x
    }
}

// Calculates a diagonal distance
fn distance2d_diagonal<P>(start: P, end: P) -> f32
where
    P: GridPoint,
{
    let start = start.as_vec2();
    let end = end.as_vec2();

    start.sub(end).abs().max_element()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_pythagoras_distance() {
        let mut d = DistanceAlg::Pythagoras.distance2d((0, 0), (5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d((0, 0), (-5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d((0, 0), (0, 5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d((0, 0), (0, -5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d((0, 0), (5, 5));
        assert!(f32::abs(d - 7.071_068) < f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_squared_distance() {
        let mut d = DistanceAlg::PythagorasSquared.distance2d((0, 0), (5, 0));
        assert!(f32::abs(d - 25.0) < f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d((0, 0), (-5, 0));
        assert!(f32::abs(d - 25.0) < f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d((0, 0), (0, 5));
        assert!(f32::abs(d - 25.0) < f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d((0, 0), (0, -5));
        assert!(f32::abs(d - 25.0) < f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d((0, 0), (5, 5));
        assert!(f32::abs(d - 50.0) < f32::EPSILON);
    }

    #[test]
    fn test_manhattan_distance() {
        let mut d = DistanceAlg::Manhattan.distance2d((0, 0), (5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d((0, 0), (-5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d((0, 0), (0, 5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d((0, 0), (0, -5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d((0, 0), (5, 5));
        assert!(f32::abs(d - 10.0) < f32::EPSILON);
    }

    #[test]
    fn test_chebyshev_distance() {
        let mut d = DistanceAlg::Chebyshev.distance2d((0, 0), (5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d((0, 0), (-5, 0));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d((0, 0), (0, 5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d((0, 0), (0, -5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d((0, 0), (5, 5));
        assert!(f32::abs(d - 5.0) < f32::EPSILON);
    }
}
