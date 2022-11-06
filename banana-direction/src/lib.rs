mod bitmap;
mod cardinal;
mod direction;
mod iter;
mod ordinal;
mod table;

pub mod prelude {
    pub use crate::bitmap::*;
    pub use crate::cardinal::*;
    pub use crate::direction::*;
    pub use crate::iter::*;
    pub use crate::ordinal::*;
    pub use crate::table::*;

    #[cfg(feature = "rng")]
    pub use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn iteration() {
        {
            use CardinalDirection::*;
            assert_eq!(
                CardinalDirections.into_iter().collect::<Vec<_>>(),
                vec![North, East, South, West]
            )
        }

        {
            use OrdinalDirection::*;
            assert_eq!(
                OrdinalDirections.into_iter().collect::<Vec<_>>(),
                vec![NorthEast, SouthEast, SouthWest, NorthWest]
            )
        }

        {
            use Direction::*;
            assert_eq!(
                Directions.into_iter().collect::<Vec<_>>(),
                vec![North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,]
            )
        }
    }

    #[test]
    fn table_iteration() {
        {
            let table = CardinalDirectionTable::new_fn(|d| d);
            assert!(table.enumerate().all(|(a, &b)| a == b));
        }
        {
            let table = OrdinalDirectionTable::new_fn(|d| d);
            assert!(table.enumerate().all(|(a, &b)| a == b));
        }
        {
            let table = DirectionTable::new_fn(|d| d);
            assert!(table.enumerate().all(|(a, &b)| a == b));
        }
    }
}
