#![no_std]

pub mod themes;
pub mod widgets;

use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{Point, Size},
    primitives::Rectangle,
};
use embedded_gui::{
    geometry::{BoundingBox, MeasuredSize},
    Canvas,
};

trait ToRectangle {
    fn to_rectangle(self) -> Rectangle;
}

impl ToRectangle for BoundingBox {
    fn to_rectangle(self) -> Rectangle {
        let top_left = Point::new(self.position.x, self.position.y);
        let size = Size::new(self.size.width, self.size.height);

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
