use embedded_graphics::pixelcolor::BinaryColor;
use embedded_gui::widgets::slider::Horizontal;

use crate::themes::default::slider::{SliderVisualState, SliderVisualStyle};

#[derive(Default)]
pub struct SliderStyle;

pub struct SliderIdle;
impl SliderVisualState<BinaryColor> for SliderIdle {
    const BACKGROUND_LINE_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
    const BORDER_THICKNESS: u32 = 1;
    const FILL_COLOR: Option<BinaryColor> = Some(BinaryColor::Off);
}

pub struct SliderHovered;
impl SliderVisualState<BinaryColor> for SliderHovered {
    const BACKGROUND_LINE_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<BinaryColor> = Some(BinaryColor::Off);
    const BORDER_THICKNESS: u32 = 1;
    const FILL_COLOR: Option<BinaryColor> = Some(BinaryColor::On);
}

impl SliderVisualStyle<BinaryColor> for SliderStyle {
    type Direction = Horizontal;

    const THICKNESS: u32 = 7;
    const WIDTH: u32 = 5;

    type Inactive = SliderIdle;
    type Idle = SliderIdle;
    type Hovered = SliderHovered;
    type Dragged = SliderHovered;
}
