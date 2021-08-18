use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::PixelColor,
    prelude::Primitive,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    Drawable,
};
use embedded_gui::{
    widgets::{
        border::{Border, BorderProperties},
        Widget,
    },
    WidgetRenderer,
};

use crate::{themes::Theme, EgCanvas, ToRectangle};

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

impl<C> Default for BorderStyle<C>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            color: C::BORDER_COLOR,
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

    fn set_border_color(&mut self, color: Self::Color) {
        self.color = color;
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Border
impl<W, C, DT> WidgetRenderer<EgCanvas<DT>> for Border<W, BorderStyle<C>>
where
    W: Widget + WidgetRenderer<EgCanvas<DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    BorderStyle<C>: BorderProperties,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        let style = self.border_properties.build_style();

        self.bounding_box()
            .to_rectangle()
            .into_styled(style)
            .draw(&mut canvas.target)?;

        self.inner.draw(canvas)
    }
}
