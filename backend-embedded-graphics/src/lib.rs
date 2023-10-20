//! backend-embedded-graphics
//! =========================
//!
//! `backend-embedded-graphics` is a platform specific backend crate for [`embedded-gui`]. It enables
//! using `embedded-gui` on platforms that support [`embedded-graphics`].
//!
//! [`embedded-gui`]: https://github.com/bugadani/embedded-gui/backend-embedded-graphics
//! [`embedded-graphics`]: https://github.com/embedded-graphics/embedded-graphics
//!
//! Backend features
//! ----------------
//!
//! The crate implements all generic widget primitives defined in `embedded-gui`. On top of that,
//! this backend contains custom constructors and styling options for the following widgets:
//!
//!  - [`Label`]
//!  - [`TextBlock`]
//!  - [`TextBox`]
//!
//! Custom constructors allow you to specify through the imported constructor the character set used
//! in these widgets. For example, a `Label` constructed by importing the
//! [widgets::label::iso_8859_2::LabelConstructor] trait will support the `iso_8859_2` character set.
//!
//! Themes
//! ------
//!
//! This crate provides collections of commonly used widget compositions, called themes. Rendering
//! and visual style of graphical widgets are also implemented by themes.
//!
//!  - [`DefaultTheme`]: supports both `BinaryColor`, `Rgb555`, `Rgb565`, `Rgb888` draw targets.
//!
//! [`Label`]: widgets::label::Label
//! [`TextBlock`]: widgets::text_block::TextBlock
//! [`TextBox`]: widgets::text_box::TextBox
//! [`DefaultTheme`]: themes::default::DefaultTheme
//!

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
