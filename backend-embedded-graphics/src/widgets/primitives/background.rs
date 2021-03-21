use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{BinaryColor, PixelColor},
    prelude::Primitive,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    Drawable,
};
use embedded_gui::{
    widgets::{
        primitives::background::{Background, BackgroundProperties},
        Widget, WidgetDataHolderTrait, WidgetWrapper,
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
    fn build_style(&self) -> PrimitiveStyle<C> {
        PrimitiveStyleBuilder::new().fill_color(self.color).build()
    }
}

impl Default for BackgroundStyle<BinaryColor> {
    fn default() -> Self {
        Self {
            color: BinaryColor::On,
        }
    }
}

impl<C> BackgroundProperties for BackgroundStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn background_color(&mut self, color: Self::Color) {
        self.color = color;
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Background
impl<W, C, DT, DH> WidgetRenderer<EgCanvas<C, DT>>
    for WidgetWrapper<Background<W, BackgroundStyle<C>>, DH>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    DH: WidgetDataHolderTrait<Owner = Background<W, BackgroundStyle<C>>>,
    BackgroundStyle<C>: BackgroundProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        let style = self.widget.background_properties.build_style();

        self.bounding_box()
            .to_rectangle()
            .into_styled(style)
            .draw(&mut canvas.target)?;

        self.widget.inner.draw(canvas)
    }
}
