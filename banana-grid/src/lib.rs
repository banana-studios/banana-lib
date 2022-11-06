mod axis;
mod grid_like;
mod grid_point;
mod grids;
mod size_2d;

pub mod prelude {
    pub use crate::axis::*;
    pub use crate::grid_like::*;
    pub use crate::grid_point::*;
    pub use crate::grids::grid::*;
    pub use crate::size_2d::*;
    pub use crate::{impl_grid_point_array, impl_grid_point_tuple};

    #[cfg(not(feature = "bvy"))]
    pub use glam::{IVec2, UVec2, Vec2};

    #[cfg(feature = "bvy")]
    pub use bevy::prelude::{IVec2, UVec2, Vec2};

    #[cfg(feature = "2d")]
    pub use crate::grids::grid_2d::*;
}
