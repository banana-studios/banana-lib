use banana_grid::prelude::IVec2;

mod rect;
use rect::*;

pub mod prelude {
    pub use crate::rect::*;
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn GridShape>;
}

impl<T> ShapeClone for T
where
    T: 'static + GridShape + Clone,
{
    fn clone_box(&self) -> Box<dyn GridShape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GridShape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for Box<dyn GridShape> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridShade {{ pos {:?}, points {:?} }}",
            self.pos(),
            self.iter().collect::<Vec<IVec2>>()
        )
    }
}

impl PartialEq for Box<dyn GridShape> {
    fn eq(&self, other: &Self) -> bool {
        self.pos() == other.pos() && self.iter().eq(other.iter())
    }
}

pub trait GridShape: ShapeClone + Sync + Send + 'static {
    fn iter(&self) -> GridShapeIterator;
    fn pos(&self) -> IVec2;
    fn set_pos(&mut self, pos: IVec2);
}

#[derive(Debug, Clone)]
pub enum GridShapeIterator {
    Point(std::iter::Once<IVec2>),
    // Circle(GridCircleIter),
    // CircleOutline(GridCircleOutlineIter),
    Rect(RectIter),
    // Line(GridLineIter),
    // LineOrtho(GridLineOrthoIter),
    // Cone(GridConeIter),
}

impl Iterator for GridShapeIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            GridShapeIterator::Point(i) => i.next(),
            // GridShapeIterator::Circle(i) => i.next(),
            // GridShapeIterator::CircleOutline(i) => i.next(),
            GridShapeIterator::Rect(i) => i.next(),
            // GridShapeIterator::Line(i) => i.next(),
            // GridShapeIterator::LineOrtho(i) => i.next(),
            // GridShapeIterator::Cone(i) => i.next(),
        }
    }
}
