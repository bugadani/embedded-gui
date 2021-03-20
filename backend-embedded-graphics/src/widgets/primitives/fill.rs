use embedded_graphics::{draw_target::DrawTarget, pixelcolor::PixelColor};
use embedded_gui::{
    widgets::{
        primitives::fill::{FillDirection, FillParent, HorizontalAlignment, VerticalAlignment},
        Widget,
    },
    WidgetRenderer,
};

use crate::EgCanvas;

impl<C, DT, W, FD, H, V> WidgetRenderer<EgCanvas<C, DT>> for FillParent<W, FD, H, V>
where
    FD: FillDirection,
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}
