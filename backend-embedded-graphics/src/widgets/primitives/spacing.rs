use embedded_graphics::{draw_target::DrawTarget, pixelcolor::PixelColor};
use embedded_gui::{
    widgets::{primitives::spacing::Spacing, Widget},
    WidgetRenderer,
};

use crate::EgCanvas;

impl<C, DT, I> WidgetRenderer<EgCanvas<C, DT>> for Spacing<I>
where
    I: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}
