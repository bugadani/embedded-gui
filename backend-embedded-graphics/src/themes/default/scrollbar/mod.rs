use core::{marker::PhantomData, ops::RangeInclusive};

use embedded_graphics::{draw_target::DrawTarget, prelude::PixelColor};
use embedded_gui::{
    data::WidgetData,
    geometry::{BoundingBox, MeasuredSize},
    widgets::slider::{
        Horizontal, Slider, SliderDirection, SliderFields, SliderProperties, Vertical,
    },
    WidgetRenderer,
};

use crate::{themes::default::DefaultTheme, EgCanvas};

pub mod binary_color;
pub mod rgb;

pub trait ScrollbarVisualStyle<C>: Default
where
    C: PixelColor,
{
    type Direction: SliderDirection;

    const THICKNESS: u32;

    fn draw<DT: DrawTarget<Color = C>>(
        &self,
        canvas: &mut EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<C, Self>>,
    ) -> Result<(), DT::Error>;
}

pub struct ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    visual: VS,
    window_length: u32,
    _marker: PhantomData<C>,
}

impl<C, VS> ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    pub fn new() -> Self {
        Self {
            visual: VS::default(),
            window_length: 0,
            _marker: PhantomData,
        }
    }

    pub fn set_length(&mut self, length: u32) {
        self.window_length = length;
    }
}

impl<C, VS> SliderProperties for ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    type Direction = VS::Direction;
    const THICKNESS: u32 = VS::THICKNESS;

    fn length(&self) -> u32 {
        self.window_length
    }
}

impl<VS, C, DT, D> WidgetRenderer<EgCanvas<DT>> for Slider<ScrollbarProperties<C, VS>, D>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    VS: ScrollbarVisualStyle<C>,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.visual.draw(canvas, &self.fields)
    }
}

pub type StyledVerticalScrollbar<C> =
    Slider<ScrollbarProperties<C, <C as DefaultTheme>::VerticalScrollbar>>;

pub fn vertical_scrollbar<C: DefaultTheme>() -> StyledVerticalScrollbar<C> {
    Slider::new(0..=30, ScrollbarProperties::new())
}
