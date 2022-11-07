use super::Rect;
use banana_grid::prelude::{IVec2, Size2d};

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct RectPointIter {
    curr: IVec2,
    size: IVec2,

    /// The minimum corner point of the rect.
    pub min: IVec2,
    /// The maximum corner point of the rect.
    pub max: IVec2,
}

impl RectPointIter {
    pub fn new(min: impl Size2d, max: impl Size2d) -> Self {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        let size = max - min;
        Self { min, max, size, curr: IVec2::ZERO }
    }
}

impl Iterator for RectPointIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.cmpge(self.max).any() {
            return None;
        }

        let p = self.curr;
        self.curr.x += 1;
        if self.curr.x == self.size.x {
            self.curr.x = 0;
            self.curr.y += 1;
        }
        Some(self.min + p)
    }
}

impl IntoIterator for Rect {
    type Item = IVec2;
    type IntoIter = RectPointIter;

    fn into_iter(self) -> Self::IntoIter {
        RectPointIter::new(self.min, self.max)
    }
}

impl From<Rect> for RectPointIter {
    fn from(rect: Rect) -> Self {
        rect.into_iter()
    }
}
