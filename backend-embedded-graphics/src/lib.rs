#![no_std]

pub mod widgets;

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::PixelColor,
    prelude::{Point, Size},
    primitives::Rectangle,
};
use embedded_gui::{BoundingBox, Canvas, MeasuredSize};

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

pub struct EgCanvas<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub target: D,
}

impl<C, D> EgCanvas<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub fn new(target: D) -> Self {
        Self { target }
    }
}

impl<C: PixelColor, D: DrawTarget<Color = C>> Canvas for EgCanvas<C, D> {
    type Error = <D as DrawTarget>::Error;

    fn size(&self) -> MeasuredSize {
        let size = self.target.bounding_box().size;

        MeasuredSize {
            width: size.width,
            height: size.height,
        }
    }
}
