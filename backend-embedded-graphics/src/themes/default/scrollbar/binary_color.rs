use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Primitive,
    primitives::PrimitiveStyle, Drawable,
};
use embedded_gui::widgets::slider::{Horizontal, Slider, SliderFields, Vertical};

use crate::{
    themes::default::scrollbar::{ScrollbarProperties, ScrollbarVisualState, ScrollbarVisualStyle},
    ToRectangle,
};

#[derive(Default)]
pub struct VerticalScrollbar;

pub struct ScrollbarIdle;
impl ScrollbarVisualState<BinaryColor> for ScrollbarIdle {
    const BACKGROUND_FILL_COLOR: Option<BinaryColor> = None;
    const BACKGROUND_BORDER_COLOR: Option<BinaryColor> = None;
    const BORDER_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
    const BORDER_THICKNESS: u32 = 1;
    const FILL_COLOR: Option<BinaryColor> = Some(BinaryColor::Off);
}

pub struct ScrollbarHovered;
impl ScrollbarVisualState<BinaryColor> for ScrollbarHovered {
    const BACKGROUND_FILL_COLOR: Option<BinaryColor> = None;
    const BACKGROUND_BORDER_COLOR: Option<BinaryColor> = None;
    const BORDER_COLOR: Option<BinaryColor> = Some(BinaryColor::Off);
    const BORDER_THICKNESS: u32 = 1;
    const FILL_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
}

impl ScrollbarVisualStyle<BinaryColor> for VerticalScrollbar {
    type Direction = Vertical;

    const THICKNESS: u32 = 6;

    type Inactive = ScrollbarIdle;
    type Idle = ScrollbarIdle;
    type Hovered = ScrollbarHovered;
    type Dragged = ScrollbarHovered;

    fn draw<DT: DrawTarget<Color = BinaryColor>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<BinaryColor, Self>, D>,
    ) -> Result<(), DT::Error> {
        self.draw_vertical(canvas, slider)
    }
}

#[derive(Default)]
pub struct HorizontalScrollbar;

impl ScrollbarVisualStyle<BinaryColor> for HorizontalScrollbar {
    type Direction = Horizontal;

    const THICKNESS: u32 = 6;

    type Inactive = ScrollbarIdle;
    type Idle = ScrollbarIdle;
    type Hovered = ScrollbarHovered;
    type Dragged = ScrollbarHovered;

    fn draw<DT: DrawTarget<Color = BinaryColor>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<BinaryColor, Self>, D>,
    ) -> Result<(), DT::Error> {
        self.draw_horizontal(canvas, slider)
    }
}
