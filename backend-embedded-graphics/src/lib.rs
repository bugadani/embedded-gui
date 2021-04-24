#![no_std]

pub mod themes;
pub mod widgets;

use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{Point, Size},
    primitives::Rectangle,
};
use embedded_gui::{
    geometry::{BoundingBox, MeasuredSize, Position},
    Canvas,
};

trait ToPoint {
    fn to_point(self) -> Point;
}

impl ToPoint for Position {
    fn to_point(self) -> Point {
        Point::new(self.x, self.y)
    }
}

trait ToSize {
    fn to_size(self) -> Size;
}

impl ToSize for MeasuredSize {
    fn to_size(self) -> Size {
        Size::new(self.width, self.height)
    }
}

trait ToRectangle {
    fn to_rectangle(self) -> Rectangle;
}

impl ToRectangle for BoundingBox {
    fn to_rectangle(self) -> Rectangle {
        let top_left = self.position.to_point();
        let size = self.size.to_size();

        Rectangle::new(top_left, size)
    }
}

pub struct EgCanvas<D>
where
    D: DrawTarget,
{
    pub target: D,
}

impl<D> EgCanvas<D>
where
    D: DrawTarget,
{
    pub fn new(target: D) -> Self {
        Self { target }
    }
}

impl<D: DrawTarget> Canvas for EgCanvas<D> {
    type Error = <D as DrawTarget>::Error;

    fn size(&self) -> MeasuredSize {
        let size = self.target.bounding_box().size;

        MeasuredSize {
            width: size.width,
            height: size.height,
        }
    }
}
