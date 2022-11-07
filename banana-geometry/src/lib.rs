#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod distance;
mod shapes;

pub mod prelude {
    pub use crate::distance::*;
    pub use crate::shapes::*;
    pub use crate::*;
}
