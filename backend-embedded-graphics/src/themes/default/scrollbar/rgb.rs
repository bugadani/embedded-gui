use core::marker::PhantomData;

use embedded_graphics::prelude::WebColors;
use embedded_gui::widgets::slider::Vertical;

use crate::themes::default::scrollbar::ScrollbarVisualStyle;

pub struct VerticalScrollbar<C>(pub PhantomData<C>);

impl<C> Default for VerticalScrollbar<C>
where
    C: WebColors,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C> ScrollbarVisualStyle<C> for VerticalScrollbar<C>
where
    C: WebColors,
{
    type Direction = Vertical;

    const THICKNESS: u32 = 6;

    fn draw<DT: embedded_graphics::draw_target::DrawTarget<Color = C>>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
    ) -> Result<(), DT::Error> {
        todo!()
    }
}
