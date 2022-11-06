use crate::prelude::*;
use std::mem;
use std::ops::Range;

#[macro_export]
macro_rules! make_direction_iter {
    ($col_name:ident, $iter_name:ident, $type:ident, $count:expr) => {
        #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone)]
        /// Iterate over all directions of the respectively-named type of direction
        pub struct $iter_name(Range<u8>);
        impl $iter_name {
            pub const fn new() -> Self {
                $iter_name(0..$count as u8)
            }
        }

        impl Iterator for $iter_name {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|n| unsafe { mem::transmute(n) })
            }
        }

        /// Represents a collection of the respectively-named type of direction
        #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy)]
        pub struct $col_name;
        impl IntoIterator for $col_name {
            type Item = $type;
            type IntoIter = $iter_name;
            fn into_iter(self) -> Self::IntoIter {
                $iter_name::new()
            }
        }
    };
}

// IntoIter implementations for iterating over all directions of a type. E.g.:
// for direction in CardinalDirections { ... }
make_direction_iter! {Directions, DirectionIter, Direction, NUM_DIRECTIONS}
make_direction_iter! {CardinalDirections, CardinalDirectionIter, CardinalDirection, NUM_CARDINAL_DIRECTIONS}
make_direction_iter! {OrdinalDirections, OrdinalDirectionIter, OrdinalDirection, NUM_ORDINAL_DIRECTIONS}

#[macro_export]
macro_rules! make_subdirection_iter {
    ($col_name:ident, $backing_col_name:ident, $iter_name:ident, $backing_iter_name:ident) => {
        #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone)]
        /// Iterator over a particular collection of `Direction`s
        pub struct $iter_name($backing_iter_name);

        impl $iter_name {
            pub const fn new() -> Self {
                Self($backing_iter_name::new())
            }
        }

        impl Iterator for $iter_name {
            type Item = Direction;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|d| d.direction())
            }
        }

        #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy)]
        /// Represents a particular collection of `Direction`s
        pub struct $col_name;
        impl IntoIterator for $col_name {
            type Item = Direction;
            type IntoIter = $iter_name;
            fn into_iter(self) -> Self::IntoIter {
                $iter_name($backing_col_name.into_iter())
            }
        }
    };
}

// IntoIter implementations for iterating over a subset of directions. E.g.:
// for direction in DirectionsCardinal { ... }
make_subdirection_iter! {
  DirectionsCardinal,
  CardinalDirections,
  DirectionCardinalIter,
  CardinalDirectionIter
}
make_subdirection_iter! {
  DirectionsOrdinal,
  OrdinalDirections,
  DirectionOrdinalIter,
  OrdinalDirectionIter
}
