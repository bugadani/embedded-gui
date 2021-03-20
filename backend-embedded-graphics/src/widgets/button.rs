use embedded_graphics::{draw_target::DrawTarget, pixelcolor::PixelColor};
use embedded_gui::{
    data::WidgetData,
    widgets::{button::Button, Widget},
    WidgetRenderer,
};

use crate::EgCanvas;

impl<C, DT, W, D> WidgetRenderer<EgCanvas<C, DT>> for Button<W, D>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}
