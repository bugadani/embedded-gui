use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{BinaryColor, PixelColor},
    prelude::Primitive,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    Drawable,
};
use embedded_gui::{
    data::WidgetData,
    widgets::{
        primitives::border::{Border, BorderProperties},
        Container, Widget,
    },
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

pub struct BorderStyle<C>
where
    C: PixelColor,
{
    color: C,
    width: u32,
}

impl<C> BorderStyle<C>
where
    C: PixelColor,
{
    fn build_style(&self) -> PrimitiveStyle<C> {
        PrimitiveStyleBuilder::new()
            .stroke_alignment(StrokeAlignment::Inside)
            .stroke_color(self.color)
            .stroke_width(self.width)
            .build()
    }
}

impl Default for BorderStyle<BinaryColor> {
    fn default() -> Self {
        Self {
            color: BinaryColor::On,
            width: 1,
        }
    }
}

impl<C> BorderProperties for BorderStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn get_border_width(&self) -> u32 {
        self.width
    }

    fn border_color(&mut self, color: Self::Color) {
        self.color = color;
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Border
impl<W, C, DT, D> WidgetRenderer<EgCanvas<C, DT>> for Container<Border<W, BorderStyle<C>>, D>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    BorderStyle<C>: BorderProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        let style = self.widget.border_properties.build_style();

        self.bounding_box()
            .to_rectangle()
            .into_styled(style)
            .draw(&mut canvas.target)?;

        self.widget.inner.draw(canvas)
    }
}
