use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Primitive,
    primitives::PrimitiveStyleBuilder, Drawable,
};
use embedded_gui::widgets::slider::{Horizontal, Slider, SliderFields, Vertical};

use crate::{
    themes::default::scrollbar::{ScrollbarProperties, ScrollbarVisualStyle},
    ToRectangle,
};

#[derive(Default)]
pub struct VerticalScrollbar;

impl ScrollbarVisualStyle<BinaryColor> for VerticalScrollbar {
    type Direction = Vertical;

    const THICKNESS: u32 = 6;

    fn draw<DT: DrawTarget<Color = BinaryColor>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<BinaryColor, Self>, D>,
    ) -> Result<(), DT::Error> {
        // TODO: for the default theme, this may be extracted as the default implementation

        // TODO: add visual states and color constants
        let bg_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .build();

        let fg_style = if slider.state.has_state(Slider::STATE_HOVERED) {
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::Off)
                .fill_color(BinaryColor::On)
                .stroke_width(1)
                .build()
        } else {
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .fill_color(BinaryColor::Off)
                .stroke_width(1)
                .build()
        };

        // Background
        slider
            .bounds
            .to_rectangle()
            .into_styled(bg_style)
            .draw(&mut canvas.target)?;

        // Foreground
        slider
            .slider_bounds()
            .to_rectangle()
            .into_styled(fg_style)
            .draw(&mut canvas.target)
    }
}

#[derive(Default)]
pub struct HorizontalScrollbar;

impl ScrollbarVisualStyle<BinaryColor> for HorizontalScrollbar {
    type Direction = Horizontal;

    const THICKNESS: u32 = 6;

    fn draw<DT: DrawTarget<Color = BinaryColor>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<BinaryColor, Self>, D>,
    ) -> Result<(), DT::Error> {
        todo!()
    }
}
