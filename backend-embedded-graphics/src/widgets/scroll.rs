use embedded_graphics::draw_target::{Clipped, DrawTarget, DrawTargetExt};
use embedded_gui::{
    data::WidgetData,
    widgets::{scroll::Scroll, Widget},
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

impl<W, SD, D, DT> WidgetRenderer<EgCanvas<DT>> for Scroll<W, SD, D>
where
    W: Widget + for<'a> WidgetRenderer<EgCanvas<Clipped<'a, DT>>>,
    D: WidgetData,
    DT: DrawTarget,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        let bounds = self.fields.bounds.to_rectangle();
        let mut clipped = EgCanvas::new(canvas.target.clipped(&bounds));

        self.fields.inner.draw(&mut clipped)
    }
}
