use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::PixelColor,
    prelude::Primitive,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    Drawable,
};
use embedded_gui::{
    widgets::{
        background::{Background, BackgroundProperties},
        Widget,
    },
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

pub struct BackgroundStyle<C>
where
    C: PixelColor,
{
    color: C,
}

impl<C> BackgroundStyle<C>
where
    C: PixelColor,
{
    pub fn new(color: C) -> Self {
        Self { color }
    }

    fn build_style(&self) -> PrimitiveStyle<C> {
        PrimitiveStyleBuilder::new().fill_color(self.color).build()
    }
}

impl<C> BackgroundProperties for BackgroundStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn set_background_color(&mut self, color: Self::Color) {
        self.color = color;
    }
}

impl<W, C, DT> WidgetRenderer<EgCanvas<DT>> for Background<W, BackgroundStyle<C>>
where
    W: Widget + WidgetRenderer<EgCanvas<DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    BackgroundStyle<C>: BackgroundProperties,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        let style = self.background_properties.build_style();

        self.bounding_box()
            .to_rectangle()
            .into_styled(style)
            .draw(&mut canvas.target)?;

        self.inner.draw(canvas)
    }
}
