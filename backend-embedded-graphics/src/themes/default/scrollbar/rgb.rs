use core::marker::PhantomData;

use embedded_graphics::{draw_target::DrawTarget, prelude::WebColors};
use embedded_gui::widgets::slider::{Horizontal, SliderFields, Vertical};

use crate::themes::default::scrollbar::{
    ScrollbarProperties, ScrollbarVisualState, ScrollbarVisualStyle,
};

pub struct VerticalScrollbar<C>(pub PhantomData<C>);

impl<C> Default for VerticalScrollbar<C>
where
    C: WebColors,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct ScrollbarInactive<C>(pub PhantomData<C>);
impl<C> ScrollbarVisualState<C> for ScrollbarInactive<C>
where
    C: WebColors,
{
    const BACKGROUND_FILL_COLOR: Option<C> = None;
    const BACKGROUND_BORDER_COLOR: Option<C> = None;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_SLATE_GRAY);
}

pub struct ScrollbarIdle<C>(pub PhantomData<C>);
impl<C> ScrollbarVisualState<C> for ScrollbarIdle<C>
where
    C: WebColors,
{
    const BACKGROUND_FILL_COLOR: Option<C> = None;
    const BACKGROUND_BORDER_COLOR: Option<C> = None;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_SLATE_GRAY);
}

pub struct ScrollbarHovered<C>(pub PhantomData<C>);
impl<C> ScrollbarVisualState<C> for ScrollbarHovered<C>
where
    C: WebColors,
{
    const BACKGROUND_FILL_COLOR: Option<C> = None;
    const BACKGROUND_BORDER_COLOR: Option<C> = None;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_LIGHT_SLATE_GRAY);
}

pub struct ScrollbarDragged<C>(pub PhantomData<C>);
impl<C> ScrollbarVisualState<C> for ScrollbarDragged<C>
where
    C: WebColors,
{
    const BACKGROUND_FILL_COLOR: Option<C> = None;
    const BACKGROUND_BORDER_COLOR: Option<C> = None;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_STEEL_BLUE);
}

impl<C> ScrollbarVisualStyle<C> for VerticalScrollbar<C>
where
    C: WebColors,
{
    type Direction = Vertical;

    const THICKNESS: u32 = 6;

    type Inactive = ScrollbarInactive<C>;
    type Idle = ScrollbarIdle<C>;
    type Hovered = ScrollbarHovered<C>;
    type Dragged = ScrollbarDragged<C>;
}

pub struct HorizontalScrollbar<C>(pub PhantomData<C>);

impl<C> Default for HorizontalScrollbar<C>
where
    C: WebColors,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C> ScrollbarVisualStyle<C> for HorizontalScrollbar<C>
where
    C: WebColors,
{
    type Direction = Horizontal;

    const THICKNESS: u32 = 6;

    type Inactive = ScrollbarInactive<C>;
    type Idle = ScrollbarIdle<C>;
    type Hovered = ScrollbarHovered<C>;
    type Dragged = ScrollbarDragged<C>;
}
