#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

//! banana-lib is a wrapper of the banana- set of crates designed initally
//! for roguelike development

/// prelude
pub mod prelude {
    pub use banana_direction::prelude::*;
    pub use banana_geometry::prelude::*;
    pub use banana_grid::prelude::*;
    pub use banana_utils::*;
}

pub mod geometry {
    pub use banana_geometry::prelude::*;
}

pub mod grid {
    pub use banana_grid::prelude::*;
}

pub mod direction {
    pub use banana_direction::prelude::*;
}

pub mod utils {
    pub use banana_utils::*;
}
