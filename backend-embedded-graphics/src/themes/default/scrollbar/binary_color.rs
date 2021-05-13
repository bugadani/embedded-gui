use embedded_graphics::pixelcolor::BinaryColor;
use embedded_gui::widgets::slider::{Horizontal, Vertical};

use crate::themes::default::scrollbar::{ScrollbarVisualState, ScrollbarVisualStyle};

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
}
