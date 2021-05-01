use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use embedded_gui::widgets::slider::{Horizontal, Vertical};

use crate::themes::default::scrollbar::ScrollbarVisualStyle;

#[derive(Default)]
pub struct VerticalScrollbar;

impl ScrollbarVisualStyle<BinaryColor> for VerticalScrollbar {
    type Direction = Vertical;

    const THICKNESS: u32 = 6;

    fn draw<DT: DrawTarget<Color = BinaryColor>>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
    ) -> Result<(), DT::Error> {
        todo!()
    }
}

#[derive(Default)]
pub struct HorizontalScrollbar;

impl ScrollbarVisualStyle<BinaryColor> for HorizontalScrollbar {
    type Direction = Horizontal;

    const THICKNESS: u32 = 6;

    fn draw<DT: DrawTarget<Color = BinaryColor>>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
    ) -> Result<(), DT::Error> {
        todo!()
    }
}
