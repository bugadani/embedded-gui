use core::marker::PhantomData;

use embedded_graphics::prelude::WebColors;
use embedded_gui::widgets::slider::Horizontal;

use crate::themes::default::slider::{SliderVisualState, SliderVisualStyle};

pub struct SliderStyle<C>(pub PhantomData<C>);

impl<C> Default for SliderStyle<C>
where
    C: WebColors,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct SliderInactive<C>(pub PhantomData<C>);
impl<C> SliderVisualState<C> for SliderInactive<C>
where
    C: WebColors,
{
    const BACKGROUND_LINE_COLOR: Option<C> = Some(C::CSS_GRAY);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_SLATE_GRAY);
}

pub struct SliderIdle<C>(pub PhantomData<C>);
impl<C> SliderVisualState<C> for SliderIdle<C>
where
    C: WebColors,
{
    const BACKGROUND_LINE_COLOR: Option<C> = Some(C::CSS_GRAY);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_SLATE_GRAY);
}

pub struct SliderHovered<C>(pub PhantomData<C>);
impl<C> SliderVisualState<C> for SliderHovered<C>
where
    C: WebColors,
{
    const BACKGROUND_LINE_COLOR: Option<C> = Some(C::CSS_GRAY);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_LIGHT_SLATE_GRAY);
}

pub struct SliderDragged<C>(pub PhantomData<C>);
impl<C> SliderVisualState<C> for SliderDragged<C>
where
    C: WebColors,
{
    const BACKGROUND_LINE_COLOR: Option<C> = Some(C::CSS_GRAY);
    const BACKGROUND_LINE_THICKNESS: u32 = 1;
    const BORDER_COLOR: Option<C> = None;
    const BORDER_THICKNESS: u32 = 0;
    const FILL_COLOR: Option<C> = Some(C::CSS_STEEL_BLUE);
}

impl<C> SliderVisualStyle<C> for SliderStyle<C>
where
    C: WebColors,
{
    type Direction = Horizontal;

    const THICKNESS: u32 = 7;
    const WIDTH: u32 = 5;

    type Inactive = SliderInactive<C>;
    type Idle = SliderIdle<C>;
    type Hovered = SliderHovered<C>;
    type Dragged = SliderDragged<C>;
}
