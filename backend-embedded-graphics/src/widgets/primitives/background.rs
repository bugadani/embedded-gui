use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{BinaryColor, PixelColor},
    prelude::Primitive,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    Drawable,
};
use embedded_gui::{
    data::WidgetData,
    widgets::{
        primitives::background::{Background, BackgroundProperties},
        Widget, WidgetWrapper,
    },
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

pub struct BackgroundStyle<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,
}

impl Default for BackgroundStyle<BinaryColor> {
    fn default() -> Self {
        Self {
            style: PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .build(),
        }
    }
}

impl<C> BackgroundProperties for BackgroundStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn background_color(&mut self, color: Self::Color) {
        self.style.fill_color = Some(color);
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Background
impl<W, C, DT, D> WidgetRenderer<EgCanvas<C, DT>>
    for WidgetWrapper<Background<W, BackgroundStyle<C>>, D>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    BackgroundStyle<C>: BackgroundProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.bounding_box()
            .to_rectangle()
            .into_styled(self.widget.background_properties.style)
            .draw(&mut canvas.target)?;

        self.widget.inner.draw(canvas)
    }
}
