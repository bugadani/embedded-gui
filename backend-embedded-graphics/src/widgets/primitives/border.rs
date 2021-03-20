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
        Widget, WidgetWrapper,
    },
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

pub struct BorderStyle<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,
}

impl Default for BorderStyle<BinaryColor> {
    fn default() -> Self {
        Self {
            style: PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build(),
        }
    }
}

impl<C> BorderProperties for BorderStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn get_border_width(&self) -> u32 {
        self.style.stroke_width
    }

    fn border_color(&mut self, color: Self::Color) {
        self.style.stroke_color = Some(color);
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Border
impl<W, C, DT, D> WidgetRenderer<EgCanvas<C, DT>> for WidgetWrapper<Border<W, BorderStyle<C>>, D>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    BorderStyle<C>: BorderProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.bounding_box()
            .to_rectangle()
            .into_styled(self.widget.border_properties.style)
            .draw(&mut canvas.target)?;

        self.widget.inner.draw(canvas)
    }
}
